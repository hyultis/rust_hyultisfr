use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoORGECO() -> impl IntoView {

	view! {
		<h2><span>ORGECO 30</span></h2>

		<article>
			<s><a href="https://orgeco30.fr/" rel="noopener noreferrer nofollow" target="_blank">"https://orgeco30.fr/"</a></s><br/>
			<br/>
			<Translate key="pagePersoORGECO30_desc"/><br/><br/>
	        <img src="/img/orgeco30old.png" style="width: 100%; border: 2px solid black" alt="orgeco screen"/>
		</article>
    }
}
