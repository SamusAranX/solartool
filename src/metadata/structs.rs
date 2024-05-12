use serde::{Deserialize, Serialize};

pub const PROPERTY_SOLAR: &str = "apple_desktop:solar";
pub const PROPERTY_APR: &str = "apple_desktop:apr";
pub const PROPERTY_H24: &str = "apple_desktop:h24";

pub const PROPERTIES: &[&str] = &[PROPERTY_SOLAR, PROPERTY_APR, PROPERTY_H24];

#[derive(Deserialize, Serialize, Debug)]
pub enum MetadataType {
	Solar(SolarMetadata),
	Appearance(AppearanceMetadata),
	H24(H24Metadata),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SolarMetadata {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "ap")]
	appearance: Option<AppearanceMetadata>,

	#[serde(rename = "si")]
	images: Vec<SolarImage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppearanceMetadata {
	/// The index of the Dark Mode subimage
	#[serde(rename = "d")]
	dark: usize,

	/// The index of the Light Mode subimage
	#[serde(rename = "l")]
	light: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct H24Metadata {
	#[serde(rename = "ti")]
	pub images: Vec<TimeImage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SolarImage {
	/// The sun's altitude in degrees from -90 to 90
	#[serde(rename = "a")]
	pub altitude: f32,

	/// The sun's azimuth in degrees from 0 to 360
	#[serde(rename = "z")]
	pub azimuth: f32,

	/// The index of the relevant subimage
	#[serde(rename = "i")]
	pub index: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeImage {
	/// The time of day as a number from 0.0 to 1.0 (meant to be multiplied by 24)
	#[serde(rename = "t")]
	pub time: f32,

	/// The index of the relevant subimage
	#[serde(rename = "i")]
	pub index: usize,
}