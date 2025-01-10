use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use leptos_router::components::A;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[component]
pub fn PersoRustWebsite() -> impl IntoView {

	view! {
		<h2><A href="/Perso"><Translate key="menu_me"/></A>" / "<Translate key="pagePersoRustWebsite_title"/></h2>

		<article>
			<div><Translate key="pagePersoRustWebsite_desc"/>" "<A href="../Hwe">Hwe</A>.</div>
		</article>
    }
}
