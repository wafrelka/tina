use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use log::LogLevelFilter;


const WNI_MOD_PATH: &'static str = "tina::source::wni_client";
const EEW_MOD_PATH: &'static str = "tina::destination::logging_wrapper";
const TINA_ROOT_MOD_PATH: &'static str = "tina";
const FORMAT: &'static str = "[{d(%Y/%m/%d-%H:%M:%S)(utc)}][{l}\\({f}\\)] {m}{n}";

pub struct LogConfig {
	pub wni_log_path: Option<String>,
	pub wni_log_console: bool,
	pub eew_log_path: Option<String>,
	pub eew_log_console: bool,
	pub main_log_level: LogLevelFilter
}

fn register_inspection_point(file_path: Option<String>, to_root: bool, mod_path: String,
	level: LogLevelFilter, appenders: &mut Vec<Appender>, loggers: &mut Vec<Logger>)
	-> Result<(), ()>
{
	if let Some(path) = file_path {

		let name = mod_path.clone();
		let encoder = Box::new(PatternEncoder::new(FORMAT));
		let recorder = try!(FileAppender::builder().encoder(encoder).build(path).map_err(|_| ()));
		let appender = Appender::builder().build(name.clone(), Box::new(recorder));
		let logger = Logger::builder().appender(name).additive(to_root).build(mod_path, level);
		appenders.push(appender);
		loggers.push(logger);

	} else if !to_root {

		let logger = Logger::builder().additive(false).build(mod_path, LogLevelFilter::Off);
		loggers.push(logger);
	}

	Ok(())
}

pub fn setup_logging(conf: LogConfig) -> Result<(), ()>
{
	let mut appenders = Vec::new();
	let mut loggers = Vec::new();

	try!(register_inspection_point(conf.wni_log_path, conf.wni_log_console,
		WNI_MOD_PATH.to_string(), LogLevelFilter::Debug, &mut appenders, &mut loggers));
	try!(register_inspection_point(conf.eew_log_path, conf.eew_log_console,
		EEW_MOD_PATH.to_string(), LogLevelFilter::Info, &mut appenders, &mut loggers));

	let con_encoder = Box::new(PatternEncoder::new(FORMAT));
	let con_recorder = ConsoleAppender::builder().encoder(con_encoder).build();
	appenders.push(Appender::builder().build("con".to_string(), Box::new(con_recorder)));

	let root = Root::builder().build(LogLevelFilter::Off);
	let tina_root_logger =
		Logger::builder().appender("con".to_string()).additive(false)
		.build(TINA_ROOT_MOD_PATH.to_string(), conf.main_log_level);
	loggers.push(tina_root_logger);

	let log4rs_conf =
		try!(Config::builder().appenders(appenders).loggers(loggers).build(root).map_err(|_| ()));

	let handle = init_config(log4rs_conf);

	handle.map_err(|_| ()).map(|_| ())
}
