use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoORGECO() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / ORGECO 30"</h2>

		<article>
			<div>
				<s><a href="https://orgeco30.fr/" rel="noopener noreferrer nofollow" target="_blank">"https://orgeco30.fr/"</a></s><br/>
				<br/>
				<Translate key="pagePersoORGECO30_desc"/><br/><br/>
			</div>
	        <img src="/img/orgeco30old.png" style="width: 100%; border: 2px solid black" alt="orgeco screen"/>
		</article>
    }
}
