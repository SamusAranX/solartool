use std::fmt::format;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use clap::Parser;
use const_format::formatcp;
use libheif_rs::{ColorSpace, CompressionFormat, Encoder, EncoderQuality, HeifContext, ItemId, LibHeif, RgbChroma};
use crate::metadata::solar::get_solar_metadata;

mod metadata;

const GIT_HASH: &str = env!("GIT_HASH");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const GIT_VERSION: &str = env!("GIT_VERSION");
const BUILD_DATE: &str = env!("BUILD_DATE");

const CLAP_VERSION: &str = formatcp!("{GIT_VERSION} [{GIT_BRANCH}, {GIT_HASH}, {BUILD_DATE}]");

#[derive(Parser, Debug)]
#[command(version = CLAP_VERSION, about = "Derives an image with alpha channel from two alpha-less images")]
struct Args {
	#[arg(short = 'W', long, help = "Whether to write solar metadata to the output directory")]
	write_metadata: bool,

	#[arg(short, long, help = "The directory to write split .heic files to [default: the input file's name minus its extension]")]
	out_dir: Option<PathBuf>,

	#[arg(help = "The input .heic file")]
	input: PathBuf,
}

fn main() -> Result<(), String> {
	let mut args = Args::parse();

	// make input path absolute
	args.input = args.input.canonicalize().expect("Can't canonicalize input path");

	// construct out_dir if none was given
	let input_stem = args.input.file_stem().unwrap();
	let out_dir: PathBuf;
	if args.out_dir.is_none() {
		let parent = args.input.parent().unwrap().to_path_buf();
		out_dir = parent.join(Path::new(input_stem));
	} else {
		out_dir = args.out_dir.unwrap();
	}

	let metadata_file = out_dir.join("solar.json");

	let image_ctx = HeifContext::read_from_file(args.input.to_str().unwrap()).expect("Can't load input image");
	let num_images = image_ctx.number_of_top_level_images();

	println!("Number of contained images: {num_images}");
	if num_images <= 1 {
		println!("The input file only contains a single image. No work needs to be done.");
		return Ok(());
	}

	create_dir_all(out_dir.clone()).expect("Can't create output directory");

	let lib_heif = LibHeif::new();
	let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Hevc).expect("encoder_for_format");
	encoder.set_quality(EncoderQuality::LossLess).expect("set_quality");

	for (i, handle) in image_ctx.top_level_image_handles().iter().enumerate() {
		let image_path = out_dir.join(format!("{}.heic", i+1));
		println!("Extracting image {} to {}…", i+1, image_path.to_str().unwrap());

		let color_space = handle.preferred_decoding_colorspace().expect("Can't get color space");
		let color_profile = handle.color_profile_raw().expect("Can't get color profile");

		let mut image = lib_heif.decode(handle, color_space, None).expect("Can't decode image");
		image.set_color_profile_raw(&color_profile).expect("Can't set color profile");

		let mut context = HeifContext::new().unwrap();
		context.encode_image(&image, &mut encoder, None).expect("Can't encode image");

		context.write_to_file(image_path.to_str().unwrap()).expect("Can't write image");
	}

	// let metadata = get_solar_metadata(image_ctx);
	// println!("metadata: {}", metadata);

	println!("All images extracted.");


	Ok(())
}
