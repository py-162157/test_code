use std::sync::atomic::{AtomicUsize, Ordering};
use std::default::Default;
use core::clone::Clone;
use std::sync::Arc;

struct CacheLine {
    inner: AtomicUsize,
}

struct RingBuf<T> {
    head: CacheLine,
    tail: CacheLine,
    container: Vec::<T>,
    cap: usize,
}

struct Producer<T> {
    inner: Arc<RingBuf<T>>
}

struct Consumer<T> {
    inner: Arc<RingBuf<T>>
}

impl CacheLine {
    fn new(x: usize) -> Self {
        CacheLine {
            inner: AtomicUsize::new(x)
        }
    }
}

impl<T: Clone + Default> RingBuf<T> {
    fn new() -> Self {
        RingBuf {
            head: CacheLine::new(0),
            tail: CacheLine::new(0),
            container: Vec::<T>::new(),
            cap: 0
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        assert!(capacity <= usize::MAX && capacity >= usize::MIN);

        let mut buffer = Vec::<T>::new();
        for _ in 0..capacity {
            buffer.push(T::default());
        }
        RingBuf {
            head: CacheLine::new(0),
            tail: CacheLine::new(0),
            container: buffer,
            cap: capacity,
        }
    }

    fn try_push(&mut self, data: T) -> Option<T> {
        let cur_head = self.head.inner.load(Ordering::Relaxed);
        let mut head_next = cur_head + 1;
        
        if head_next >= self.cap {
            head_next = 0;
        }
        if head_next == self.tail.inner.load(Ordering::Acquire) {
            println!("error: the ringbuffer is full!");
            Some(data)
        } else {
            self.container[cur_head] = data;
            self.head.inner.store(head_next, Ordering::Release);
            None
        }
    }

    fn try_pop(&mut self) -> Option<T> {
        let cur_tail = self.tail.inner.load(Ordering::Relaxed);
        let mut tail_next = cur_tail + 1;

        if tail_next >= self.cap {
            tail_next = 0;
        }
        if cur_tail == self.head.inner.load(Ordering::Acquire) {
            println!("error: the ringbuffer is empty!");
            None
        } else {
            self.tail.inner.store(tail_next, Ordering::Release);
            Some(self.container[cur_tail].clone())
        }
    }
}


#[test]
fn push_pop_test() {
    let mut buffer = RingBuf::<usize>::with_capacity(10);
    for i in 0..10 {
        let result = buffer.try_push(i);
    }
    for i in 0..10 {
        let result = buffer.try_pop();
    }
}

#[test]
fn full_empty__check() {
    let mut buffer = RingBuf::<usize>::with_capacity(1);
    buffer.try_push(1);
    assert_eq!(buffer.try_push(1), Some(1));

    buffer.try_pop();
    assert_eq!(buffer.try_pop(), None);
}
