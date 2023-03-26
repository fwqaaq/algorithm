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

use std::marker::PhantomData;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread::{self, Thread};
use std::time::Duration;
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

// const EMPTY: u8 = 0;
// const WAITING: u8 = 1;
// const READY: u8 = 2;
// const READING: u8 = 3;

// pub struct Channel<T> {
//     message: UnsafeCell<MaybeUninit<T>>,
//     state: AtomicU8,
// }

// unsafe impl<T> Sync for Channel<T> where T: Send {}

// impl<T> Channel<T> {
//     pub const fn new() -> Self {
//         Self {
//             message: UnsafeCell::new(MaybeUninit::uninit()),
//             state: AtomicU8::new(EMPTY),
//         }
//     }

//     pub fn send(&self, message: T) {
//         // 在这里使用 Relaxed 内存序，因为这里只修改了一个变量值
//         // 并且没有以来于其它内存操作，如果有多个线程调用
//         // 依然可能会调用之后的 write 函数，所以这里只能是 1 对 1 的情况
//         if self
//             .state
//             .compare_exchange(EMPTY, WAITING, Relaxed, Relaxed)
//             .is_err()
//         {
//             panic!("can't send more than one mesage");
//         }
//         unsafe {
//             (*self.message.get()).write(message);
//         }
//         // 在这里，使用 Release 告诉其他线程 write 以及 compare_exchange 操作都在它之前发生了
//         // 所以当 store 操作完成时，会通知其他线程可以安全地读取 state 的 READY
//         // 比如之后的 park 就可以 unpark 了
//         self.state.store(READY, Release)
//     }

//     pub fn is_ready(&self) -> bool {
//         // 在这里 state 的值为 READY，不需要依赖其他操作
//         // 如果有其他线程在 is_ready 方法返回 true 之后尝试调用 receive 方法，
//         // 它们会看到状态已经改变为 READING，并会失败。并且这会导致性能下降，
//         // Acquire 内存顺序会等待之前的所有操作完成，然后刷新内存
//         self.state.load(Relaxed) == READY
//     }

//     pub fn receive(&self) -> T {
//         // 在这里，成功状态需要使用 Acquire 标明，让其他线程可见
//         // 如果使用 Relaxed 内存顺序，对于其他线程不可见，可能当前状态还是 READY
//         if self
//             .state
//             .compare_exchange(READY, READING, Acquire, Relaxed)
//             .is_err()
//         {
//             panic!("no message Availad")
//         }

//         unsafe { (*self.message.get()).assume_init_read() }
//     }
// }

// impl<T> Drop for Channel<T> {
//     fn drop(&mut self) {
//         if *self.state.get_mut() == READY {
//             unsafe { self.message.get_mut().assume_init_drop() }
//         }
//     }
// }

// fn main() {
//     let channel = Channel::new();
//     let t = thread::current();
//     thread::scope(|s| {
//         s.spawn(|| {
//             channel.send("message");
//             t.unpark();
//         });
//         // 如果未就绪，则阻塞当前线程
//         while !channel.is_ready() {
//             thread::park();
//         }
//         // assert_eq!("message", channel.receive());
//         println!("{}.state {:?}", channel.receive(), channel.state);
//     });
// }

///////////////////////////////////////////////////////////////////////

// struct Channel<T> {
//     message: UnsafeCell<MaybeUninit<T>>,
//     ready: AtomicBool,
// }

// pub struct Sender<T> {
//     channel: Arc<Channel<T>>,
// }

// pub struct Receiver<T> {
//     channel: Arc<Channel<T>>,
// }

// unsafe impl<T> Sync for Channel<T> where T: Send {}

// pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
//     let a = Arc::new(Channel {
//         message: UnsafeCell::new(MaybeUninit::uninit()),
//         ready: AtomicBool::new(false),
//     });
//     (Sender { channel: a.clone() }, Receiver { channel: a })
// }

// impl<T> Sender<T> {
//     pub fn send(self, message: T) {
//         unsafe { (*self.channel.message.get()).write(message) };
//         self.channel.ready.store(true, Release);
//     }
// }

// impl<T> Receiver<T> {
//     pub fn is_ready(&self) -> bool {
//         self.channel.ready.load(Relaxed)
//     }

//     pub fn receive(self) -> T {
//         if !self.channel.ready.swap(false, Acquire) {
//             panic!("no message availbale!!");
//         }

//         unsafe { (*self.channel.message.get()).assume_init_read() }
//     }
// }

// impl<T> Drop for Channel<T> {
//     fn drop(&mut self) {
//         if *self.ready.get_mut() {
//             unsafe { self.message.get_mut().assume_init_drop() };
//         }
//     }
// }

// fn main() {
//     thread::scope(|s| {
//         let (sender, receiver) = channel();
//         let t = thread::current();
//         s.spawn(move || {
//             sender.send("hello");
//             t.unpark();
//         });
//         while !receiver.is_ready() {
//             thread::park();
//         }
//         assert_eq!("hello", receiver.receive());
//     });
// }

//////////////////////////////////////////////////////////////////////

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub struct Sender<'a, T: 'a> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}

// _no_send 是阻止 Receiver 在线程间共享，
// 比如 move 语义，可以将 Sender 移动到其他线程
pub struct Receiver<'a, T: 'a> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Channel<T> {
    // 编译时初始化
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        };
        self.channel.ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("hello");
        });
        assert_eq!("hello", receiver.receive());
    });
}
