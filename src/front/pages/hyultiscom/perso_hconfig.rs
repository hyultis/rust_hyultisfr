use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, StyleAttribute};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoHconfig() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / Hconfig"</h2>

		<article>
			<div>
				<Translate key="pagePersoHconfig_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://crates.io/crates/Hconfig" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
				<a href="https://github.com/hyultis/rust_Hconfig" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
