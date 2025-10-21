use std::{
	fs::{self, OpenOptions},
	io::{Write, Result, Error as IoError, ErrorKind},
	path::PathBuf,
};

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::io;

use rufs::{InodeNum, InodeType, InodeAttr};
use crate::{Fs};

fn lookup(fs: &mut Fs, path: &str) -> Result<InodeNum> {
	if !path.starts_with('/') {
		return Err(IoError::new(ErrorKind::InvalidInput, "Path must be absolute"));
	}

	let mut inr = InodeNum::ROOT;
	
	for comp in path.split('/').filter(|s| !s.is_empty()) {
		inr = fs.ufs.dir_lookup(inr, comp.as_ref())?;
	}
	Ok(inr)
}

fn getattr(fs: &mut Fs, inr: InodeNum) -> Result<InodeAttr> {
	let ino = fs.ufs.inode_attr(inr)?;
	Ok(ino)
}

fn read(fs: &mut Fs, inr: InodeNum, size: u64) -> Result<Vec<u8>> {
	let off: u64 = 0;
	let mut buf = vec![0u8; size as usize];
	let _num = fs.ufs.inode_read(inr, off, &mut buf)?;

	Ok(buf)
}

struct FsEntry {
	name: String,
	kind: InodeType,
	inr: InodeNum,
}

pub fn run(mut fs: Fs, out_path: &PathBuf) -> Result<()> {
	let path = "/";
	run_tree(&mut fs, path, 0, out_path)?;
	Ok(())
}

fn run_tree(fs: &mut Fs, path: &str, depth: usize, out_path: &PathBuf) -> Result<()> {
	let entries = read_dir(fs, path)?;
	
	for entry in entries {
		let full_path = if path == "/" {
			format!("/{}", entry.name)
		} else {
			format!("{}/{}", path, entry.name)
		};
		let save_path = out_path.join(&full_path.trim_start_matches('/'));

		if entry.kind == InodeType::Symlink {
			let link = fs.ufs.symlink_read(entry.inr)?;
			let dest = String::from_utf8_lossy(&link).to_string();
			println!("{} : Symlink to {}", full_path, dest);
		}

		if entry.kind == InodeType::Directory {
			println!("{} : Directory", full_path);
			fs::create_dir_all(&save_path)?;

			run_tree(fs, &full_path, depth + 1, out_path)?;
		}

		if entry.kind == InodeType::RegularFile {
			let attr = getattr(fs, entry.inr)?;
			println!("{} : Regular File, Size: {}", full_path, attr.size);

			let data = read(fs, entry.inr, attr.size)?;
			let mut out_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(save_path)?; 
            out_file.write_all(&data)?;
		}
	}
	
	Ok(())
}

fn read_dir(fs: &mut Fs, path: &str) -> Result<Vec<FsEntry>> {
	let pinr = lookup(fs, path)?;
	let mut entries: Vec<FsEntry> = Vec::new();

	let _iter = fs.ufs.dir_iter(pinr, |name, inr, kind| {
		#[cfg(unix)]
		let bytes = name.as_bytes();

		#[cfg(windows)]
		let bytes = name.to_str()
    		.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")).expect("Invalid UTF-8")
    		.as_bytes();

		let name = String::from_utf8_lossy(bytes).to_string();
		if name != "." && name != ".." {
			entries.push(FsEntry { name, kind, inr });
		}

		if true {
			None
		} else {
			Some(())
		}
	});

	Ok(entries)
}