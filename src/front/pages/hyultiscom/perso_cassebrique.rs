use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn PersoCasseBrique() -> impl IntoView {

	view! {
		<h2><span><Translate key="pagePersoCasseBrique_title"/></span></h2>

		<article>
			<Translate key="pagePersoCasseBrique_desc"/>
			<br/><br/>
			<Translate key="pagePerso_all_link"/>" "
			<a href="https://github.com/hyultis/old_cassebrique" rel="noopener noreferrer nofollow" target="_blank">Github</a>
		</article>
    }
}
