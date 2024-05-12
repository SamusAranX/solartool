use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use libheif_rs::HeifContext;
use xmp_toolkit::{FromStrOptions, XmpMeta};

use crate::metadata::structs::*;

const NAMESPACE_APPLE: &str = "http://ns.apple.com/namespace/1.0/";

pub fn get_solar_metadata(ctx: &HeifContext) -> Option<MetadataType> {
	let mut mime_id = vec![0; 1];
	let handle = ctx.primary_image_handle().unwrap();
	handle.metadata_block_ids(&mut mime_id, b"mime");

	let metadata = handle.metadata(mime_id[0]).expect("Can't get MIME metadata");
	let xml = String::from_utf8(metadata).expect("Can't decode metadata");
	let xmp = XmpMeta::from_str_with_options(xml.as_str(), FromStrOptions::default()).unwrap();

	for &path in PROPERTIES {
		if let Some(value) = xmp.property(NAMESPACE_APPLE, path) {
			let decoded = BASE64_STANDARD.decode(value.value).expect("Can't decode metadata");

			return match path {
				PROPERTY_SOLAR => {
					let m: SolarMetadata = plist::from_bytes(&*decoded).expect("Corrupted solar metadata found");
					Some(MetadataType::Solar(m))
				}
				PROPERTY_APR => {
					let m: AppearanceMetadata = plist::from_bytes(&*decoded).expect("Corrupted appearance metadata found");
					Some(MetadataType::Appearance(m))
				}
				PROPERTY_H24 => {
					let m: H24Metadata = plist::from_bytes(&*decoded).expect("Corrupted h24 metadata found");
					Some(MetadataType::H24(m))
				}
				_ => { continue }
			}
		}
	}

	None
}