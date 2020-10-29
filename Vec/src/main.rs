#![feature(ptr_internals, alloc)]

use std::ptr::{Unique, self}; 
use std::ops::{Deref, DerefMut};
use std::mem;
use std::marker::PhantomData;
use std::alloc::{GlobalAlloc, System, Layout, alloc, realloc, dealloc};
use std::num::NonZeroUsize;

pub struct MyVec<T> {
    buf: RawMyVec<T>, 
    len: usize,
}

struct RawMyVec<T> {
    ptr: Unique<T>,
    cap: usize,
}

struct RawIter<T> {
    start: *const T,
    end: *const T,
}

struct IntoIter<T> {
    _buf: RawMyVec<T>,
    iter: RawIter<T>,
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut MyVec<T>>,
    iter: RawIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut self.iter {}
    }
}

impl<T> RawIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RawIter {
            start: slice.as_ptr(),
            end: if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().offset(slice.len() as isize)
            }
        }
    }
}

impl<T> RawMyVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs!");
        RawMyVec {ptr: Unique::dangling(), cap: 0}
    }

    fn grow(&mut self) {//2为底的指数增长
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let layout = Layout::new::<T>();
            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;

                assert!(old_num_bytes <= (isize::MAX as usize) / 2, "capacity overflow");

                let new_num_bytes = old_num_bytes *2; 
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_num_bytes);
                (new_cap, ptr)  
            };
        
            if ptr.is_null() { 
                std::alloc::handle_alloc_error(layout); 
            }

        self.ptr = Unique::new(ptr as *mut _).unwrap();
        self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawMyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::new::<T>();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

impl<T> Iterator for RawIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = if mem::size_of::<T>() == 0 {
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                };
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len = (self.end as usize - self.start as usize) / if elem_size == 0 {1} else {elem_size};
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = if mem::size_of::<T>() == 0 {
                    (self.end as usize - 1) as *mut T
                } else {
                    self.end.offset(-1)
                };
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> { self.iter.next_back() }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> MyVec<T> {
    fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }
    fn cap(&self) -> usize { self.buf.cap }

    pub fn new() -> Self {
        MyVec { buf: RawMyVec::new(), len: 0 }
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawIter::new(&self);
            self.len = 0;
            Drain {
                iter: iter,
                vec: PhantomData,
            }
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        unsafe{
            let buf = ptr::read(&self.buf);
            let iter = RawIter::new(&self);
            mem::forget(self);//确保不调用self的析构函数
            IntoIter {
                iter: iter,
                _buf: buf,
            }
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().offset(self.len as isize), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr().offset(self.len as isize)))
            }
        }
    }

    pub fn insert(&mut self, position: usize, elem: T) {
        if self.len >= position {
            println!("position specified is out of bounds!");
        } else {
            if self.cap() == self.len { self.buf.grow() }
            unsafe {
                ptr::copy(self.ptr().offset(position as isize),
                          self.ptr().offset(position as isize + 1),
                          self.len - position);
                ptr::write(self.ptr().offset(position as isize), elem);
                self.len += 1;
            }
        }
    }

    pub fn remove(&mut self, position: usize) {
        if self.len <= position {
            println!("position specified is out of bounds!");
        } else {
            unsafe {
                self.len -= 1;
                ptr::copy(self.ptr().offset(position as isize + 1),
                          self.ptr().offset(position as isize),
                          self.len - position);
            }
        }
    }
}

fn main() {
    //test
    let mut x = MyVec::new();
    x.push(1);
    x.push(2);
    x.push(3);
    x.remove(1);
    let iter = x.into_iter();
    for i in iter {
        println!("{}", i);
    }
}
