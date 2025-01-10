use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, StyleAttribute};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoWowMyStats() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / WowMyStats"</h2>

		<article>
			<div>
				<Translate key="pagePersoWowMyStats_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://github.com/hyultis/old_Wowmystats" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
