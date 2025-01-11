use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoSingletonThread() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / SingletonThread"</h2>

		<article>
			<div>
				<Translate key="pagePersoSingletonThread_desc"/>
			</div>
			<br/><br/>
			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://crates.io/crates/singletonThread" rel="noopener noreferrer nofollow" target="_blank">Crates.io</a>," "
				<a href="https://github.com/hyultis/singletonThread" rel="noopener noreferrer nofollow" target="_blank">Github</a>
			</div>
		</article>
    }
}
