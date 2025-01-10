use leptos::{component, view, IntoView};
use leptos::prelude::{signal, ElementChild, GlobalAttributes, StyleAttribute};
use leptos_obfuscate::ObfuscateEmail;
use leptos_router::components::A;
use time::OffsetDateTime;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[component]
pub fn PersoCV() -> impl IntoView {
	let now = OffsetDateTime::now_utc();
	let (email, _) = signal("hyultis@gmail.com".to_string());

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / Curriculum vitae"</h2>

		<article id="cv">
			<div style="text-align: left;">
				M." " J<span style="color: #aaa">---</span>" " B<span style="color: #aaa">---</span><br/>
				Age :" " {move || now.year() - 1989}" " ans<br/>
				Courriel :" " <ObfuscateEmail email /><br/>

				<Translate key="pagePersoCV_desc"/>
			</div>
		</article>
    }
}
