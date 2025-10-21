use std::fs::File;

use anyhow::Result;
use clap::Parser;
use rufs::Ufs;

use crate::cli::Cli;

macro_rules! err {
	($n:ident) => {
		std::io::Error::from_raw_os_error(libc::$n)
	};
}

mod cli;

mod reader;

struct Fs {
	ufs: Ufs<File>,
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	env_logger::builder()
		.filter_level(cli.verbose.log_level_filter())
		.init();

	let fs = Fs {
		ufs: Ufs::open(&cli.device, false)?,
	};

	reader::run(fs, &cli.out_path)?;

	println!("\nDone!");

	Ok(())
}
