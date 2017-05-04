use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use csv;
use serde_yaml;
use serde_yaml::Value;
use log::LogLevelFilter;

use tina::LogConfig;


#[derive(Debug, Clone)]
pub enum ConfigLoadError {
	ConfigFileIo,
	CodeDictFileIO,
	InvalidCodeFormat,
	InvalidConfigFormat,
	MissingRequiredKey,
}

#[derive(Deserialize, Debug)]
struct RawRootConfig {
	pub path: DictPathConfig,
	pub wni: WNIConfig,
	pub twitter: Option<TwitterConfig>,
}

#[derive(Deserialize, Debug)]
struct DictPathConfig {
	pub area: String,
	pub epicenter: String,
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
	pub in_reply_to_enabled: bool,
}

#[derive(Debug)]
pub struct Config {
	pub area_dict: HashMap<[u8; 3], String>,
	pub epicenter_dict: HashMap<[u8; 3], String>,
	pub wni: WNIConfig,
	pub twitter: Option<TwitterConfig>,
	pub log_config: LogConfig,
}

fn load_code_dict(path: &str) -> Result<HashMap<[u8; 3], String>, ConfigLoadError>
{
	let mut reader = try!(csv::Reader::from_file(path).map_err(|_| ConfigLoadError::CodeDictFileIO));

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
			try!(serde_yaml::from_str(&data).map_err(|_| ConfigLoadError::InvalidConfigFormat));
		let raw_root_conf: RawRootConfig =
			try!(serde_yaml::from_value(raw_value.clone()).map_err(|_| ConfigLoadError::MissingRequiredKey));

		let area_dict = try!(load_code_dict(&raw_root_conf.path.area));
		let epicenter_dict = try!(load_code_dict(&raw_root_conf.path.epicenter));

		// FIXME: load config
		let log_conf = LogConfig {
			wni_log_path: None,
			wni_log_console: false,
			eew_log_path: None,
			eew_log_console: false,
			main_log_level: LogLevelFilter::Off
		};

		let conf = Config {
			area_dict: area_dict,
			epicenter_dict: epicenter_dict,
			wni: raw_root_conf.wni,
			twitter: raw_root_conf.twitter,
			log_config: log_conf,
		};

		Ok(conf)
	}
}
