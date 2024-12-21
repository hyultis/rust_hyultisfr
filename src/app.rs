use crate::pages::hyultiscom::game_heatchain::GameHeatchain;
use crate::pages::hyultiscom::accueil::Accueil;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta http-equiv="content-type" content="text/html; charset=UTF-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<meta http-equiv="Referrer-Policy" content="no-referrer, strict-origin-when-cross-origin"/>
				<meta http-equiv="Content-Security-Policy" content="script-src'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval'"/> // https:
				<AutoReload options=options.clone() />
				<HydrationScripts options/>
				<MetaTags/>
			</head>
			<body>
				<App/>
			</body>
		</html>
	}
}

/*
				<meta http-equiv="Content-Security-Policy" content="default-src https: 'unsafe-inline' 'unsafe-eval'"/>
				<meta http-equiv="Referrer-Policy" content="no-referrer, strict-origin-when-cross-origin"/>
 */
	
#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/hyultisfr.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>
		<Meta name="description" content="Site personnel de hyultis"/>

		<Link rel="preload" fetchpriority="high" as_="image" href="./img/header.png" type_="image/png"/>

		<div id="body">
			<div class="imgheader"></div>

			<header>Hyultis</header>


			// content for this welcome page
			<Router>
				<nav>
					<ul class="menu">
						<li><A href="/">HOME</A></li>
						<li>trad

							<ul>
								<li>EN</li>
								<li>FR</li>
							</ul>
						</li>
					</ul>
				</nav>

				<section>
					<Routes fallback=|| "Page not found.".into_view()>
						<Route path=StaticSegment("") view=Accueil/>
						<Route path=StaticSegment("GameHeatchain") view=GameHeatchain/>
					</Routes>
				</section>
			</Router>


			<footer>
				design by <a href="mailto:%68%79%75%6c%74%69%73%40%67%6d%61%69%6c%2e%63%6f%6d">Hyultis</a><br/>
				<span style="font-size: 0.5em">realised with <a href="https://leptos.dev/">leptos</a> in <a href="https://www.rust-lang.org/">rust</a></span>
			</footer>
		</div>
	}
}