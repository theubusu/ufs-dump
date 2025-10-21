use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	/// Mount options to pass to the kernel
	#[arg(short, long, value_delimiter(','))]
	pub options: Vec<String>,

	/// Path to the device
	pub device:     PathBuf,
	/// Path to the mount point
	pub mountpoint: PathBuf,

	#[command(flatten)]
	pub verbose: Verbosity<WarnLevel>,

	/// Wait until the filesystem is unmounted.
	#[arg(short)]
	pub foreground: bool,
}