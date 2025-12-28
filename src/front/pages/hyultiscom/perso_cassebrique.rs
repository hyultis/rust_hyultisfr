use leptos::prelude::RenderHtml;
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[island]
pub fn PersoCasseBrique() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / "<Translate key="pagePersoCasseBrique_title"/></h2>

		<article>
			<div>
				<Translate key="pagePersoCasseBrique_desc"/>
			</div>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://github.com/hyultis/old_cassebrique" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
