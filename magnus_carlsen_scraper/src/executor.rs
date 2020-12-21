use std::{
    fmt::{Debug, Display},
    future::Future,
    pin::Pin,
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake},
};

use crate::waker::wake_ref;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a + Send>>;

pub struct Executor<'a, T> {
    ready_queue: Receiver<Arc<Task<'a, T>>>,
}

#[derive(Clone)]
pub struct Spawner<'a, T> {
    task_sender: Sender<Arc<Task<'a, T>>>,
}

pub struct Task<'a, T> {
    future: Mutex<Option<BoxFuture<'a, T>>>,

    task_sender: Sender<Arc<Task<'a, T>>>,
}

impl<'a, T> Spawner<'a, T> {
    pub fn spawn(&self, future: impl Future<Output = T> + 'static + Send) {
        let future = Box::pin(future);

        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl<'a, T> Wake for Task<'a, T> {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {
        let cloned = self.clone();

        match self.task_sender.send(cloned) {
            Ok(_) => {}
            Err(e) => {
                //  self.task_sender.send(e.0).unwrap();
            }
        }
    }
}

impl<'a, T> Executor<'a, T> {
    pub fn run(&self, callback: Box<dyn Fn(T) + 'a>) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                let waker = wake_ref(&task);

                let context = &mut Context::from_waker(&*waker);

                match future.as_mut().poll(context) {
                    Poll::Ready(value) => callback(value),
                    Poll::Pending => *future_slot = Some(future),
                };
            }
        }
    }
    //mut callback: Box<dyn FnMut(T) + 'a>
    pub fn block_on(&self) -> Option<T>
    where
        T: Debug,
    {
        if let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                let waker = wake_ref(&task);

                let context = &mut Context::from_waker(&*waker);

                if let Poll::Ready(v) = future.as_mut().poll(context) {
                    println!("v {:?}", v);
                    return Some(v);
                } else {
                    *future_slot = Some(future);
                }
            }
        }

        None
    }
}

pub fn new_executor_and_spawner<'a, T>() -> (Executor<'a, T>, Spawner<'a, T>) {

    let (task_sender, ready_queue) = channel();

    (Executor { ready_queue }, Spawner { task_sender })
}
