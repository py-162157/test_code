type TreeNode<T> = Option<Box<Node<T>>>;

struct Node<T: std::fmt::Display> {
    left: TreeNode<T>,
    right: TreeNode<T>,
    value: Option<T>,
}

trait BinaryTree<T> {
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
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
}

impl<T: std::fmt::Display + PartialOrd> Node<T> {
    fn init() -> Node<T> {
        Node {
            left: None,
            right: None,
            value: None,
        }
    }

    /*fn new(key: T) -> Node<T> {
        Node {
            left: None,
            right: None,
            value: Some(key),
        }
    }*/
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
    tree.in_order();
    //println!("test successful!");
}