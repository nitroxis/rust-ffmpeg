use std::cell::RefCell;

use libc::{c_char, c_int, c_void};
use log;
use vsprintf::vsprintf;

use crate::ffi::*;

pub fn set_level(level: log::Level) {
	unsafe {
		av_log_set_level(match level {
			log::Level::Error => AV_LOG_ERROR,
			log::Level::Warn => AV_LOG_WARNING,
			log::Level::Info => AV_LOG_INFO,
			log::Level::Debug => AV_LOG_DEBUG,
			log::Level::Trace => AV_LOG_TRACE,
		});
	}
}

thread_local! {
	static LOG_BUF: RefCell<String> = RefCell::new(String::new());
}

unsafe extern "C" fn callback(_ptr: *mut c_void, level: c_int, fmt: *const c_char, args: *mut __va_list_tag) {
	let string = vsprintf(fmt, args).unwrap();
	let level = match level {
		AV_LOG_PANIC | AV_LOG_FATAL | AV_LOG_ERROR => log::LevelFilter::Error,
		AV_LOG_WARNING => log::LevelFilter::Warn,
		AV_LOG_INFO => log::LevelFilter::Info,
		AV_LOG_VERBOSE | AV_LOG_DEBUG => log::LevelFilter::Debug,
		AV_LOG_TRACE => log::LevelFilter::Trace,
		_ => log::LevelFilter::Off,
	};

	if let Some(level) = level.to_level() {
		LOG_BUF.with(|buf| {
			let mut split = string.split('\n').peekable();

			while let Some(str) = split.next() {
				if split.peek().is_none() {
					// Last item without \n at the end, append to buffer.
					*buf.borrow_mut() += str;
				}
				else {
					// Prepend buffered string.
					let mut buffered = buf.borrow_mut();
					log::log!(target: "ffmpeg", level, "{buffered}{str}");
					buffered.clear();
				}
			}
		});
	}
}

pub fn register() {
	unsafe {
		av_log_set_callback(Some(callback));
	}

	if let Some(level) = log::max_level().to_level() {
		set_level(level);
	}
}
