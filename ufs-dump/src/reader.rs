use std::{
	io::Result,
	path::PathBuf,
	os::unix::ffi::OsStrExt,
	path::Path,
};

use rufs::{InodeNum, InodeType};
use crate::{Fs};

fn lookup(fs: &mut Fs, path: &Path) -> Result<InodeNum> {
	if !path.is_absolute() {
		return Err(err!(EINVAL));
	}

	let mut inr = InodeNum::ROOT;
	for comp in path.components().skip(1) {
		inr = fs.ufs.dir_lookup(inr, comp.as_os_str())?;
	}
	Ok(inr)
}

struct FsEntry {
	name: String,
	kind: InodeType,
	inr: InodeNum,
}

pub fn run(mut fs: Fs) -> Result<()> {
	let path = PathBuf::from("/");
	run_tree(&mut fs, &path, 0)?;
	Ok(())
}

fn run_tree(fs: &mut Fs, path: &Path, depth: usize) -> Result<()> {
	let entries = read_dir(fs, path)?;
	
	for entry in entries {
		let full_path = path.join(&entry.name);

		if entry.kind == InodeType::Symlink {
			let link = fs.ufs.symlink_read(entry.inr)?;
			let dest = String::from_utf8_lossy(&link).to_string();
			println!("{} : Symlink to {}", full_path.display(), dest);
		}

		if entry.kind == InodeType::Directory {
			println!("{} : Directory", full_path.display());
			let child_path = path.join(&entry.name);
			run_tree(fs, &child_path, depth + 1)?;
		}

		if entry.kind == InodeType::RegularFile {
			println!("{} : Regular File", full_path.display());
		}
	
	}
	
	Ok(())
}

fn read_dir(fs: &mut Fs, path: &Path) -> Result<Vec<FsEntry>> {
	let pinr = lookup(fs, path)?;
	let mut entries: Vec<FsEntry> = Vec::new();

	let _iter = fs.ufs.dir_iter(pinr, |name, inr, kind| {
		let name = String::from_utf8_lossy(name.as_bytes()).to_string();
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