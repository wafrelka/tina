extern crate yaml_rust;
extern crate csv;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use yaml_rust::YamlLoader;


#[derive(Debug)]
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
}

pub struct Config {
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

		let (code, name): (String, String) =
			try!(record.map_err(|_| ConfigLoadError::InvalidCodeFormat));

		let mut encoded = [0; 3];
		let bytes = code.as_bytes();

		if bytes.len() != encoded.len() {
			return Err(ConfigLoadError::InvalidCodeFormat);
		}

		for i in 0..(encoded.len()) {
			encoded[i] = bytes[i];
		}

		dict.insert(encoded, name);
	}

	return Ok(dict);
}

fn fetch_str<'a>(keys: &[&str], conf: &'a yaml_rust::Yaml) -> Option<&'a str>
{
	let mut elem = conf;
	for &k in keys {
		elem = &elem[k];
	}
	elem.as_str()
}

impl Config {

	pub fn load_config(path: &str) -> Result<Config, ConfigLoadError>
	{
		let mut file = try!(File::open(path).map_err(|_| ConfigLoadError::Io));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(|_| ConfigLoadError::Io));

		let docs = try!(YamlLoader::load_from_str(&data).map_err(|_| ConfigLoadError::Io));
		let conf = try!(docs.first().ok_or(ConfigLoadError::InvalidConfigFormat));

		let wni_id =
			try!(fetch_str(&["wni", "id"], conf).ok_or(ConfigLoadError::MissingRequiredKey));
		let wni_password =
			try!(fetch_str(&["wni", "password"], conf).ok_or(ConfigLoadError::MissingRequiredKey));

		let area_dict_path =
			try!(fetch_str(&["path", "area"], conf).ok_or(ConfigLoadError::MissingRequiredKey));
		let epicenter_dict_path =
			try!(fetch_str(&["path", "epicenter"], conf).ok_or(ConfigLoadError::MissingRequiredKey));

		let area_dict = try!(load_code_dict(area_dict_path));
		let epicenter_dict = try!(load_code_dict(epicenter_dict_path));

		let twitter_consumer_token = fetch_str(&["twitter", "consumer_token"], conf);
		let twitter_consumer_secret = fetch_str(&["twitter", "consumer_secret"], conf);
		let twitter_access_token = fetch_str(&["twitter", "access_token"], conf);
		let twitter_access_secret = fetch_str(&["twitter", "access_secret"], conf);

		let tw_full = {
			let v = [twitter_consumer_token, twitter_consumer_secret,
				twitter_access_token, twitter_consumer_secret];
			v.iter().all(|i| i.is_some())
		};

		let tw = match tw_full {
			true => Some(TwitterConfig {
				consumer_token: twitter_consumer_token.unwrap().to_string(),
				consumer_secret: twitter_consumer_secret.unwrap().to_string(),
				access_token: twitter_access_token.unwrap().to_string(),
				access_secret: twitter_access_secret.unwrap().to_string(),
			}),
			false => None
		};

		let c = Config {
			wni_id: wni_id.to_string(),
			wni_password: wni_password.to_string(),
			twitter: tw,
			area_dict: area_dict,
			epicenter_dict: epicenter_dict,
		};

		return Ok(c);
	}
}
