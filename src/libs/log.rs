use chrono::{
	DateTime,
	Datelike,
	Local,
	TimeZone,
	Utc,
};
use colored::Colorize;
use std::panic;

#[track_caller]
pub fn log_info(msg: &str) {
	let utc_time: DateTime<Utc> = Utc::now();
	let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

	let log = format!(
		"[{}-{}-{} {} INFO {}]: {msg}",
		local_time.month(),
		local_time.day(),
		utc_time.year(),
		local_time.time(),
		panic::Location::caller()
	);
	write_log(log);
}

#[track_caller]
pub fn log_error(msg: &str) {
	let utc_time: DateTime<Utc> = Utc::now();
	let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

	let log = format!(
		"[{}-{}-{} {} ERROR {}]: {msg}",
		local_time.month(),
		local_time.day(),
		utc_time.year(),
		local_time.time(),
		panic::Location::caller()
	);
	write_log(log);
}

#[track_caller]
pub fn log_warn(msg: &str) {
	let utc_time: DateTime<Utc> = Utc::now();
	let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

	let log = format!(
		"[{}-{}-{} {} WARN {}]: {msg}",
		local_time.month(),
		local_time.day(),
		utc_time.year(),
		local_time.time(),
		panic::Location::caller()
	);
	write_log(log);
}

#[track_caller]
pub fn log_debug(msg: &str) {
	let utc_time: DateTime<Utc> = Utc::now();
	let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

	let log = format!(
		"[{}-{}-{} {} DEBUG {}]: {msg}",
		local_time.month(),
		local_time.day(),
		utc_time.year(),
		local_time.time(),
		panic::Location::caller()
	);
	write_log(log);
}

pub fn write_log(text: String) {
	println!("{}", text);
}
