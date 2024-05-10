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
#[command(version = CLAP_VERSION, about = "Derives an image with alpha channel from two alpha-less images")]
struct Args {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Query {
		#[arg(help = "The input .heic file")]
		input: PathBuf,
	},
	Extract {
		#[arg(short = 'W', long, help = "Whether to write solar metadata to the output directory")]
		write_metadata: bool,

		#[arg(short, long, help = "The directory to write split .heic files to [default: the input file's name minus its extension]")]
		out_dir: Option<PathBuf>,

		#[arg(help = "The input .heic file")]
		input: PathBuf,
	},
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
		Commands::Extract { write_metadata, out_dir, input } => {
			extract(write_metadata, out_dir, input)
		}
		Commands::Create {} => {
			unimplemented!()
		}
	}
}
