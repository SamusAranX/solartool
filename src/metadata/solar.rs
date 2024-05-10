use libheif_rs::HeifContext;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn get_solar_metadata(ctx: HeifContext) {
	// TODO: this. also remove the allows above when this is done
	let mut mime_id = vec![0; 1];
	let handle = ctx.primary_image_handle().unwrap();
	handle.metadata_block_ids(&mut mime_id, b"mime");

	let metadata = handle.metadata(mime_id[0]).expect("Can't get MIME metadata");
	let xml = String::from_utf8(metadata).expect("Can't decode metadata");
}