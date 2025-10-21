use std::{
	fs::{self, OpenOptions},
	io::{Write, Result},
	path::PathBuf,
	os::unix::ffi::OsStrExt,
	path::Path,
};

use rufs::{InodeNum, InodeType, InodeAttr};
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
	let path = PathBuf::from("/");
	run_tree(&mut fs, &path, 0, out_path)?;
	Ok(())
}

fn run_tree(fs: &mut Fs, path: &Path, depth: usize, out_path: &PathBuf) -> Result<()> {
	let entries = read_dir(fs, path)?;
	
	for entry in entries {
		let full_path = path.join(&entry.name);
		let save_path = out_path.join(&full_path.strip_prefix("/").unwrap());

		if entry.kind == InodeType::Symlink {
			let link = fs.ufs.symlink_read(entry.inr)?;
			let dest = String::from_utf8_lossy(&link).to_string();
			println!("{} : Symlink to {}", full_path.display(), dest);
		}

		if entry.kind == InodeType::Directory {
			println!("{} : Directory", full_path.display());
			fs::create_dir_all(&save_path)?;

			let child_path = path.join(&entry.name);
			run_tree(fs, &child_path, depth + 1, out_path)?;
		}

		if entry.kind == InodeType::RegularFile {
			let attr = getattr(fs, entry.inr)?;
			println!("{} : Regular File, Size: {}", full_path.display(), attr.size);

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