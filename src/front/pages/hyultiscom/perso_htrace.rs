use leptos::prelude::RenderHtml;
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[island]
pub fn PersoHtrace() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / Htrace"</h2>

		<article>
			<div>
				<Translate key="pagePersoHtrace_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://crates.io/crates/Htrace" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
				<a href="https://github.com/hyultis/rust_Htrace" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
