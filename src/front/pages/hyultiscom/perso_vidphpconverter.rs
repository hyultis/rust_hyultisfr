use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, StyleAttribute};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoVidPHPConverter() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / VidPHPConverter"</h2>

		<article>
			<div>
				<Translate key="pagePersoVidPHPConverter_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://github.com/hyultis/old_VidPHPConverter" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
