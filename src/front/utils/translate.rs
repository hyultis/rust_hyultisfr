use std::collections::HashMap;
use std::sync::Arc;
use leptos::{component, view, IntoView};
use leptos::children::ChildrenFn;
use leptos::prelude::{expect_context, Get, IntoAny, Read, Resource};
use leptos::suspense::Transition;
use reactive_stores::Store;
use crate::front::utils::fluent::FluentManager::FluentManager;
use crate::front::utils::usersData::{UserData, UserDataStoreFields};

#[component]
pub fn TranslateCurrentLang() -> impl IntoView {
	let userData = expect_context::<Store<UserData>>();
	let langActual = Resource::new(
		move || userData.lang().get(),
		move |lang| async move {
			let mut tmp = "swap_to_".to_string();
			tmp.push_str(lang.clone().as_str());
			return tmp;
		}
	);
	view!{
		<TranslateFn key=move || langActual.read().clone().unwrap_or("swap_to_EN".to_string())/>
	}
}

#[component]
pub fn Translate(#[prop(into)] key: String,
                 #[prop(optional)]
                 params: HashMap<String,String>,
                 #[prop(optional)]
                 children: Option<ChildrenFn>) -> impl IntoView {

	if let Some(children) = children {
		return view!{
			<TranslateFn key=move || key.clone() params=params children=children/>
		}.into_any()
	}

	return view!{
		<TranslateFn key=move || key.clone() params=params/>
	}.into_any();
}

#[component]
pub fn TranslateFn(
	key: impl Fn() -> String + Send + Sync + 'static,
    #[prop(optional)]
	mut params: HashMap<String,String>,
	#[prop(optional)]
	children: Option<ChildrenFn>) -> impl IntoView {

	let key = Arc::new(move || key());
	let userData = expect_context::<Store<UserData>>();
	let splitted= "{--$chidren--}";

	if(children.is_some())
	{
		params.insert("children".to_string(),splitted.to_string());
	}
	let params = Arc::new(params);

	let subkey = key.clone();
	let translate = Resource::new(
		move || userData.lang().get(),
		move |lang| {
			FluentManager::singleton().translate(lang, subkey.clone()(),params.clone())
		}
	);

	let altkey = key.clone();
	let altkey2 = key.clone();
	view! {
		<Transition fallback=move || view! { {format!("{}_fallback",altkey.clone()())} }>
			{move || {
				let Some(translate) = translate.get() else {
					return view! { {altkey2.clone()()} }.into_any();
				};
				let splitted = splitted.to_string();
				if(translate.contains(splitted.as_str()))
				{
					let splitVar = translate.split_once(splitted.as_str());
					let (prefix,suffix) = splitVar.unwrap();
					let prefix = prefix.to_string();
					let suffix = suffix.to_string();
					if let Some(children) = &children
					{
						view! { {prefix}{children()}{suffix} }.into_any()
					}
					else
					{
						view! { {prefix}{suffix} }.into_any()
					}
				}
				else
				{
					view! { {translate} }.into_any()
				}
			}}
		</Transition>
	}
}