type TreeNode<T> = Option<Box<Node<T>>>;

struct Stack<T: std::fmt::Display> {//基于数组的栈
    data: Vec<T>,
    top: usize,
}

struct Queue<T> {
    data: Vec<T>,
    len: usize,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            data: Vec::new(),
            len: 0,
        }
    }

    fn EnQueue(&mut self, key: T) {
        self.data.push(key);
        self.len += 1;
    }

    fn DeQueue(&mut self) -> Option<T> {
        if self.len <= 0 {
            println!("The queue is empty!");
            None
        } else {
            let temp = self.data.remove(0);
            self.len -= 1;
            Some(temp)
        }
    }
}

impl<T: std::fmt::Display + std::clone::Clone> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            data: Vec::new(),
            top: 0,
        }
    }

    fn push(&mut self, key: T) {
        self.data.push(key);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top <= 0 {
            println!("The stack is empty!");
            None
        } else {
            self.top -= 1;
            self.data.pop()
        }
    }

    fn PrintPop(&mut self) {
        if self.top <= 0 {
            println!("The stack is empty!");
        } else {
            self.top -= 1;
            println!("{}", self.data.pop().unwrap())
        }
    }

    fn IsEmpty(self) {
        if self.top == 0 {
            println!("The stack is empty!");
        } else {
            println!("The stack is not empty!");
        }
    }

    fn GetTop(&self) -> Option<T> {
        if self.top == 0 {
            println!("The stack is empty!");
            None
        } else {
            Some(self.data[self.top].clone())
        }
    }

    fn PopAndPrintAll(&mut self) {
        while self.top != 0 {
            self.PrintPop();
        }
    }
}

struct Node<T: std::fmt::Display> {
    left: TreeNode<T>,
    right: TreeNode<T>,
    value: Option<T>,
}

trait BinaryTree<T> {
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
    fn level_order(self);
}

trait BinarySearchTree<T: std::fmt::Display> : BinaryTree<T> {
    fn insert(&mut self, key: T);
}

impl<T: std::fmt::Display> BinaryTree<T> for Node<T> {
    fn pre_order(&self) {
        if let Some(ref value) = self.value {
            println!("{}", value);
            if let Some(ref left) = self.left {
                left.pre_order();
            }

            if let Some(ref right) = self.right {
                right.pre_order();
            }
        }
    }

    fn in_order(&self) {
        if let Some(ref value) = self.value {
            if let Some(ref left) = self.left {
                left.in_order();
            }

            println!("{}", value);
            if let Some(ref right) = self.right {
                right.in_order();
            }
        }
    }

    fn post_order(&self) {
        if let Some(ref value) = self.value {
            if let Some(ref left) = self.left {
                left.post_order();
            }

            if let Some(ref right) = self.right {
                right.post_order();
            }

            println!("{}", value);
        }
    }

    fn level_order(self) {
        let mut Q = Queue::<Node<T>>::new();
        if let Some(ref value) = self.value {//此处&表示借用
            Q.EnQueue(self);
            while Q.len != 0 {
                if let Some(p) = Q.DeQueue() {
                    println!("{}", p.value.unwrap());//此处可能panic
                    if let Some(left) = p.left {
                        Q.EnQueue(*left);
                    }
                    if let Some(right) = p.right {
                        Q.EnQueue(*right);
                    }
                } else {
                    println!("Error happen when dequeue!");
                }
            }
        } else {
            println!("The bitree is empty!");
        }
    }
}

impl<T: std::fmt::Display + PartialOrd> Node<T> {
    fn init() -> Node<T> {
        Node {
            left: None,
            right: None,
            value: None,
        }
    }
}

impl<T: PartialOrd + std::fmt::Display> BinarySearchTree<T> for Node<T> {
    fn insert(&mut self, key: T) {
        if let Some(temp) = self.value.as_ref() {
            if key <= *temp {
                if let Some(ref mut left) = self.left {
                    left.insert(key);//8月31日记录，left不是node类型  //9月1日已解决
                } else {
                    self.left = Some(Box::new(Node {
                        left: None,
                        right: None,
                        value: Some(key),
                    }))
                }
            } else {
                if let Some(ref mut right) = self.right {
                    right.insert(key);
                } else {
                    self.right = Some(Box::new(Node {
                        left: None,
                        right: None,
                        value: Some(key),
                    }))
                }
            }
        } else {
            self.value = Some(key);
        }
    }
}

fn main() {
    let mut tree = Node::<i32>::init();
    let index = vec![5, 6, 4, 1, 3, -2, 18];
    for i in index {
        tree.insert(i);
    }
    tree.level_order();
}

#[cfg(test)]
mod tests {
    use crate::{Node, BinaryTree, BinarySearchTree};
    use crate::{Stack, Queue};

    #[test]
    fn BinarySearchTree_Check() {
        let mut tree = Node::<i32>::init();
        let index = vec![5, 6, 4, 1, 3, -2, 18];
        for i in index {
            tree.insert(i);
        }
        assert_eq!(tree.value, Some(5));
        if let Some(ref left) = tree.left {
            assert_eq!(left.value, Some(4));
        }
        if let Some(ref right) = tree.right {
            assert_eq!(right.value, Some(6));
        }
    }

    #[test]
    fn Stack_Check() {
        let mut S = Stack::new();
        S.push(1); S.push(2); S.push(3);
        assert_eq!(S.pop(), Some(3));
        assert_eq!(S.pop(), Some(2));
        assert_eq!(S.pop(), Some(1));
    }

    #[test]
    fn Queue_Check() {
        let mut Q = Queue::new();
        Q.EnQueue(1); Q.EnQueue(2); Q.EnQueue(3);
        assert_eq!(Q.DeQueue(), Some(1));
        assert_eq!(Q.DeQueue(), Some(2));
        assert_eq!(Q.DeQueue(), Some(3));
    }
}