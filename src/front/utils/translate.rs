use leptos::{component, view, IntoView};
use leptos::logging::log;
use leptos::prelude::{expect_context, Get, Resource};
use leptos::suspense::Transition;
use reactive_stores::Store;
use crate::front::utils::fluent::FluentManager::FluentManager;
use crate::front::utils::usersData::{UserData, UserDataStoreFields};


#[component]
pub fn Translate(#[prop(into)] key: String) -> impl IntoView {
	view!{
		<TranslateFn key=move || key.clone()/>
	}
}

#[component]
pub fn TranslateFn(key: impl Fn() -> String + Send + Sync + 'static) -> impl IntoView {

	let key = key();
	let userData = expect_context::<Store<UserData>>();

	let subkey = key.clone();
	let translate = Resource::new(
		move || userData.lang().get(),
		move |lang| {
			FluentManager::singleton().translate(lang, subkey.clone())
		}
	);

	let altkey = key.clone();
	view! {
		<Transition fallback=move || view! { {format!("{}_fallback",altkey.clone())} }>
			{translate}
		</Transition>
	}
}

/*
	let async_data = Resource::new(
		move || userData.lang().get(),
		// every time `count` changes, this will run
		move |lang| APItranslate_getBook(lang)
	);

	let tmp = move || -> String {

		let add_todo_action = Action::new(|truc: &(String,String)| {
			let (lang,key) = truc.to_owned();
			async move { APItranslate_updateMissing(lang,key).await }
		});

		let binding = async_data.read();
		let subbinding = binding.as_ref().unwrap();
		let dd = subbinding.as_ref().unwrap();
		match dd.get(key())
		{
			Some(value) => return value.to_string(),
			None => {
				add_todo_action.dispatch((userData.lang().get(),key()));
				return key();
			}
		}
	};

	{tmp}
 */