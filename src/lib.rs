mod my_task;

use futures::task;
use my_task::Task;
use std::{collections::VecDeque, future::Future, sync::mpsc, sync::Arc, task::Context};
// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

///
///
pub struct MiniTokio {
    // tasks: VecDeque<Task>,
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokio {
    /// Initialize a new mini-toio instance
    pub fn new() -> MiniTokio {
        // MiniTokio {
        //     tasks: VecDeque::new(),
        // }
        let (sender, scheduled) = mpsc::channel();

        MiniTokio { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance
    ///
    /// The given future is wrapped with the `Task` harness and
    /// pushed into the `scheduled` queue. The future will be executed
    /// when `run` is called.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // self.tasks.push_back(Box::pin(future));
        Task::spawn(future, &self.sender);
    }

    pub fn run(&mut self) {
        // let waker = task::noop_waker();
        // let mut cx = Context::from_waker(&waker);

        // while let Some(mut task) = self.tasks.pop_front() {
        //     if task.as_mut().poll(&mut cx).is_pending() {
        //         self.tasks.push_back(task);
        //     }
        // }

        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}
