use std::path::PathBuf;

use clap::{Parser, Subcommand};
use const_format::formatcp;

use crate::cmd_extract::extract;
use crate::cmd_query::query;

mod metadata;
mod cmd_query;
mod cmd_extract;

const GIT_HASH: &str = env!("GIT_HASH");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const GIT_VERSION: &str = env!("GIT_VERSION");
const BUILD_DATE: &str = env!("BUILD_DATE");

const CLAP_VERSION: &str = formatcp!("{GIT_VERSION} [{GIT_BRANCH}, {GIT_HASH}, {BUILD_DATE}]");

#[derive(Parser, Debug)]
#[command(version = CLAP_VERSION, about = "A commandline tool for working with \"dynamic wallpaper\" HEIC images")]
struct Args {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	#[command(about = "Prints the number of subimages and exits")]
	Query {
		#[arg(help = "The input .heic file")]
		input: PathBuf,
	},

	#[command(about = "Extracts images and metadata from a dynamic wallpaper HEIC file")]
	Extract {
		#[arg(short = 'W', long, help = "Whether to write solar metadata to the output directory")]
		write_metadata: bool,

		#[arg(short = 'O', long, help = "Whether to overwrite images")]
		overwrite_images: bool,

		#[arg(short, long, help = "Specify a value in the range of 0–100 to use lossy, rather than lossless, encoding for output images")]
		lossy_quality: Option<u8>,

		#[arg(short, long, help = "The directory to write split .heic files to [default: the input file's name minus its extension]")]
		out_dir: Option<PathBuf>,

		#[arg(help = "The input .heic file")]
		input: PathBuf,
	},

	#[command(about = "[Not yet implemented] Assemble a dynamic wallpaper from metadata and a number of images")]
	Create {
		// TODO
	}
}

fn main() -> Result<(), String> {
	let args = Args::parse();
	match args.command {
		Commands::Query { input } => {
			query(input)
		}
		Commands::Extract { write_metadata, overwrite_images, lossy_quality, out_dir, input } => {
			extract(write_metadata, overwrite_images, lossy_quality, out_dir, input)
		}
		Commands::Create {} => {
			unimplemented!()
		}
	}
}
