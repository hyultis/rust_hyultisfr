use leptos::prelude::RenderHtml;
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[island]
pub fn PersoWebhome() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / WebHome"</h2>

		<article>
			<div>
				<Translate key="pagePersoWebHome_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://github.com/hyultis/web-home" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
