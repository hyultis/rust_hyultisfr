use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoHconfig() -> impl IntoView {

	view! {
		<h2><span>Hconfig</span></h2>

		<article>
			<Translate key="pagePersoHconfig_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://crates.io/crates/Hconfig" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
			<a href="https://github.com/hyultis/rust_Hconfig" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
