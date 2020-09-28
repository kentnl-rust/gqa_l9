extern crate crossbeam;

//use crossbeam::sync::MsQueue;
//use crossbeam::sync::SegQueue;
use crossbeam::scope;

use std::sync::Arc;






use std::sync::atomic::Ordering::{SeqCst};
use std::{ptr, mem};

use crossbeam::mem::epoch::{self, Atomic, Owned};
use crossbeam::mem::CachePadded;

/// A Michael-Scott lock-free queue.
///
/// Usable with any number of producers and consumers.
pub struct MsQueue<T> {
    head: CachePadded<Atomic<Node<T>>>,
    tail: CachePadded<Atomic<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> MsQueue<T> {
    /// Create a new, empty queue.
    pub fn new() -> MsQueue<T> {
    let q = MsQueue {
        head: CachePadded::new(Atomic::null()),
        tail: CachePadded::new(Atomic::null()),
    };
    let sentinel = Owned::new(Node {
        data: unsafe { mem::uninitialized() },
        next: Atomic::null()
    });
    let guard = epoch::pin();
    let sentinel = q.head.store_and_ref(sentinel, SeqCst, &guard);
    q.tail.store_shared(Some(sentinel), SeqCst);
    q
}

/// Add `t` to the back of the queue.
pub fn push(&self, t: T) {
    let mut n = Owned::new(Node {
        data: t,
        next: Atomic::null()
    });
    loop {
        let guard = epoch::pin();
        let tail = self.tail.load(SeqCst, &guard).unwrap();
        if let Some(next) = tail.next.load(SeqCst, &guard) {
            self.tail.cas_shared(Some(tail), Some(next), SeqCst);
            continue;
        }

        match tail.next.cas_and_ref(None, n, SeqCst, &guard) {
            Ok(shared) => {
            self.tail.cas_shared(Some(tail), Some(shared), SeqCst);
            break;
        }
        Err(owned) => {
        n = owned;
    }
}
        }
    }

    /// Attempt to dequeue from the front.
    ///
    /// Returns `None` if the queue is observed to be empty.
    pub fn pop(&self) -> Option<T> {
        loop {
            let guard = epoch::pin();
            let head = self.head.load(SeqCst, &guard).unwrap();

            if let Some(next) = head.next.load(SeqCst, &guard) {
                unsafe {
                    if self.head.cas_shared(Some(head), Some(next), SeqCst) {
                        guard.unlinked(head);
                        return Some(ptr::read(&(*next).data))
                    }
                }
            } else {
                return None
            }
        }
    }
}






const DUP: usize = 4;
const THREADS: u32 = 2;
const COUNT: u64 = 100000;

fn main() {
    scope(|s| {
        for _i in 0..DUP {
            let q = Arc::new(MsQueue::new());
            let qs = q.clone();

            s.spawn(move || {
                for i in 1..COUNT { qs.push(i) }
            });

            for _i in 0..THREADS {
                let qr = q.clone();
                s.spawn(move || {
                    let mut cur: u64 = 0;
                    for _j in 0..COUNT {
                        if let Some(new) = qr.pop() {
                            assert!(new > cur);
                            cur = new;
                        }
                    }
                });
            }
        }
    });
}
