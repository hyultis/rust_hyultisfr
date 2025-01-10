use std::collections::HashMap;
use std::sync::Arc;
use leptos::{component, view, IntoView};
use leptos::children::ChildrenFn;
use leptos::html::InnerHtmlAttribute;
use leptos::prelude::{expect_context, Get, IntoAny, Read, Resource};
use leptos::suspense::Transition;
use reactive_stores::Store;
use leptos::prelude::ElementChild;
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
	view! {
		<Transition fallback=move || view! { <span>{format!("{}_fallback",altkey.clone()())}</span> }>
			{move || translate.read().as_ref().cloned().map(|translated|{
					if(translated.contains(splitted))
					{
						let splitVar = translated.split_once(splitted);
						let (prefix,suffix) = splitVar.unwrap();
						let prefix = prefix.to_string();
						let suffix = suffix.to_string();
						if let Some(children) = &children
						{
							view! { <span><span inner_html=move || prefix.clone()/>{children()}<span inner_html=move || suffix.clone()/></span > }.into_any()
						}
						else
						{
							view! { <span><span inner_html=move || prefix.clone()/><span inner_html=move || suffix.clone()/></span> }.into_any()
						}
					}
					else
					{
						//view! { <span>{translated.clone()}</span> }.into_any()
						view! { <span inner_html=move || translated.clone()/> }.into_any()
					}
				})
			}
		</Transition>
	}
}