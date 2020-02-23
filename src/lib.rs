mod lock_manager;
mod mutex;
mod rwlock;
pub use mutex::{Mutex, MutexGuard};
pub use rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub mod prelude {
    pub use crate::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
}
/// Replaces the locks defined in this crate by their `std` counterparts.
/// Convenient to switch to the less costly `std` implementations.
pub mod std_override {
    pub use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
}