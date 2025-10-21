mod blockreader;
mod data;
mod decoder;
mod inode;
mod ufs;

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "macos"))]
pub const ENOATTR: i32 = libc::ENOATTR;
#[cfg(target_os = "linux")]
pub const ENOATTR: i32 = libc::ENODATA;
#[cfg(windows)]
pub const ENOATTR: i32 = 93; // ENODATA equivalent on Windows

pub use crate::{
	blockreader::{Backend, BlockReader},
	data::{InodeAttr, InodeNum, InodeType},
	ufs::{Info, Ufs},
};
