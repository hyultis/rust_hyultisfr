use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoWowMyStats() -> impl IntoView {

	view! {
		<h2><span>WowMyStats</span></h2>

		<article>
			<Translate key="pagePersoWowMyStats_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://github.com/hyultis/old_Wowmystats" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
