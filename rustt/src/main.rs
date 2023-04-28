use std::{
    sync::atomic::{compiler_fence, AtomicBool, AtomicUsize},
    thread,
};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        // Spawn four threads, that each iterate a million times.
        for _ in 0..20 {
            s.spawn(|| {
                for _ in 0..100_000 {
                    // Acquire the lock, using the wrong memory ordering.
                    while locked.swap(true, Relaxed) {}
                    compiler_fence(Acquire);

                    // Non-atomically increment the counter, while holding the lock.
                    let old = counter.load(Relaxed);
                    let new = old + 1;
                    counter.store(new, Relaxed);

                    // Release the lock, using the wrong memory ordering.
                    compiler_fence(Release);
                    locked.store(false, Relaxed);
                }
            });
        }
    });

    println!("{}", counter.into_inner());
}
