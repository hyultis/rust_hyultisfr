use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[component]
pub fn PersoRustWebsite() -> impl IntoView {

	view! {
		<h2><span><Translate key="pagePersoRustWebsite_title"/><br/><br/></span></h2>

		<article>
			<Translate key="pagePersoRustWebsite_desc"/>" "<A href="PersoHwe">Hwe</A>.
		</article>
    }
}
