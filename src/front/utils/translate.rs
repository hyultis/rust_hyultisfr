use std::collections::HashMap;
use leptos::{component, view, IntoView};
use leptos::children::ChildrenFn;
use leptos::html::InnerHtmlAttribute;
use leptos::prelude::{expect_context, Get, IntoAny};
use leptos::suspense::Transition;
use leptos::prelude::ElementChild;
use reactive_stores::Store;
use crate::front::utils::fluent::FluentManager::FluentManager;
use crate::front::utils::usersData::{UserData, UserDataStoreFields};

#[component]
pub fn TranslateCurrentLang() -> impl IntoView {

	view! { <TranslateFn key=move || {
		let tmp = expect_context::<Store<UserData>>().lang().get();
		return format!("swap_to_{}",tmp);
	}/> }.into_any()
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
	key: impl Fn() -> String + Send + Sync + Clone + 'static,
    #[prop(optional)]
	mut params: HashMap<String,String>,
	#[prop(optional)]
	children: Option<ChildrenFn>) -> impl IntoView {

	let splitted= "{--$chidren--}";

	if(children.is_some())
	{
		params.insert("children".to_string(),splitted.to_string());
	}

	let translate = FluentManager::getAsResource(key.clone(),params);

	let altkey = key.clone();
	view! {
		<Transition fallback=move || view! { <span>{format!("{}_fallback",altkey.clone()())}</span> }.into_any()>
			{move || translate.get().map(|translated|{
					if let Some((prefix,suffix)) = translated.split_once(splitted)
					{
						let prefix = prefix.to_string();
						let suffix = suffix.to_string();
						if let Some(children) = &children
						{
							view! { <span inner_html={prefix}/>{children()}<span inner_html={suffix}/> }.into_any()
						}
						else
						{
							view! { <span inner_html={prefix}/><span inner_html={suffix}/> }.into_any()
						}
					}
					else
					{
						// first "" is important to fix the hydration bug from fallback
						view! { <span inner_html={translated}/> }.into_any()
					}
				})
			}
		</Transition>
	}
}