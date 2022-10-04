use core::fmt;
use std::{marker::PhantomData, ptr::NonNull};

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}

pub struct Linkedlist<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _maker: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct IndexOutOfRangeError;

impl fmt::Display for IndexOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index out of range")
    }
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
    pub fn into_val(self: Box<Self>) -> T {
        self.val
    }
}

/* linked
 -----         ------       ------
|head| -->    | next | -->  | next| ---> None
|    |        |      |      |     |
|    |        |      | <--  | pre |
|tail|        |pre:N |      |     |
------        ------         ------
  |                           |
  -----------------------------
*/

impl<T> Linkedlist<T> {
    pub fn new() -> Self {
        Linkedlist {
            length: 0,
            head: None,
            tail: None,
            _maker: PhantomData,
        }
    }
    fn push_front(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.next = self.head;
        node.prev = None;
        let node = NonNull::new(Box::into_raw(node));
        if let Some(head) = self.head {
            unsafe {
                (*head.as_ptr()).prev = node;
            }
        } else {
            // empty node
            self.tail = node;
        }
        self.head = node;
        self.length += 1;
    }

    fn push_back(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.prev = self.tail;
        node.next = None;
        let node = NonNull::new(Box::into_raw(node));
        if let Some(tail) = self.tail {
            unsafe {
                (*tail.as_ptr()).next = node;
            }
        } else {
            // empty node
            self.head = node;
        }
        self.tail = node;
        self.length += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head| unsafe {
            let node = Box::from_raw(head.as_ptr());
            self.head = node.next;
            if let Some(head) = self.head {
                //* 移除头节点的下一个节点，prev 指针需要指向 none */
                (*head.as_ptr()).prev = None;
            } else {
                /* 如果没有下一个节点，tail 指针指向none */
                self.tail = None;
            }
            self.length -= 1;
            node.into_val()
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.head.map(|head| unsafe {
            let node = Box::from_raw(head.as_ptr());
            self.tail = node.prev;
            if let Some(head) = self.tail {
                (*head.as_ptr()).next = None;
            } else {
                self.head = None;
            }
            self.length -= 1;
            node.into_val()
        })
    }

    fn peek_front(&self) -> Option<&T> {
        // as_ref --> unsafe{&*self.head.as_ptr()}
        unsafe { self.head.as_ref().map(|head| &head.as_ref().val) }
    }
    fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|tail| &tail.as_ref().val) }
    }

    fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|head| &mut head.as_mut().val) }
    }
    fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|tail| &mut tail.as_mut().val) }
    }
}

impl<T> Default for Linkedlist<T> {
    fn default() -> Self {
        Self::new()
    }
}
