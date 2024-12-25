use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use leptos::prelude::StyleAttribute;
use leptos::prelude::GlobalAttributes;

/// Renders the home page of your application.
#[component]
pub fn GameHeatchain() -> impl IntoView {

	view! {
		<h2><span>Heatchain</span></h2>

		<iframe style="margin: 0 auto;display:block;border: 0;"
		        width="560"
		        height="315"
		        src="https://www.youtube-nocookie.com/embed/qaBgZoz4Zwg?si=OOdKFJAOwNvLIlTW"
		        title="YouTube video player"
		        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

		<br/><br/>
		Quickly find the next glyph, take the bonuses without wasting time and make the longest chain!<br/>
		<br/>
		This little arcade game focuses on short and intense games.<br/>
		<br/>
		<h3>learn more about the game on :</h3>
		<ul>
			<li>Steam : <a href="https://store.steampowered.com/app/2521350/Heatchain" rel="noopener noreferrer nofollow" target="_blank">"https://store.steampowered.com/app/2521350/Heatchain"</a></li>
			<li>Itch.io : <a href="https://hyultis.itch.io/heatchain" rel="noopener noreferrer nofollow" target="_blank">"https://hyultis.itch.io/heatchain"</a></li>
			<li>Android : <a href="https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity" rel="noopener noreferrer nofollow" target="_blank">"https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity"</a></li>
			<li>Discord : <a href="https://discord.gg/m3pnhTVCSr" rel="noopener noreferrer nofollow" target="_blank">"https://discord.gg/m3pnhTVCSr"</a></li>
			<li>Github : <a href="https://github.com/hyultis/heatchain_public" rel="noopener noreferrer nofollow" target="_blank">"https://github.com/hyultis/heatchain_public"</a></li>
		</ul>
    }

	/*

		<ul>
			<li>Steam : <a href="https://store.steampowered.com/app/2521350/Heatchain" rel="noopener noreferrer nofollow" target="_blank">https://store.steampowered.com/app/2521350/Heatchain</a></li>
			<li>Itch.io : <a href="https://hyultis.itch.io/heatchain" rel="noopener noreferrer nofollow" target="_blank">https://hyultis.itch.io/heatchain</a></li>
			<li>Android : <a href="https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity" rel="noopener noreferrer nofollow" target="_blank">https://play.google.com/store/apps/details?id=hyultis.heatchain.MainActivity</a></li>
			<li>Discord : <a href="https://discord.gg/m3pnhTVCSr" rel="noopener noreferrer nofollow" target="_blank">https://discord.gg/m3pnhTVCSr</a></li>
			<li>Github : <a href="https://github.com/hyultis/heatchain_public" rel="noopener noreferrer nofollow" target="_blank">https://github.com/hyultis/heatchain_public</a></li>
		</ul>
	 */
}
