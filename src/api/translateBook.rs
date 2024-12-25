use std::collections::HashMap;
use leptos::server_fn::serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use Hconfig::serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateBook {
	_timestamp: i64,
	_datas: HashMap<String, String>
}

impl TranslateBook {
	pub fn new() -> Self {
		Self {
			_timestamp: 0,
			_datas: HashMap::new()
		}
	}

	pub fn get(&self, key: String) -> Option<&String>
	{
		return self._datas.get(&key);
	}

	#[cfg(feature = "ssr")]
	pub fn from(timestamp: i64, rootJson: &Value) -> Result<TranslateBook, &'static str>
	{
		let mut newdatas = HashMap::new();
		if(!rootJson.is_object())
		{
			return Err("root is not an object");
		}

		rootJson.as_object().unwrap().iter().for_each(|(key, value)| {
			newdatas.insert(key.to_string(), value.as_str().unwrap_or(key).to_string());
		});

		return Ok(Self {
			_timestamp: timestamp,
			_datas: newdatas
		});
	}
}