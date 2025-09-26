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

			<div>
				<Translate key="pagePerso_all_link"/>" "
				<a href="https://github.com/hyultis/rust_hyultisfr" rel="noopener noreferrer nofollow" target="_blank">Github</a>
				<br/><br/>
				Icons :<br/>
				<a href="https://www.flaticon.com/fr/icone-gratuite/curriculum-vitae_1150635" rel="noopener noreferrer nofollow" target="_blank">curriculum-vitae</a><br/>
				<a href="https://www.flaticon.com/fr/icone-gratuite/apprentissage-en-ligne_2436874" rel="noopener noreferrer nofollow" target="_blank">apprentissage-en-ligne</a><br/>
				<a href="https://www.flaticon.com/fr/icone-gratuite/valide_8766381" rel="noopener noreferrer nofollow" target="_blank">valide</a><br/>
				<a href="https://www.flaticon.com/fr/icone-gratuite/diminue_4847128" rel="noopener noreferrer nofollow" target="_blank">diminue</a><br/>
				<a href="https://rustacean.net/" rel="noopener noreferrer nofollow" target="_blank">Ferris</a>

			</div>
		</article>
    }
}
