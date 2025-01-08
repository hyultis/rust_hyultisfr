use leptos::{component, view, IntoView};
use leptos::prelude::{signal, ElementChild};
use leptos_obfuscate::ObfuscateEmail;
use time::OffsetDateTime;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[component]
pub fn PersoCV() -> impl IntoView {
	let now = OffsetDateTime::now_utc();
	let (email, _) = signal("hyultis@gmail.com".to_string());

	view! {
		<h2><span>Curriculum vitae</span></h2>

		<article>
			M." " J<span style="color: #aaa">---</span>" " B<span style="color: #aaa">---</span><br/>
			Age :" " {move || now.year() - 1989}" " ans<br/>
			Courriel :" " <ObfuscateEmail email /><br/>

			<Translate key="pagePersoCV_desc"/>
		</article>
    }
}
