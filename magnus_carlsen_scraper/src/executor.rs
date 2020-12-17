use std::{fmt::Display, future::Future, pin::Pin, sync::mpsc::{sync_channel, Receiver, SyncSender}, sync::{Arc, Mutex}, task::{Context, Poll, Wake}};

use crate::waker::wake_ref;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a + Send>>;

pub struct Executor<T> {
    ready_queue: Receiver<Arc<Task<T>>>,
}

#[derive(Clone)]
pub struct Spawner<T> {
    task_sender: SyncSender<Arc<Task<T>>>,
}

pub struct Task<T> {
    future: Mutex<Option<BoxFuture<'static, T>>>,

    task_sender: SyncSender<Arc<Task<T>>>,
}

impl<T> Spawner<T> {
    pub fn spawn(&self, future: impl Future<Output = T> + 'static + Send) {
        let future = Box::pin(future);

        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl<T> Wake for Task<T> {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {
        let cloned = self.clone();

        self.task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl<T > Executor<T>  where T: Display {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                let waker = wake_ref(&task);

                let context = &mut Context::from_waker(&*waker);

                if let Poll::Ready(value) = future.as_mut().poll(context) {

                    println!("VALUE: {}",value);

                } else {
                    
                    *future_slot = Some(future);
                }

                // match future.as_mut().poll(context) {
                //     Poll::Ready(value) => {value},
                //     Poll::Pending => {
                //         *future_slot = Some(future);
                //     }
                // };
            }
        };
    }
}

pub fn new_executor_and_spawner<T>() -> (Executor<T>, Spawner<T>) {
    const MAX_QUEUED_TASKS: usize = 10_000;

    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}
