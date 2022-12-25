use std::fmt;
use std::option::Option;

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64;
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct LinkedListIter<T> {
    cur: Option<Box<Node<T>>>,
}

pub struct RefLinkedListIter<'a, T> {
    cur: &'a Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Self>>) -> Self {
        Node { value, next }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cur: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::from("#");
        while let Some(node) = cur {
            result = format!("{} -> {}", result, node.value);
            cur = &node.next;
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    // where Option<Box<Node<T>>>: Clone {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            size: self.size,
        }
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut cur: &Option<Box<Node<f64>>> = &self.head;
        let mut result = 0.;
        while let Some(node) = cur {
            result += node.value.powi(2);
            cur = &node.next;
        }
        result.sqrt()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    // where Option<Box<Node<T>>>: Clone {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.head == other.head
    }
}

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node: Box<Node<T>> = self.cur.take()?;
        self.cur = node.next;
        Some(node.value)
    }
}

impl<T: Clone> Iterator for RefLinkedListIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.cur {
            self.cur = &node.next;
            Some(node.value.clone())
        } else {
            None
        }
    }
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {
            cur: self.head.clone(),
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = RefLinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefLinkedListIter { cur: &self.head }
    }
}
