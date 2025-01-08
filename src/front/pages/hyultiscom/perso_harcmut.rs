use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoHArcMut() -> impl IntoView {

	view! {
		<h2><span>HArcMut</span></h2>

		<article>
			<Translate key="pagePersoHArcMut_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://crates.io/crates/HArcMut" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
			<a href="https://github.com/hyultis/HArcMut" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
