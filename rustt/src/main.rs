// fn main() {
//     let text = String::from("xxx xx x ....");
//     let mut wo = Word::new(&text);

//     println!("{}", wo.next_word().unwrap());
// }

// struct Word<'a> {
//     position: usize,
//     string: &'a str,
// }

// impl Word<'_> {
//     fn new(string: &str) -> Word {
//         Word {
//             position: 0,
//             string,
//         }
//     }

//     fn next_word(&mut self) -> Option<&str> {
//         let start_of_world = &self.string[self.position..];
//         let index_of_next_scope = start_of_world.find(' ').unwrap_or(start_of_world.len());
//         if !start_of_world.is_empty() {
//             self.position += index_of_next_scope + 1;
//             Some(&start_of_world[..index_of_next_scope])
//         } else {
//             None
//         }
//     }
// }

use std::{
    cell::Cell,
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // let t1 = thread::spawn(f);
    // let t2 = thread::spawn(f);

    // let numbers = vec![1, 2, 3];

    // let numbers = Vec::from_iter(0..=100);

    // let t = thread::spawn(move || {
    //     // for n in &numbers {
    //     //     println!("{n}");
    //     // }
    //     numbers.iter().sum::<usize>() / numbers.len()
    // })
    // .join()
    // .unwrap();

    // thread::scope(|s| {
    //     s.spawn(|| {
    //         println!("length: {}", numbers.len());
    //     });

    //     s.spawn(|| {
    //         for n in &numbers {
    //             println!("{n}");
    //         }
    //     });
    // });

    // let x: &'static [i32; 3  ] = Box::leak(Box::new([1, 2, 3]));

    // thread::spawn(move || dbg!(x));
    // thread::spawn(move || dbg!(x));

    // let a = Arc::new([1, 2, 3]);
    // thread::spawn({
    //     let a = a.clone();
    //     move || dbg!(a)
    // });
    // dbg!(a);
    // println!("Hello, this is main thread");

    // t1.join().unwrap();
    // t2.join().unwrap();

    // let c = Cell::new(1);
    // test_cell(&c, &c);

    // let n = Mutex::new(0);
    // thread::scope(|s| {
    //     for _ in 0..10 {
    //         s.spawn(|| {
    //             let mut guard = n.lock().unwrap();
    //             for _ in 0..100 {
    //                 *guard += 1;
    //             }
    //             drop(guard);
    //             thread::sleep(Duration::from_secs(1));
    //         });
    //     }
    // });

    // assert_eq!(n.into_inner().unwrap(), 1000);

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            // let item = queue.lock().unwrap().pop_front();
            // if let Some(item) = item {
            //     dbg!(item);
            //     println!("{item}");
            // } else {
            //     thread::park();
            // }
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            // t.thread().unpark();
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn f() {
    println!("hello from another thread");

    let id = thread::current().id();
    println!("this is my thread id: {id:?}");
}

fn x(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    if before != after {
        println!("xxx");
    }

    let a = [1, 2, 3];
    let b = unsafe { a.get_unchecked(3) };
}

fn test_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        println!("xxx");
    }
}
