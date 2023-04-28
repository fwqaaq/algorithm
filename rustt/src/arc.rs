use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{ops::Deref, ptr::NonNull, sync::atomic::AtomicUsize};
// struct ArcData<T> {
//     ref_count: AtomicUsize,
//     data: T,
// }

// pub struct Arc<T> {
//     ptr: NonNull<ArcData<T>>,
// }

// unsafe impl<T: Send + Sync> Send for Arc<T> {}
// unsafe impl<T: Send + Sync> Sync for Arc<T> {}

// impl<T> Arc<T> {
//     pub fn new(data: T) -> Arc<T> {
//         Self {
//             ptr: NonNull::from(Box::leak(Box::new(ArcData {
//                 ref_count: AtomicUsize::new(1),
//                 data,
//             }))),
//         }
//     }
//     fn data(&self) -> &ArcData<T> {
//         unsafe { self.ptr.as_ref() }
//     }

//     pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
//         if arc.data().ref_count.load(Relaxed) == 1 {
//             fence(Acquire);

//             unsafe { Some(&mut arc.ptr.as_mut().data) }
//         } else {
//             None
//         }
//     }
// }

// impl<T> Clone for Arc<T> {
//     fn clone(&self) -> Self {
//         if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
//             std::process::abort();
//         }
//         Arc { ptr: self.ptr }
//     }
// }

// impl<T> Deref for Arc<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.data().data
//     }
// }

// impl<T> Drop for Arc<T> {
//     fn drop(&mut self) {
//         if self.data().ref_count.fetch_sub(1, Release) == 1 {
//             fence(Acquire);
//             unsafe {
//                 drop(Box::from_raw(self.ptr.as_ptr()));
//             };
//             println!("xxxxx");
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//         static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

//         #[derive(Debug)]
//         struct DetectDrop;

//         impl Drop for DetectDrop {
//             fn drop(&mut self) {
//                 NUM_DROPS.fetch_add(1, Relaxed);
//                 println!("drop add 1");
//             }
//         }

//         let x = Arc::new(("hello", DetectDrop));
//         let y = x.clone();

//         let t = std::thread::spawn(move || {
//             assert_eq!(x.0, "hello");
//         });

//         assert_eq!(y.0, "hello");

//         t.join().unwrap();

//         assert_eq!(NUM_DROPS.load(Relaxed), 0);
//         drop(y);
//         assert_eq!(NUM_DROPS.load(Relaxed), 1);
//     }
// }

///////////////////////////////////////////////////////////////////////////

// struct ArcData<T> {
//     // Numbers of Arc
//     data_ref_count: AtomicUsize,
//     // Numbers of Arc and Weak combined
//     alloc_ref_count: AtomicUsize,
//     data: UnsafeCell<Option<T>>,
// }

// pub struct Arc<T> {
//     weak: Weak<T>,
// }

// pub struct Weak<T> {
//     ptr: NonNull<ArcData<T>>,
// }

// unsafe impl<T: Sync + Send> Sync for Weak<T> {}
// unsafe impl<T: Sync + Send> Send for Weak<T> {}

// impl<T> Arc<T> {
//     pub fn new(data: T) -> Self {
//         Arc {
//             weak: Weak {
//                 ptr: NonNull::from(Box::leak(Box::new(ArcData {
//                     data_ref_count: AtomicUsize::new(1),
//                     alloc_ref_count: AtomicUsize::new(1),
//                     data: UnsafeCell::new(Some(data)),
//                 }))),
//             },
//         }
//     }

//     pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
//         if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
//             fence(Acquire);
//             let arcdata = unsafe { arc.weak.ptr.as_mut() };
//             let option = arcdata.data.get_mut();
//             option.as_mut()
//             // Some(data)
//         } else {
//             None
//         }
//     }

//     pub fn downgrade(arc: &Self) -> Weak<T> {
//         arc.weak.clone()
//     }
// }

// impl<T> Weak<T> {
//     fn data(&self) -> &ArcData<T> {
//         unsafe { self.ptr.as_ref() }
//     }

//     pub fn upgrade(&self) -> Option<Arc<T>> {
//         println!("data {:?}", self.data().data_ref_count.load(Relaxed));
//         println!("alloc {:?}", self.data().alloc_ref_count.load(Relaxed));
//         let mut n = self.data().data_ref_count.load(Relaxed);
//         loop {
//             if n == 0 {
//                 return None;
//             }
//             assert!(n < usize::MAX);
//             if let Err(e) =
//                 self.data()
//                     .data_ref_count
//                     .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
//             {
//                 n = e;
//                 continue;
//             }
//             return Some(Arc { weak: self.clone() });
//         }
//     }
// }

// impl<T> Deref for Arc<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         let ptr = self.weak.data().data.get();
//         unsafe { (*ptr).as_ref().unwrap() }
//     }
// }

// impl<T> Clone for Weak<T> {
//     fn clone(&self) -> Self {
//         if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
//             std::process::abort();
//         }
//         Weak { ptr: self.ptr }
//     }
// }

// impl<T> Clone for Arc<T> {
//     fn clone(&self) -> Self {
//         let weak = self.weak.clone();
//         if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
//             std::process::abort();
//         }
//         Arc { weak }
//     }
// }

// impl<T> Drop for Weak<T> {
//     fn drop(&mut self) {
//         if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
//             fence(Acquire);
//             unsafe { drop(Box::from_raw(self.ptr.as_ptr())) }
//         }
//     }
// }

// impl<T> Drop for Arc<T> {
//     fn drop(&mut self) {
//         if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
//             fence(Acquire);
//             let ptr = self.weak.data().data.get();
//             unsafe {
//                 *ptr = None;
//             }
//         }
//     }
// }

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    let x = Arc::new(("hello", DetectDrop));
    let y = Arc::downgrade(&x);
    let z = Arc::downgrade(&x);

    let t = std::thread::spawn(move || {
        let y = y.upgrade().unwrap();
        assert_eq!(y.0, "hello");
    });

    assert_eq!(x.0, "hello");
    t.join().unwrap();

    drop(x);
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
    assert!(z.upgrade().is_none());
}

fn main() {}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    data: UnsafeCell<ManuallyDrop<T>>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                alloc_ref_count: AtomicUsize::new(1),
                data_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // 与 Drop 中的释放操作匹配
        //确保任意 upgrade 之后的指针在内存中可见
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }
        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        //为了确保在 downgrade 之后对 data_ref_count 的更改
        //不会更改上面的 is_unique 的结果
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique {
            return None;
        }
        //匹配 Arc::drop 中正在释放而减少的 data_ref_count
        //确保没有其它内容正在访问数据
        fence(Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut n = arc.data().alloc_ref_count.load(Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = arc.data().alloc_ref_count.load(Relaxed);
                continue;
            }

            assert!(n < usize::MAX - 1);
            if let Err(e) = arc
                .data()
                .alloc_ref_count
                .compare_exchange(n, n + 1, Acquire, Relaxed)
            {
                n = e;
                continue;
            }
            println!("data {:?}", arc.data().data_ref_count.load(Relaxed));
            println!("alloc {:?}", arc.data().alloc_ref_count.load(Relaxed));
            return Weak { ptr: arc.ptr };
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe { ManuallyDrop::drop(&mut *self.data().data.get()) };
            drop(Weak { ptr: self.ptr });
        }
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                println!(
                    "downgrade data {:?}",
                    self.data().data_ref_count.load(Relaxed)
                );
                println!(
                    "downgrade alloc {:?}",
                    self.data().alloc_ref_count.load(Relaxed)
                );
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}
