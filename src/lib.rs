#![no_std]
#![allow(unused)]
extern crate alloc;

#[macro_use]
extern crate log;
mod file;
mod fs_type;
mod inode;

use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};
use jammdb::{DB};
use rvfs::StrResult;
use spin::{Once};

pub use fs_type::DBFS_TYPE;

struct SafeDb(DB);

impl Deref for SafeDb {
    type Target = DB;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SafeDb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Sync for SafeDb {}
unsafe impl Send for SafeDb {}

static DB: Once<Arc<SafeDb>> = Once::new();

/// Initialize the global DBFS database
pub fn init_dbfs(db: DB) -> StrResult<()> {
    DB.call_once(|| Arc::new(SafeDb(db)));
    Ok(())
}

fn clone_db() -> Arc<SafeDb> {
    DB.get().unwrap().clone()
}
