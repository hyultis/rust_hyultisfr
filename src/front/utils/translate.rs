use leptos::{component, view, IntoView};
use leptos::prelude::{expect_context, Action, Get, Read, Signal};
use leptos::server::Resource;
use leptos::suspense::Transition;
use reactive_stores::Store;
use crate::api::translateCall::{APItranslate_getBook, APItranslate_updateMissing};
use crate::front::utils::usersData::{UserData, UserDataStoreFields};


#[component]
pub fn Translate(#[prop(into)] key: String) -> impl IntoView {
	view!{
		<TranslateFn key=move || key.clone()/>
	}
}

#[component]
pub fn TranslateFn(key: impl Fn() -> String + Send + Sync + 'static) -> impl IntoView {

	//let (lang, _) = signal(lang.into());
	let userData = expect_context::<Store<UserData>>();
	//let key = key.into();

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

	view! {
		<Transition fallback=move || view! { {"test"} }>
			{tmp}
		</Transition>
	}
}