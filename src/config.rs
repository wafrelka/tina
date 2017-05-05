use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use csv::Reader;
use serde::{Deserializer, Deserialize};
use serde::de::Error;
use serde_yaml;
use serde_yaml::Value;
use slog::Level;


#[derive(Debug, Clone)]
pub enum ConfigLoadError {
	ConfigFileIo,
	CodeDictFileIO,
	InvalidCodeFormat,
	InvalidYamlFormat,
	InvalidKeyValue,
}

#[derive(Deserialize, Debug)]
struct RawRootConfig {
	pub path: DictPathConfig,
	pub wni: WNIConfig,
	pub twitter: Option<TwitterConfig>,
	pub log: LogConfig,
}

#[derive(Deserialize, Debug)]
struct DictPathConfig {
	pub area: String,
	pub epicenter: String,
}

#[derive(Deserialize, Debug)]
pub struct LogConfig {
	pub wni_log_path: Option<String>,
	pub eew_log_path: Option<String>,
	#[serde(default)] pub wni_stdout_log: bool,
	#[serde(default)] pub eew_stdout_log: bool,
	#[serde(deserialize_with = "deserialize_log_level")] pub log_level: Level,
}

#[derive(Deserialize, Debug)]
pub struct WNIConfig {
	pub id: String,
	pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct TwitterConfig {
	pub consumer_token: String,
	pub consumer_secret: String,
	pub access_token: String,
	pub access_secret: String,
	#[serde(default)] pub in_reply_to_enabled: bool,
}

#[derive(Debug)]
pub struct Config {
	pub area_dict: HashMap<[u8; 3], String>,
	pub epicenter_dict: HashMap<[u8; 3], String>,
	pub wni: WNIConfig,
	pub twitter: Option<TwitterConfig>,
	pub log: LogConfig,
}

fn deserialize_log_level<'d, D>(deserializer: D) -> Result<Level, D::Error>
	where D: Deserializer<'d>
{
	if let Ok(s) = String::deserialize(deserializer) {
		match s.as_str() {
			"critical" => Ok(Level::Critical),
			"error" => Ok(Level::Error),
			"warning" => Ok(Level::Warning),
			"info" => Ok(Level::Info),
			_ => Err(D::Error::custom("unknown log level"))
		}
	} else {
		Ok(Level::Info)
	}
}

fn load_code_dict(path: &str) -> Result<HashMap<[u8; 3], String>, ConfigLoadError>
{
	let mut reader = try!(Reader::from_file(path).map_err(|_| ConfigLoadError::CodeDictFileIO));

	let mut dict = HashMap::new();

	for record in reader.decode() {

		let (code, name): (String, String) = try!(record.map_err(|_| ConfigLoadError::InvalidCodeFormat));

		let mut encoded = [0; 3];
		let bytes = code.as_bytes();

		if bytes.len() != encoded.len() {
			return Err(ConfigLoadError::InvalidCodeFormat);
		}

		encoded.copy_from_slice(bytes);
		dict.insert(encoded, name);
	}

	return Ok(dict);
}

impl Config {

	pub fn load_config(path: &str) -> Result<Config, ConfigLoadError>
	{
		let mut file = try!(File::open(path).map_err(|_| ConfigLoadError::ConfigFileIo));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(|_| ConfigLoadError::ConfigFileIo));

		let raw_value: Value =
			try!(serde_yaml::from_str(&data).map_err(|_| ConfigLoadError::InvalidYamlFormat));
		let raw_root_conf: RawRootConfig =
			try!(serde_yaml::from_value(raw_value.clone()).map_err(|_| ConfigLoadError::InvalidKeyValue));

		let area_dict = try!(load_code_dict(&raw_root_conf.path.area));
		let epicenter_dict = try!(load_code_dict(&raw_root_conf.path.epicenter));

		let conf = Config {
			area_dict: area_dict,
			epicenter_dict: epicenter_dict,
			wni: raw_root_conf.wni,
			twitter: raw_root_conf.twitter,
			log: raw_root_conf.log,
		};

		Ok(conf)
	}
}
