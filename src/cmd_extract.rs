use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use libheif_rs::{CompressionFormat, EncoderQuality, HeifContext, LibHeif};

use crate::metadata::structs::MetadataType;
use crate::metadata::utils::get_solar_metadata;

fn write_metadata_to_file(out_dir: &PathBuf, metadata: MetadataType) -> std::io::Result<()> {
	let json_path: PathBuf;
	let json_string: String;

	match metadata {
		MetadataType::Solar(solar) => {
			json_path = out_dir.join("solar.json");
			json_string = serde_json::to_string(&solar)?;
		}
		MetadataType::Appearance(ap) => {
			json_path = out_dir.join("ap.json");
			json_string = serde_json::to_string(&ap)?;
		}
		MetadataType::H24(h24) => {
			json_path = out_dir.join("h24.json");
			json_string = serde_json::to_string(&h24)?;
		}
	}

	// println!("{json_string}");

	let mut output = File::create(json_path)?;
	output.write(json_string.as_bytes())?;

	Ok(())
}

pub fn extract(arg_write_metadata: bool, arg_overwrite_images: bool,
               arg_lossy_quality: Option<u8>,
               arg_out_dir: Option<PathBuf>, arg_input: PathBuf) -> Result<(), String> {
	// make input path absolute
	let input = arg_input.canonicalize().expect("Can't canonicalize input path");

	// construct out_dir if none was given
	let input_stem = input.file_stem().unwrap().to_str().unwrap();
	let out_dir: PathBuf;
	if arg_out_dir.is_none() {
		let parent = input.parent().unwrap().to_path_buf();
		out_dir = parent.join(Path::new(input_stem));
	} else {
		out_dir = arg_out_dir.unwrap();
	}

	let image_ctx = HeifContext::read_from_file(input.to_str().unwrap()).expect("Can't load input image");
	let num_images = image_ctx.number_of_top_level_images();

	if num_images <= 1 {
		println!("{} only contains a single image.", input.file_name().unwrap().to_str().unwrap());
		return Ok(());
	}

	create_dir_all(out_dir.clone()).expect("Can't create output directory");

	// Extract metadata

	if arg_write_metadata {
		if let Some(metadata) = get_solar_metadata(&image_ctx) {
			write_metadata_to_file(&out_dir, metadata).expect("Can't write metadata to file");
		}
	}

	// Extract images

	let lib_heif = LibHeif::new();
	let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Hevc).expect("Can't create HEVC encoder");

	let encoder_quality: EncoderQuality;
	match arg_lossy_quality {
		None => { encoder_quality = EncoderQuality::LossLess; }
		Some(quality) => { encoder_quality = EncoderQuality::Lossy(quality); }
	}
	encoder.set_quality(encoder_quality).expect("Can't set encoder quality");

	for (i, handle) in image_ctx.top_level_image_handles().iter().enumerate() {
		let image_path = out_dir.join(format!("{} {}.heic", input_stem, i + 1));
		if image_path.exists() && !arg_overwrite_images {
			println!("Skipping image {}/{num_images}…", i + 1);
			continue
		}

		println!("Extracting image {}/{num_images} to {}…", i + 1, image_path.to_str().unwrap());

		let color_space = handle.preferred_decoding_colorspace().expect("Can't get color space");
		let mut image = lib_heif.decode(handle, color_space, None).expect("Can't decode image");

		let color_profile = handle.color_profile_raw();
		if color_profile.is_some() {
			image.set_color_profile_raw(&color_profile.unwrap()).expect("Can't set color profile");
		}

		let mut context = HeifContext::new().unwrap();
		context.encode_image(&image, &mut encoder, None).expect("Can't encode image");

		context.write_to_file(image_path.to_str().unwrap()).expect("Can't write image");
	}

	Ok(())
}