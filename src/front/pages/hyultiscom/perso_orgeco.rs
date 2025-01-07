use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;

/// Renders the home page of your application.
#[component]
pub fn PersoORGECO() -> impl IntoView {

	view! {
		<h2><span>ORGECO 30</span></h2>

		<article>
			<a href="https://orgeco30.fr/" rel="noopener noreferrer nofollow" target="_blank">"https://orgeco30.fr/"</a><br/>
			<br/>
			It was a local consumer protection association.<br/>
			I worked on the website since 2012.<br/>
	        "I'm the designer of the logo/website theme."<br/>
	        <br/>
	        The association closed in 2023.<br/><br/>
	        <img src="./img/orgeco30old.png" style="width: 100%; border: 2px solid black" alt="orgeco screen"/>
		</article>
    }
}
