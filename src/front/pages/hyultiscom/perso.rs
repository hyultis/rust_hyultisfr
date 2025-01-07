use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;
use leptos_router::components::A;
use time::OffsetDateTime;

/// Renders the home page of your application.
#[component]
pub fn Perso() -> impl IntoView {

	let now = OffsetDateTime::now_utc();

	view! {
		<h2><span>About Me</span></h2>

		<article>
	        "I'm Hyultis," {move || now.year() - 1989} " years old, passionate by programming since my childhood."<br/>
	        You can see my <span class="linktointernal" click="changepage('cv','FR')">Curriculum vitae (french only)</span>.<br/>
	        Graduated from a BTS IG management, here is some project i developed on my free time :
			<ul>
				<li><A href="/">Home</A>" : My individual enterprise, i'm developing games."</li>
	            <li><A href="PersoRustWebsite">Website in rust</A>  : this website realised in rust with leptos.</li>
	            <li><A href="PersoHtrace">Htrace</A>  : this website realised in rust with leptos.</li>
	            <li><A href="PersoHconfig">Hconfig</A>  : this website realised in rust with leptos.</li>
	            <li><A href="PersoHArcMut">HArcMut</A>  : this website realised in rust with leptos.</li>
	            <li><A href="PersoSingletonThread">singletonThread</A>  : this website realised in rust with leptos.</li>
			</ul>
			Old and Abandoned project :
			<ul>
	            <li><A href="PersoHwe">Hwe</A>  : old website system from ~2018</li>
	            <li><A href="PersoORGECO">OR.GE.CO 30</A> : website of a consumer protection association, the association closed in 2023.</li>
				<li><A href="PersoHcms">Hcms</A> : old website system from ~2010</li>
				<li><A href="PersoVidPHPConverter">VidPHPConverter</A> : PHP class that overlay FFMpeg executable (getting infos, conversion, streaming, etc)</li>
				<li><A href="PersoCasseBrique">Casse brique</A> : briks breaker like, in pure HTML/JAVASCRIPT (experimental in 2012)</li>
				<li><A href="PersoWowmystats">Wowmystats</A> : a system who follow evolving stats of a WOW character</li>
			</ul>
		</article>
    }
}
