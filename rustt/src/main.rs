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
    sync::{
        atomic::{AtomicBool, AtomicI32, AtomicU64, AtomicUsize},
        Condvar, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use std::sync::atomic::Ordering::Relaxed;

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

    // let queue = Mutex::new(VecDeque::new());
    // let not_empty = Condvar::new();

    // thread::scope(|s| {
    //     s.spawn(|| loop {
    //         // let item = queue.lock().unwrap().pop_front();
    //         // if let Some(item) = item {
    //         //     dbg!(item);
    //         //     println!("{item}");
    //         // } else {
    //         //     thread::park();
    //         // }
    //         let mut q = queue.lock().unwrap();
    //         let item = loop {
    //             if let Some(item) = q.pop_front() {
    //                 break item;
    //             } else {
    //                 q = not_empty.wait(q).unwrap();
    //             }
    //         };
    //         drop(q);
    //         dbg!(item);
    //     });

    //     for i in 0.. {
    //         queue.lock().unwrap().push_back(i);
    //         // t.thread().unpark();
    //         not_empty.notify_one();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // static STOP: AtomicBool = AtomicBool::new(false);
    // let background_thread = thread::spawn(|| {
    //     while !STOP.load(Relaxed) {
    //         thread::sleep(Duration::from_secs(1));
    //         println!("help");
    //     }
    // });
    // for line in std::io::stdin().lines() {
    //     match line.unwrap().as_str() {
    //         "help" => println!("commands: help, stop"),
    //         "stop" => break,
    //         cmd => println!("unkown command, {cmd:?}"),
    //     }
    // }
    // STOP.store(true, Relaxed);
    // background_thread.join().unwrap();

    // let num_done = AtomicUsize::new(0);
    // let main_thread = thread::current();
    // thread::scope(|s| {
    //     s.spawn(|| {
    //         for i in 0..100 {
    //             println!("{i}");
    //             num_done.store(i + 1, Relaxed);
    //             main_thread.unpark();
    //         }
    //     });
    //     loop {
    //         let n = num_done.load(Relaxed);
    //         if n == 100 {
    //             break;
    //         }
    //         println!("Working.. {n}/100 done");
    //         // thread::sleep(Duration::from_secs(1));
    //         thread::park_timeout(Duration::from_secs(1));
    //     }
    // });
    // println!("Done!");

    // let num_done = &AtomicI32::new(0);
    // thread::scope(|s| {
    //     for t in 0..4 {
    //         s.spawn(move || {
    //             for i in 0..25 {
    //                 println!("{}", t * 25 + i);
    //                 num_done.fetch_add(1, Relaxed);
    //             }
    //         });
    //     }

    //     loop {
    //         let n = num_done.load(Relaxed);
    //         if n == 100 {
    //             break;
    //         }
    //         println!("Working.. {n}/100 done");
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // println!("Done");

    // let num_done = &AtomicUsize::new(0);
    // let total_time = &AtomicU64::new(0);
    // let max_time = &AtomicU64::new(0);
    // thread::scope(|s| {
    //     for t in 0..4 {
    //         s.spawn(move || {
    //             for i in 0..25 {
    //                 let start = Instant::now();
    //                 println!("{}", t * 25 + i);
    //                 for i in 0..1000 {
    //                     let _ = t * 1000 + i * 100 + 999 - 100;
    //                 }
    //                 let time_taken = start.elapsed().as_micros() as u64;
    //                 num_done.fetch_add(1, Relaxed);
    //                 total_time.fetch_add(time_taken, Relaxed);
    //                 max_time.fetch_max(time_taken, Relaxed);
    //             }
    //         });
    //     }
    //     loop {
    //         let total_time = Duration::from_micros(total_time.load(Relaxed));
    //         let max_time = Duration::from_micros(max_time.load(Relaxed));
    //         let n = num_done.load(Relaxed);
    //         if n == 100 {
    //             break;
    //         }
    //         if n == 0 {
    //             println!("Working.. nothing done yet");
    //         } else {
    //             println!(
    //                 "Working.. {n}/100 done, {:?} average, {:?} peak",
    //                 total_time / n as u32,
    //                 max_time
    //             );
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // println!("Done");

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                println!("{}", get_x());
            });
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

fn increment(a: &AtomicI32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

fn allocate_new_id() -> i32 {
    static NEXT_ID: AtomicI32 = AtomicI32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "TOO MANY IDs!!!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x += 1;
        X.store(x, Relaxed);
    }
    x
}
