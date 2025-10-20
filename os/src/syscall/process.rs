//! 应用程序管理系统调用
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

/// 任务退出并提交退出代码
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}


/// 当前任务为其他任务放弃资源
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// 以毫秒为单位获取时间
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
