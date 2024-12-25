use reactive_stores::Store;

#[derive(Clone, Debug, Store)]
pub struct UserData
{
	lang: String
}

impl UserData {
	pub fn new(lang: &String) -> Self
	{
		let mut new = Self::default();
		new.lang_set(lang);
		return new;
	}

	pub fn lang_get(&self) -> String
	{
		self.lang.clone()
	}

	pub fn lang_set(&mut self, lang: impl Into<String>)
	{
		let lang = lang.into();
		let splittedVal = lang.split('-').collect::<Vec<&str>>();
		self.lang = splittedVal.first().unwrap_or(&"EN").to_string().to_uppercase();
	}
}

impl Default for UserData
{
	fn default() -> Self
	{
		Self {
			lang: "EN".to_string(),
		}
	}
}