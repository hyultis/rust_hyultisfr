use std::sync::OnceLock;
use dashmap::DashMap;
use Hconfig::HConfigManager::HConfigManager;
use Htrace::{HTrace, HTraceError};
use Htrace::Type::Type;
use crate::api::translateBook::TranslateBook;

pub struct TranslateManager
{
	_translateTime: DashMap<String, i64>
}

static SINGLETON: OnceLock<TranslateManager> = OnceLock::new();

impl TranslateManager
{
	pub fn singleton() -> &'static Self
	{
		SINGLETON.get_or_init(||Self {
			_translateTime: DashMap::new()
		})
	}

	pub fn updateMissing(&self, lang: String, key: String)
	{
		HTrace!((Type::WARNING) "Missing translation for {}:{}", &lang, &key);
		let mut conf = HConfigManager::singleton().get(format!("translate_{}",self.filterLang(lang)));
		conf.set(key, "---missing---");
		HTraceError!(conf.save());
	}

	pub fn getBook(&self, lang: String) -> TranslateBook
	{
		let conf = HConfigManager::singleton().get(format!("translate_{}",self.filterLang(lang.clone())));
		// TODO reload if older conf;

		if(!self._translateTime.contains_key(&lang))
		{
			self._translateTime.insert(lang.clone(), 0);
		}

		let book = match TranslateBook::from(*self._translateTime.get(&lang).unwrap().value(),conf.getRoot()) {
			Ok(book) => book,
			Err(err) => {
				HTrace!((Type::ERROR) "Book {} cannot be created : {}", &lang, &err);
				return TranslateBook::new();
			}
		};

		return book;
	}

	///// PRIVATE

	fn filterLang(&self, lang: String) -> String
	{
		let lang = lang.to_uppercase();
		let allowed = ["EN","FR"];
		if(allowed.contains(&lang.as_str()))
		{
			return lang;
		}

		return "EN".to_string();
	}


}