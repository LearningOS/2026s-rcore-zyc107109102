//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::config::BIGSTRIDE;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        //self.ready_queue.pop_front()
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut minn = isize::MAX;
        let mut idx = 0;
        for (i , task) in self.ready_queue.iter().enumerate() {
            let stride = task.inner_exclusive_access().stride;
            if stride < minn {
                minn = stride;
                idx = i;
            }
        } 
        let task = self.ready_queue.remove(idx).unwrap();
        {
            let mut task_inner = task.inner_exclusive_access();
            let priority = task_inner.prio.max(2);
            task_inner.stride += BIGSTRIDE / priority;
        }
        Some(task)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
