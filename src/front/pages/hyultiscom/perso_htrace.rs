use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoHtrace() -> impl IntoView {

	view! {
		<h2><span>Htrace</span></h2>

		<article>
			<Translate key="pagePersoHtrace_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://crates.io/crates/Htrace" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
			<a href="https://github.com/hyultis/rust_Htrace" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
