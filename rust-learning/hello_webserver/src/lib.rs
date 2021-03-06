use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::borrow::Borrow;
use std::ops::Deref;

pub mod event_loop;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Arc<Mutex<Sender<Message>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new TheadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Arc::new(Mutex::new(sender)) }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let message = Message::NewJob(Box::new(f));

        self.sender.lock().unwrap().send(message).unwrap();
    }

    pub fn execute_then<F, T>(&self, f: F, then: T)
        where
            F: FnOnce() + Send + 'static,
            T: FnOnce() + Send + 'static,
    {
        let sender = Arc::clone(&self.sender);

        let func = move || {
            f();

            sender.lock().unwrap().send(Message::NewJob(Box::new(then))).unwrap();
        };

        let message = Message::NewJob(Box::new(func));

        self.sender.lock().unwrap().send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.lock().unwrap().send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                if let Message::NewJob(job) = message {
                    println!("Worker {} got a new job; executing.", id);

                    job();
                } else {
                    println!("Worker {} got Terminate signal. Terminating", id);

                    break;
                }
            });

        let thread = Some(thread);

        Worker {
            id,
            thread,
        }
    }
}