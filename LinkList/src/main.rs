use std::ptr;

pub struct List<T> {
    len: usize,
    head: Node<T>,//error：head为Node<T>类型，不具备take()函数
    tail: *mut Node<T>,//使用小范围的unsafe实现
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut now_node = self.head.next.take();
        while let Some(mut boxed_node) = now_node {
            now_node = boxed_node.next.take();
        }
    }
}

impl<T: std::fmt::Display> Node<T> {
    fn new(elem:T) -> Self {
        Node {
            value: elem,
            next: None,
        }
    }

    fn take(self) -> Option<Box<Node<T>>> {
        Some(Box::new(self))
    }
    
    fn set_next(&mut self, elem: T) {
        self.next = Some(Box::new(Node::new(elem)));
    }

    fn insert_in_next(&mut self, elem: T) {
        if let Some(ref next_node) = self.next {
            let new_node = Node {
                value: elem,
                next: self.next.take(),
            };

            self.next = Some(Box::new(new_node));
        } else {
            self.set_next(elem);
        }
    }

    fn get_last(&mut self) -> &mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }

    fn print_all_element(&mut self) {
        if let Some(ref mut x) = self.next {
            println!("{} ", x.value);
            x.print_all_element();
        } else {
            //println!("{} ", self.value);
        }
    }
}

impl<T: std::fmt::Display + std::clone::Clone + std::cmp::PartialEq> List<T> {
    fn new(info: T) ->Self {
        List {
            len: 0,
            head: Node {
                value: info,
                next: None,
            },
            tail: ptr::null_mut(),
        }
    }

    //self.head.next: Option<Box<Node<T>>>
    fn head_insert(&mut self, elem: T) {
        let new_node = Node {
            value: elem,
            next: self.head.next.take(),
        };

        self.head.next = Some(Box::new(new_node));
        self.len += 1;
    }

    fn rear_insert(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            value: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;
        
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head.next = Some(new_tail);
        }

        self.tail = raw_tail;
        self.len += 1;
    }

    fn insert_by_key(&mut self, key: T, elem: T) {//在指定元素key之后插入元素elem
         if let mut ptr = self.head.next.as_mut().unwrap() {
            let mut count: usize = 1;
            loop {
                if ptr.value == key {
                    ptr.insert_in_next(elem);
                    self.len += 1;
                    break;
                } else { 
                    count += 1;
                    if count > self.len {
                        println!("Can't find the element {} ", key);
                        break;
                    }
                    ptr = ptr.next.as_mut().unwrap();
                }
            }
         } else {
             println!("The List is empty!");
         }  
    }

    fn print_all_element(&mut self) {
        if self.len == 0 {
            println!("The linklist is empty!");
        } else {
            self.head.print_all_element();
        }
        println!("total length is {}", self.len);
    }
}

fn main() {
     let mut L = List::new(0);
     //test
     L.rear_insert(1);
     L.head_insert(2);
     L.insert_by_key(3, 6);
     L.print_all_element();
}