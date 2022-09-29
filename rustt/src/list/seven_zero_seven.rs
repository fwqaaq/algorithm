/*
 * @lc app=leetcode.cn id=707 lang=rust
 *
 * [707] 设计链表
 */

// @lc code=start

#[derive(Debug)]
pub struct MyLinkedList {
    pub val: i32,
    pub next: Option<Box<MyLinkedList>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { val: 0, next: None }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut i = 0;
        let mut cur = &self.next;
        while let Some(node) = cur {
            if i == index {
                return node.val;
            }
            i += 1;
            cur = &node.next;
        }
        -1
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList {
            val,
            next: self.next.take(),
        });
        self.next = Some(new_node);
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList { val, next: None });
        let mut last_node = &mut self.next;
        while let Some(node) = last_node {
            last_node = &mut node.next;
        }
        *last_node = Some(new_node);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
        } else {
            let mut i = 0;
            let mut cur = &mut self.next;
            while let Some(node) = cur {
                if i + 1 == index {
                    let new_node = Box::new(MyLinkedList {
                        val,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    break;
                }
                i += 1;
                cur = &mut node.next;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }

        let mut i = 0;
        let mut cur = self;
        while let Some(node) = cur.next.take() {
            if i == index {
                cur.next = node.next;
                break;
            }
            i += 1;
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_linked_list() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.add_at_tail(3);
        //println!("{:?}", linked_list);
        linked_list.add_at_index(1, 2);
        //println!("{:?}", linked_list);
        assert_eq!(linked_list.get(1), 2);
        linked_list.delete_at_index(1);
        //println!("{:?}", linked_list);
        assert_eq!(linked_list.get(1), 3);
    }
}
