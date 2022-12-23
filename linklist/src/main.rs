use std::fmt;

// #![allow(dead_code)]

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, next: Option<Box<Self>>) -> Self {
        Node { value, next }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    pub fn push_front(&mut self, value: i32) {
        self.head = Some(Box::new(Node::new(value, self.head.take())));
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

// impl Drop for LinkedList {
//     fn drop(&mut self) {
//         let mut cur = self.head.take();
//         while let Some(mut node) = cur {
//             cur = node.next.take();
//         }
//     }
// }

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("#");
        let mut cur = &self.head;
        while let Some(node) = cur {
            ret = format!("{} -> {}", ret, node.value);
            cur = &node.next;
        }
        write!(f, "{}", ret)
    }
}

fn main() {
    println!("test");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut list = LinkedList::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.empty(), true);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(1);
        list.push_front(4);
        
        assert_eq!(list.size(), 3);
        assert_eq!(list.empty(), false);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.size(), 0);
        assert_eq!(list.empty(), true);
    }
}