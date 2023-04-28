use std::{
    hint::black_box,
    sync::atomic::{compiler_fence, AtomicBool, AtomicI32, AtomicI64, AtomicUsize},
    thread,
    time::Instant,
};

use core::sync::atomic::Ordering::Relaxed;

#[repr(align(64))]
struct Aligned(AtomicI64);

static A: [Aligned; 3] = [
    Aligned(AtomicI64::new(0)),
    Aligned(AtomicI64::new(0)),
    Aligned(AtomicI64::new(0)),
];
fn main() {
    // black_box(&A);
    // thread::spawn(|| loop {
    //     A[0].0.store(0, Relaxed);
    //     A[2].0.store(0, Relaxed);
    // });
    // let start = Instant::now();
    // for _ in 0..1_000_000_000 {
    //     black_box(A[1].0.load(Relaxed));
    // }
    // println!("{:?}", start.elapsed());

    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    while locked.swap(true, Relaxed) {}
                    compiler_fence(std::sync::atomic::Ordering::Acquire);

                    let old = counter.load(Relaxed);
                    let new = old + 1;
                    counter.store(new, Relaxed);
                    compiler_fence(std::sync::atomic::Ordering::Release);
                    locked.store(false, Relaxed);
                }
            });
        }
    });
    println!("{}", counter.into_inner());
}
