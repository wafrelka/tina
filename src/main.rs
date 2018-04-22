#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate csv;
extern crate tina;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate slog_term;

mod config;

use std::io::stdout;
use std::env;
use std::fs::OpenOptions;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::thread::{spawn, JoinHandle};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use slog::{Drain, Logger, Discard, Duplicate};
use slog_scope::set_global_logger;
use slog_term::{PlainSyncDecorator, FullFormat};

use tina::*;
use config::*;

const REVISION: &'static str = env!("TINA_REVISION");
const CONF_PATH_ENV_VAR: &'static str = "TINA_CONF_PATH";
const DEFAULT_CONFIG_PATH: &'static str = "config/tina.yaml";
const SERVER_LIST_URL: &'static str = "http://lst10s-sp.wni.co.jp/server_list.txt";
const WNI_THREAD_COUNT: u32 = 4;

const EEW_HISTORY_CAPACITY: usize = 128;

fn build_specific_logger(log_path: &Option<String>, duplication: bool, default: &Logger) -> Logger
{
	if let Some(ref path) = *log_path {

		let file = OpenOptions::new().append(true).create(true).open(path).unwrap();
		let drain = FullFormat::new(PlainSyncDecorator::new(file)).build();

		if duplication {
			Logger::root(Duplicate(drain, default.clone()).fuse(), o!())
		} else {
			Logger::root(drain.fuse(), o!())
		}

	} else {

		if duplication { default.clone() } else { Logger::root(Discard, o!()) }
	}
}

fn spawn_conn_thread(thread_num: u32, wni: Arc<Mutex<Wni>>,
	epicenter_dict: HashMap<[u8; 3], String>, area_dict: HashMap<[u8; 3], String>,
	sock: SyncSender<EEW>) -> JoinHandle<()>
{
	spawn(move || {

		let mut moderator = Moderator::new();
		let wni = wni;

		loop {

			let mut connection = match wni.lock().unwrap().connect() {
				Ok(v) => v,
				Err(e) => {
					error!("Thread {} - ConnectionError: {:?}", thread_num, e);
					moderator.wait_for_retry();
					moderator.add_count();
					continue;
				}
			};

			moderator.reset();
			info!("Thread {} - Connected: WNI ({})", thread_num, connection.server());

			loop {

				let eew = match connection.wait_for_telegram(&epicenter_dict, &area_dict) {
					Err(e) => {
						error!("Thread {} - StreamingError: {:?}", thread_num, e);
						break;
					},
					Ok(eew) => eew
				};

				sock.try_send(eew).expect("should not fail");
			}
		}
	})
}

fn main()
{
	let cmd_args: Vec<String> = env::args().collect();

	match cmd_args.get(1).map(|s| s.as_str()) {
		Some("-v") | Some("--version") => {
			eprintln!("Tina - EEW Client (rev.{})", REVISION);
			return;
		},
		_ => {}
	}

	let conf_path_arg = cmd_args.get(1).map(|s| s.as_str());
	let conf_path_env_owned = env::var(CONF_PATH_ENV_VAR).ok();
	let conf_path_env = conf_path_env_owned.as_ref().map(|s| s.as_str());
	let conf_path = conf_path_arg.or(conf_path_env).unwrap_or(DEFAULT_CONFIG_PATH);

	let conf = match Config::load_config(conf_path) {
		Err(err) => {
			println!("Error while loading config from '{}' ({:?})", conf_path, err);
			return;
		},
		Ok(c) => c
	};

	let stdout_drain = FullFormat::new(PlainSyncDecorator::new(stdout())).build();
	let stdout_logger = Logger::root(stdout_drain.fuse(), o!());

	let root_drain = stdout_logger.clone().filter_level(conf.log.log_level);
	let root_logger = Logger::root(root_drain.fuse(), o!());
	set_global_logger(root_logger).cancel_reset();

	let eew_logger = build_specific_logger(&conf.log.eew_log_path, conf.log.eew_stdout_log, &stdout_logger);
	let wni_logger = build_specific_logger(&conf.log.wni_log_path, conf.log.wni_stdout_log, &stdout_logger);

	let raw_wni = Wni::new(
		conf.wni.id.clone(),
		"40285072".to_owned(),
		conf.wni.password.clone(),
		SERVER_LIST_URL.to_owned(),
		Some(wni_logger)
	);
	let wni = Arc::new(Mutex::new(raw_wni));
	let mut socks: Vec<Box<Routing>> = Vec::new();

	socks.push(Box::new(Router::new(Logging::new(eew_logger), TRUE_CONDITION, "Log")));
	info!("Enabled: EEW Logging");

	if let Some(ref t) = conf.twitter.as_ref() {
		let tw = Twitter::new(
			t.consumer_token.clone(), t.consumer_secret.clone(),
			t.access_token.clone(), t.access_secret.clone(), t.in_reply_to_enabled, t.updown_enabled);
		if ! tw.is_valid() {
			warn!("Twitter: Invalid tokens");
		} else {
			match t.cond {
				Some(ref v) => socks.push(Box::new(Router::new(tw, build_yaml_condition(v.clone()), "Twitter"))),
				None => socks.push(Box::new(Router::new(tw, TRUE_CONDITION, "Twitter"))),
			}
			info!("Enabled: Twitter");
		}
	}

	if let Some(ref s) = conf.slack.as_ref() {
		match Slack::build(&s.webhook_url, s.updown_enabled) {
			Ok(sl) => {
				match s.cond {
					Some(ref v) => socks.push(Box::new(Router::new(sl, build_yaml_condition(v.clone()), "Slack"))),
					None => socks.push(Box::new(Router::new(sl, TRUE_CONDITION, "Slack"))),
				}
				info!("Enabled: Slack");
			},
			Err(_) => {
				warn!("Slack: Invalid webhook url");
			}
		}
	}

	let mut conn_threads = Vec::new();
	let (eew_tx, eew_rx) = sync_channel(32);

	for thread_num in 0..WNI_THREAD_COUNT {

		let t = spawn_conn_thread(thread_num, wni.clone(),
			conf.epicenter_dict.clone(), conf.area_dict.clone(), eew_tx.clone());
		conn_threads.push(t);
	}

	let mut his = EEWHistory::new(EEW_HISTORY_CAPACITY);

	loop {
		let eew = eew_rx.recv().unwrap();
		if let Some(eew) = his.append(eew) {
			for s in socks.iter_mut() {
				s.emit(&eew);
			}
		}
	}
}
