extern crate yaml_rust;
extern crate csv;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use yaml_rust::YamlLoader;


#[derive(Debug, Clone)]
pub enum ConfigLoadError {
	Io,
	InvalidCodeFormat,
	InvalidConfigFormat,
	MissingRequiredKey,
}

pub struct TwitterConfig {
	pub consumer_token: String,
	pub consumer_secret: String,
	pub access_token: String,
	pub access_secret: String,
	pub in_reply_to_enabled: bool,
}

pub struct Config {
	pub log4rs_conf_path: Option<String>,
	pub area_dict: HashMap<[u8; 3], String>,
	pub epicenter_dict: HashMap<[u8; 3], String>,
	pub wni_id: String,
	pub wni_password: String,
	pub twitter: Option<TwitterConfig>,
}

fn load_code_dict(path: &str) -> Result<HashMap<[u8; 3], String>, ConfigLoadError>
{
	let mut reader = try!(csv::Reader::from_file(path).map_err(|_| ConfigLoadError::Io));

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
		let mut file = try!(File::open(path).map_err(|_| ConfigLoadError::Io));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(|_| ConfigLoadError::Io));

		let docs = try!(YamlLoader::load_from_str(&data).map_err(|_| ConfigLoadError::Io));
		let conf = try!(docs.first().ok_or(ConfigLoadError::InvalidConfigFormat));

		let wni_conf = &conf["wni"];
		let wni_id = try!(wni_conf["id"].as_str().ok_or(ConfigLoadError::MissingRequiredKey));
		let wni_password = try!(wni_conf["password"].as_str().ok_or(ConfigLoadError::MissingRequiredKey));

		let path_conf = &conf["path"];
		let area_dict_path = try!(path_conf["area"].as_str().ok_or(ConfigLoadError::MissingRequiredKey));
		let epicenter_dict_path =
			try!(path_conf["epicenter"].as_str().ok_or(ConfigLoadError::MissingRequiredKey));
		let log4rs_conf_path = path_conf["log4rs"].as_str().map(|s| s.to_string());

		let area_dict = try!(load_code_dict(area_dict_path));
		let epicenter_dict = try!(load_code_dict(epicenter_dict_path));

		let tw_conf = &conf["twitter"];
		let twitter_consumer_token = tw_conf["consumer_token"].as_str();
		let twitter_consumer_secret = tw_conf["consumer_secret"].as_str();
		let twitter_access_token = tw_conf["access_token"].as_str();
		let twitter_access_secret = tw_conf["access_secret"].as_str();
		let in_reply_to_enabled = tw_conf["in_reply_to_enabled"].as_bool().unwrap_or(false);

		let tw_full = {
			let v = [twitter_consumer_token, twitter_consumer_secret,
				twitter_access_token, twitter_consumer_secret];
			v.iter().all(|i| i.is_some())
		};

		let tw = match tw_full {
			true => Some(TwitterConfig {
				consumer_token: twitter_consumer_token.expect("").to_string(),
				consumer_secret: twitter_consumer_secret.expect("").to_string(),
				access_token: twitter_access_token.expect("").to_string(),
				access_secret: twitter_access_secret.expect("").to_string(),
				in_reply_to_enabled: in_reply_to_enabled,
			}),
			false => None
		};

		let c = Config {
			log4rs_conf_path: log4rs_conf_path,
			wni_id: wni_id.to_string(),
			wni_password: wni_password.to_string(),
			twitter: tw,
			area_dict: area_dict,
			epicenter_dict: epicenter_dict,
		};

		return Ok(c);
	}
}
