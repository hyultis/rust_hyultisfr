use leptos::prelude::{ClassAttribute, RenderHtml};
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[island]
pub fn PersoHwe() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / Hwe"</h2>

		<article>
			<Translate key="pagePersoHwe_desc1"/>
			<div class="element_align_left">
				<Translate key="pagePersoHwe_desc2"/>
			</div>
			<Translate key="pagePersoHwe_desc3"/>
		</article>
    }
}
