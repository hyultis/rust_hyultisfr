use leptos::server;
use leptos::server_fn::ServerFnError;
use crate::api::translateBook::TranslateBook;
#[cfg(feature = "ssr")]
use crate::api::translateManager::TranslateManager;

#[server]
pub async fn APItranslate_getBook(lang: String) -> Result<TranslateBook, ServerFnError>
{
	let book = TranslateManager::singleton().getBook(lang);

	return Ok(book);
}

#[server]
pub async fn APItranslate_updateMissing(lang: String,key: String) -> Result<(), ServerFnError>
{
	TranslateManager::singleton().updateMissing(lang,key);

	return Ok(());
}