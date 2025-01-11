use leptos::{component, view, IntoView};
use leptos::prelude::{GlobalAttributes};
use leptos_router::components::A;
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;
use leptos::prelude::StyleAttribute;

#[component]
pub fn Accueil() -> impl IntoView {
	view! {
		<h2>Home</h2>

		<article style="margin-bottom: 3em">
			<Translate key="pageAccueil_desc"/>
		</article>

		<div id="gamelist">
			<div><A href="/Game/Heatchain"><img src="./img/heatchain/heatchain_capsule.png" alt="heatchain logo"/></A></div>
			//<div style="cursor: not-allowed"><A href="/"><img style="background: orange;" src="./img/unkowngame.png" alt="unkown logo"/></A></div>
			//<!--<span class="game" style="cursor: not-allowed" title="Future game incoming"><img style="background: orange;" src="./img/unkowngame.png" alt="unkown logo"></span>-->
		</div>

		<Translate key="pageAccueil_goperso"/>" "<A href="/Perso"><Translate key="menu_me"/></A>
	}
}