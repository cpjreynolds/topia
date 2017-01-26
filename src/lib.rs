use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;
use std::collections::vec_deque;

#[derive(Debug, Clone, Hash)]
pub struct Node<T> {
    value: T,
    children: VecDeque<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            children: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn push_back(&mut self, value: T) {
        self.children.push_back(Node::new(value));
    }

    pub fn push_front(&mut self, value: T) {
        self.children.push_front(Node::new(value));
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.children.insert(index, Node::new(value));
    }

    pub fn pop_front(&mut self) -> Option<Node<T>> {
        self.children.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<Node<T>> {
        self.children.pop_back()
    }

    pub fn remove(&mut self, index: usize) -> Option<Node<T>> {
        self.children.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }

    pub fn front(&self) -> Option<&Node<T>> {
        self.children.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut Node<T>> {
        self.children.front_mut()
    }

    pub fn back(&self) -> Option<&Node<T>> {
        self.children.back()
    }

    pub fn back_mut(&mut self) -> Option<&mut Node<T>> {
        self.children.back_mut()
    }

    pub fn iter(&self) -> vec_deque::Iter<Node<T>> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> vec_deque::IterMut<Node<T>> {
        self.children.iter_mut()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<'a, T> IntoIterator for &'a Node<T> {
    type Item = &'a Node<T>;
    type IntoIter = vec_deque::Iter<'a, Node<T>>;

    fn into_iter(self) -> vec_deque::Iter<'a, Node<T>> {
        self.children.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Node<T> {
    type Item = &'a mut Node<T>;
    type IntoIter = vec_deque::IterMut<'a, Node<T>>;

    fn into_iter(self) -> vec_deque::IterMut<'a, Node<T>> {
        self.children.iter_mut()
    }
}
