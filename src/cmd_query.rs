use std::path::PathBuf;

use libheif_rs::HeifContext;

pub fn query(arg_input: PathBuf) -> Result<(), String> {
	// make input path absolute
	let input = arg_input.canonicalize().expect("Can't canonicalize input path");
	let image_ctx = HeifContext::read_from_file(input.to_str().unwrap()).expect("Can't load input image");
	let num_images = image_ctx.number_of_top_level_images();

	println!("{num_images}");

	Ok(())
}