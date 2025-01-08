use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoVidPHPConverter() -> impl IntoView {

	view! {
		<h2><span>VidPHPConverter</span></h2>

		<article>
			<Translate key="pagePersoVidPHPConverter_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://github.com/hyultis/old_VidPHPConverter" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
