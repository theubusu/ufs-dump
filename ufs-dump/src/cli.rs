use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	/// Path to the device/file
	pub device: PathBuf,

	/// Output path
	pub out_path: PathBuf,

	#[command(flatten)]
	pub verbose: Verbosity<WarnLevel>,
}