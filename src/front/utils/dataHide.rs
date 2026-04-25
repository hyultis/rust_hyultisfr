use leptos::prelude::ElementChild;
use std::time::Duration;
use leptos::{component, view, IntoView};
use leptos::prelude::{set_timeout, signal, ClassAttribute, Effect, Get, GetUntracked, RwSignal, Set};

#[component]
pub fn DataHide(data: String) -> impl IntoView {

	let (data, _) = signal(data);
	let fakeData = RwSignal::new("loading...".to_string());

	let is_initialized = RwSignal::new(false);
	Effect::new(move || {
		if(is_initialized.get_untracked()) {
			return;
		}
		is_initialized.set(true);
		let fakeData = fakeData.clone();
		set_timeout(move || {
			fakeData.set(data.get());
		}, Duration::from_secs(2));
	});

	view!{
		{move || fakeData.get()}
	}
}


#[component]
pub fn DataHideMail(mailTo: String, text: Option<String>) -> impl IntoView {

	let (email, _) = signal(mailTo);
	let fakeMail = RwSignal::new("mailto:honeypot@example.com".to_string());
	let fakeMailText = RwSignal::new("loading...".to_string());

	let is_initialized = RwSignal::new(false);
	Effect::new(move || {
		if(is_initialized.get_untracked()) {
			return;
		}
		is_initialized.set(true);
		let email = email.clone();
		let text = text.clone();
		set_timeout(move || {
			fakeMail.set(format!("mailto:{}",email.get()));
			fakeMailText.set(text.unwrap_or(email.get()));
		}, Duration::from_secs(2));
	});

	view!{
		<a class="none" href=move || fakeMail.get()>{move || fakeMailText.get()}</a>
	}
}