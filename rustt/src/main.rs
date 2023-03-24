// use std::{
//     collections::VecDeque,
//     sync::{Condvar, Mutex},
// };

// pub struct Channel<T> {
//     queue: Mutex<VecDeque<T>>,
//     iterm_ready: Condvar,
// }

// impl<T> Default for Channel<T> {
//     fn default() -> Self {
//         Channel::new()
//     }
// }

// impl<T> Channel<T> {
//     pub fn new() -> Self {
//         Self {
//             queue: Mutex::new(VecDeque::new()),
//             iterm_ready: Condvar::new(),
//         }
//     }

//     pub fn send(&self, message: T) {
//         self.queue.lock().unwrap().push_back(message);
//         self.iterm_ready.notify_one();
//     }

//     pub fn receive(&self) -> T {
//         let mut b = self.queue.lock().unwrap();
//         loop {
//             if let Some(message) = b.pop_front() {
//                 return message;
//             }
//             b = self.iterm_ready.wait(b).unwrap();
//         }
//     }
// }

//////////////////////////////////////////////////////////////////////////

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

// pub struct Channel<T> {
//     messgage: UnsafeCell<MaybeUninit<T>>,
//     ready: AtomicBool,
//     in_use: AtomicBool,
// }

// unsafe impl<T> Sync for Channel<T> where T: Send {}

// impl<T> Channel<T> {
//     pub const fn new() -> Self {
//         Self {
//             messgage: UnsafeCell::new(MaybeUninit::uninit()),
//             in_use: AtomicBool::new(false),
//             ready: AtomicBool::new(false),
//         }
//     }

//     /// unsafe: Only call once
//     pub fn send(&self, message: T) {
//         if self.in_use.swap(true, Relaxed) {
//             panic!("can't send more than one mesage");
//         }
//         unsafe { (*self.messgage.get()).write(message) };
//         self.ready.store(true, Release);
//     }

//     pub fn is_ready(&self) -> bool {
//         self.ready.load(Relaxed)
//     }

//     /// Safety: Only call this one, and only after is_ready() returns
//     /// panics if no message is available yet
//     pub fn receive(&self) -> T {
//         if !self.ready.swap(false, Acquire) {
//             panic!("no message Availad")
//         }
//         unsafe { (*self.messgage.get()).assume_init_read() }
//     }
// }

// impl<T> Drop for Channel<T> {
//     fn drop(&mut self) {
//         if *self.ready.get_mut() {
//             unsafe { self.messgage.get_mut().assume_init_drop() }
//         }
//     }
// }

//////////////////////////////////////////////////////////////////////

const EMPTY: u8 = 0;
const WAITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    pub fn send(&self, message: T) {
        // 在这里使用 Relaxed 内存序，因为这里只修改了一个变量值
        // 并且没有以来于其它内存操作
        if self
            .state
            .compare_exchange(EMPTY, WAITING, Relaxed, Relaxed)
            .is_err()
        {
            panic!("can't send more than one mesage");
        }
        unsafe {
            (*self.message.get()).write(message);
        }
        // 在这里，使用 Release 告诉其他线程 write 以及 compare_exchange 操作都在它之前发生了
        // 所以当 store 操作完成时，会通知其他线程可以安全地读取 state 的 READY
        // 比如之后的 park 就可以 unpark 了
        self.state.store(READY, Release)
    }

    pub fn is_ready(&self) -> bool {
        // 在这里 state 的值为 READY，不需要依赖其他操作
        // 如果有其他线程在 is_ready 方法返回 true 之后尝试调用 receive 方法，
        // 它们会看到状态已经改变为 READING，并会失败。并且这会导致性能下降，
        // Acquire 内存顺序会等待之前的所有操作完成，然后刷新内存
        self.state.load(Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        // 在这里，成功状态需要使用 Acquire 标明，让其他线程可见
        // 如果使用 Relaxed 内存顺序，对于其他线程不可见，可能当前状态还是 READY
        if self
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("no message Availad")
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("message"); // park 的时候，可能会执行该线程
            channel.send("hello");
            channel.send("hello");
            channel.send("hello");
            channel.send("hello");
            channel.send("hello");
            channel.send("hello");
            t.unpark();
        });
        // 如果未就绪，则阻塞当前线程
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!("message", channel.receive());
    });
}
