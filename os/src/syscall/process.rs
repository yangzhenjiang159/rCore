//! 应用程序管理系统调用
use crate::batch::run_next_app;

/// 任务退出并提交退出代码
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
