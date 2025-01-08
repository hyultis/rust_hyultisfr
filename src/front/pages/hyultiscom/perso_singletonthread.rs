use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoSingletonThread() -> impl IntoView {

	view! {
		<h2><span>SingletonThread</span></h2>

		<article>
			<Translate key="pagePersoSingletonThread_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://crates.io/crates/singletonThread" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
			<a href="https://github.com/hyultis/singletonThread" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
