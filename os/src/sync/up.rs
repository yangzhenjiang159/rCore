//! 单核处理器内部可变性原语

use core::cell::{RefCell, RefMut};

/// 将一个静态数据结构包裹在其中，这样我们就能
/// 在不使用任何`unsafe`的情况下访问它。
///
/// 我们只应在单处理器中使用它。
///
/// 若要获取内部数据的可变引用，请调用
/// `exclusive_access`。
pub struct UPSafeCell<T> {
    /// 内部数据
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// 用户有责任保证内部结构体仅在单处理器中使用。
    pub unsafe fn new(data: T) -> Self {
        Self {
            inner: RefCell::new(data),
        }
    }

    /// 对UPSafeCell中的内部数据进行独占访问。如果数据已被借用，则会触发 panic。
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}