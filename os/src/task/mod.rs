//! 任务管理器实现
//!
//! 所有与任务管理相关的内容，例如启动和切换任务，都在此处实现。
//!
//! 一个名为`TASK_MANAGER`的[`TaskManager`]全局单例实例控制着操作系统中的所有任务。
//!
//! 当你在`switch.S`中看到`__switch`汇编函数时要小心。该函数周围的控制流可能与你的预期不符。


mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

/// 任务管理器，所有任务都在此处被管理。
///
/// 在`TaskManager`上实现的函数处理所有任务状态转换
/// 以及任务上下文切换。为方便起见，你可以在模块级别找到围绕它的包装器。
///
/// `TaskManager`的大部分内容都隐藏在`inner`字段后面，以将借用检查延迟到运行时。你可以在`TaskManager`的现有函数中看到如何使用`inner`的示例。
pub struct TaskManager {
    /// 任务总数
    num_app: usize,
    /// 使用内部值来获取可变访问权限
    inner: UPSafeCell<TaskManagerInner>,
}

/// 任务管理器的内部
pub struct TaskManagerInner {
    /// 任务列表
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// 当前运行中的id
    current_task: usize,
}

lazy_static! {
    /// 全局变量：TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    /// 运行任务列表中的第一个任务。
    ///
    /// 通常，任务列表中的第一个任务是空闲任务（我们稍后称之为零进程）。
    /// 但在第3章中，我们静态加载应用程序，所以第一个任务是一个真正的应用程序。
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // 在此之前，我们应该手动释放那些必须手动释放的局部变量
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    /// 将当前“运行中”（`Running`）任务的状态更改为“就绪”（`Ready`）。
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// 将当前“运行中”（`Running`）任务的状态更改为“已退出”（`Exited`）。
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// 找到下一个要运行的任务并返回应用程序ID。
    ///
    /// 在这种情况下，我们只返回任务列表中的第一个“就绪”（`Ready`）任务。
    /// 范围 [当前任务序号加一，当前任务序号+任务总数)
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// 将当前“运行中”（`Running`）任务切换到我们找到的任务，
    /// 或者如果没有“就绪”（`Ready`）任务，我们可以在所有应用程序完成后退出
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // 在此之前，我们应该手动释放那些必须手动释放的局部变量
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }
}

    /// 运行第一个任务
    pub fn run_first_task() {
        TASK_MANAGER.run_first_task();
    }

    /// 运行下一个任务
    fn run_next_task() {
        TASK_MANAGER.run_next_task();
    }

    /// 暂停当前任务
    fn mark_current_suspended() {
        TASK_MANAGER.mark_current_suspended();
    }

    /// 退出当前任务
    fn mark_current_exited() {
        TASK_MANAGER.mark_current_exited();
    }

    /// 暂停当前任务，然后运行下一个任务
    pub fn suspend_current_and_run_next() {
        mark_current_suspended();
        run_next_task();
    }

    /// 退出当前任务，然后运行下一个任务
    pub fn exit_current_and_run_next() {
        mark_current_exited();
        run_next_task();
    }
