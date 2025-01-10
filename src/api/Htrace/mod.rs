use leptos::prelude::ServerFnError;
use leptos::server;

#[cfg(feature = "ssr")]
use Htrace::Type::Type as Htype;
#[cfg(feature = "ssr")]
use Htrace::HTracer::HTracer;
use serde::{Deserialize, Serialize};

#[server]
pub async fn API_Htrace_log( content: String, htype: Type, file: String, line: u32) -> Result<(), ServerFnError>
{
	HTracer::log(&content, htype.to_Htype(), file.as_str(), line);
	return Ok(());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type
{
	DEBUG,
	NORMAL,
	NOTICE,
	NOTICEDERR,
	WARNING,
	DEBUGERR,
	ERROR,
	FATAL,
}

#[cfg(feature = "ssr")]
impl Type
{
	pub fn to_Htype(&self) -> Htype
	{
		match self {
			Type::DEBUG => Htype::DEBUG,
			Type::NORMAL => Htype::NORMAL,
			Type::NOTICE => Htype::NOTICE,
			Type::NOTICEDERR => Htype::NOTICEDERR,
			Type::WARNING => Htype::WARNING,
			Type::DEBUGERR => Htype::DEBUGERR,
			Type::ERROR => Htype::ERROR,
			Type::FATAL => Htype::FATAL,
		}
	}
}