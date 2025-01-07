use leptos::{component, view, IntoView};
use leptos::prelude::{ClassAttribute, GlobalAttributes};
use leptos_router::components::A;
use leptos::prelude::ElementChild;
use crate::front::utils::translate::Translate;

#[component]
pub fn Accueil() -> impl IntoView {
	view! {
		<div>
			<h2><span>Home</span></h2>

			<article style="margin-bottom: 3em">
				"Welcome to Hyultis website, i'm producing video game(s)."
			</article>

			<nav class="gamelist">
				<A href="GameHeatchain"><span class="game" title="Heatchain"><img style="background: orange;" src="./img/heatchain_capsule.png" alt="heatchain logo"/></span></A>
				//<!--<span class="game" style="cursor: not-allowed" title="Future game incoming"><img style="background: orange;" src="./img/unkowngame.png" alt="unkown logo"></span>-->
			</nav>

			<hr style="visibility: hidden;clear: both;margin-bottom: 10em"/>

			want to know more about me ? go <A href="Perso"><Translate key="menu_me"/></A>
		</div>
	}
}