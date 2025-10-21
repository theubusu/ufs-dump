use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	/// Path to the file/device
	pub file: PathBuf,

	/// Output path
	pub out_path: PathBuf,

	/// Superblock at 8192
    #[arg(short = 'p')]
    pub old_sblock: bool,

	#[command(flatten)]
	pub verbose: Verbosity<WarnLevel>,
}