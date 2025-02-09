use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use fluent::bundle::FluentBundle;
use fluent::{FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use leptos::logging::log;
use crate::api::translateBooks::API_translate_getBook;

struct BookHolder
{
	content: FluentBundle<FluentResource, IntlLangMemoizer>,
	timstamp: u64
}

pub struct FluentManager {
	_resources: RwLock<HashMap<String, BookHolder>>
}

static SINGLETON: OnceLock<FluentManager> = OnceLock::new();

impl FluentManager {
	pub fn singleton() -> &'static FluentManager
	{
		return SINGLETON.get_or_init(|| FluentManager::new());
	}

	pub async fn translate(&self, lang: impl Into<String>, key: impl Into<String>, params: Arc<HashMap<String,String>>) -> String
	{
		let lang = lang.into();
		let key = key.into();
		if(!self._resources.read().unwrap().contains_key(&lang))
		{
			// TODO add a get into timestamp
			self.addResource(&lang,0).await;
		}

		let bindingMap = self._resources.read().unwrap();
		let Some(bundle) = bindingMap.get(&lang) else {
			log!("missing book");
			return key;
		};
		let Some(msg) = bundle.content.get_message(key.as_str()) else {
			log!("missing message for key {}",key);
			return key;
		};
		let Some(pattern) = msg.value() else {
			log!("missing pattern for key {}",key);
			return key;
		};
		let mut errors = vec![];

		let mut args = FluentArgs::new();
		params.iter().for_each(|(k,v)| {
			args.set(k, v);
		});

		let result = bundle.content.format_pattern(pattern, Some(&args), &mut errors);

		if(!errors.is_empty())
		{
			log!("Error while formatting fluent pattern: {:?}",errors);
		}

		return result.to_string();
	}

	//////// PRIVATE

	fn new() -> Self {
		Self {
			_resources: Default::default(),
		}
	}

	async fn addResource(&self, lang: &String, timestamp: u64)
	{
		let (content,newtime) = match API_translate_getBook(lang.clone(), timestamp).await
		{
			Ok(data) => {
				match data {
					None => return,
					Some(data) => data,
				}
			}
			Err(err) => {
				log!("err when return API_translate_getBook {}",err);
				return;
			}
		};

		let Ok(flt_res) = FluentResource::try_new(content) else {
			log!("Failed to parse an FTL string.");
			return;
		};

		let mut bindingMap = self._resources.write().unwrap();
		match bindingMap.get_mut(lang)
		{
			Some(bundle) => {
				bundle.content.add_resource_overriding(flt_res);
			},
			None => {
				let Ok(langid) = lang.parse() else {
					log!("failed to parse lang ID");
					return;
				};
				let mut bundle = FluentBundle::new_concurrent(vec![langid]);

				bundle.add_resource_overriding(flt_res);

				bindingMap.insert(lang.clone(), BookHolder {
					content: bundle,
					timstamp: newtime,
				});
			}
		}
	}
}