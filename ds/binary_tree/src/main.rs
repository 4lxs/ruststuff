use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value)
                } else {
                    self.left = Some(Box::new(Node::new(value)))
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value)
                } else {
                    self.right = Some(Box::new(Node::new(value)))
                }
            }
            Ordering::Equal => {}
        }
    }
}

impl Node<i32> {
    fn zero(&mut self) {
        let mut stack = vec![self];

        while let Some(Node { value, left, right }) = stack.pop() {
            *value = 0;

            if let Some(lchild) = left {
                stack.push(lchild)
            }

            if let Some(rchild) = right {
                stack.push(rchild)
            }
        }
    }

    fn sum(&mut self) -> i32 {
        let mut sum = 0;
        let mut stack = vec![self];

        while let Some(Node { value, left, right }) = stack.pop() {
            sum += *value;

            if let Some(lchild) = left {
                stack.push(lchild)
            }

            if let Some(rchild) = right {
                stack.push(rchild)
            }
        }

        sum
    }
}

fn main() {
    let mut n1 = Node::new(40);
    println!("{n1:?}");
    n1.insert(4);
    println!("{n1:?}");
    n1.insert(42);
    println!("{n1:?}");
    println!("sum={}", n1.sum());
    n1.zero();
    println!("{n1:?}");
    println!("sum={}", n1.sum());
}
