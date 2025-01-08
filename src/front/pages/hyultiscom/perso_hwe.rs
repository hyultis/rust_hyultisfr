use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoHwe() -> impl IntoView {

	view! {
		<h2><span>Hwe</span></h2>

		<article>
			<Translate key="pagePersoHwe_desc"/>
		</article>
    }
}
