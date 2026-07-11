use leptos::prelude::{ClassAttribute, RenderHtml};
use leptos::{island, view, IntoView};
use leptos::prelude::{signal, ElementChild, GlobalAttributes};
use leptos_obfuscate::ObfuscateEmail;
use leptos_router::components::A;
use time::OffsetDateTime;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[island]
pub fn PersoCV() -> impl IntoView {
	let now = OffsetDateTime::now_utc();
	let (email, _) = signal("hyultis@gmail.com".to_string());

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / Curriculum vitae"</h2>

		<article id="cv">
			<div class="element_align_left">
				"M. D"<span class="element_cv_color">---</span>" B"<span class="element_cv_color">---</span><br/>
				"Age : " {move || now.year() - 1989}" " ans<br/>
				"Courriel : " <ObfuscateEmail email /><br/>

				<Translate key="pagePersoCV_desc" isBloc=true/>
			</div>
		</article>
    }
}
