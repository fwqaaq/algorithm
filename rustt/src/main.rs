use std::cmp::Reverse;
use std::{collections::BinaryHeap, slice::from_raw_parts, str::from_utf8_unchecked};
fn get_memory_location() -> (usize, usize) {
    let string = "hello world";
    let pointer = string.as_ptr();
    let len = string.len();
    (pointer as usize, len)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn main() {
    // let (pointer, length) = get_memory_location();
    // let message = get_str_at_location(pointer, length);
    // println!(
    //     "The {} bytes at 0x{:X} stored: {}",
    //     length, pointer, message
    // );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
    // Converting a vector into a hashset.
    // let v = vec![1, 2, 3, 4, 5, 3, 5];
    // let vikings = v.into_iter().collect::<HashSet<_>>();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(2));
    heap.push(Reverse(6));
    heap.push(Reverse(1));
    heap.push(Reverse(3));

    println!("{:?}", heap.peek().unwrap());
    println!("{:?}", Reverse(3) < Reverse(2));
}
