use leptos::prelude::{ClassAttribute, RenderHtml};
use leptos::{island, view, IntoView};
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[island]
pub fn GameHeatchain() -> impl IntoView {

	view! {
		<h2>Heatchain</h2>

		<article id="heatchain">

			<div class="element_align_center">
				<Translate key="pageGameHeatchain_desc"/>
			</div>
			<div class="element_align_left">
				<iframe class="element_heatchain_iframe_style"
			        src="https://www.youtube-nocookie.com/embed/SEbTryPeL8o?si=OOdKFJAOwNvLIlTW"
			        title="YouTube video player"
			        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
				<br/>
				<Translate key="pageGameHeatchain_link"/><br/>
				<ul>
					<li>"Steam : "<a class="externe" href="https://store.steampowered.com/app/2521350/Heatchain" rel="noopener noreferrer nofollow" target="_blank">"https://store.steampowered.com/app/2521350/Heatchain"</a></li>
					<li>"Itch.io : "<a class="externe" href="https://hyultis.itch.io/heatchain" rel="noopener noreferrer nofollow" target="_blank">"https://hyultis.itch.io/heatchain"</a></li>
					<li>"Android : "<a class="externe" href="https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity" rel="noopener noreferrer nofollow" target="_blank">"https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity"</a></li>
					<li>"Discord : "<a class="externe" href="https://discord.gg/m3pnhTVCSr" rel="noopener noreferrer nofollow" target="_blank">"https://discord.gg/m3pnhTVCSr"</a></li>
					<li>"Github : "<a class="externe" href="https://github.com/hyultis/heatchain_public" rel="noopener noreferrer nofollow" target="_blank">"https://github.com/hyultis/heatchain_public"</a></li>
				</ul>
			</div>

		</article>
    }
}
