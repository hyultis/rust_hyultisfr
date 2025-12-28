use std::sync::atomic::AtomicBool;
use std::sync::OnceLock;

pub mod translateBooks;
pub mod Htrace;
//pub mod ApiError;


pub static IS_TRACE_FRONT_LOG: OnceLock<AtomicBool> = OnceLock::new();