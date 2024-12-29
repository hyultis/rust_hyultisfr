use crate::front::pages::hyultiscom::game_heatchain::GameHeatchain;
use crate::front::pages::hyultiscom::accueil::Accueil;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;
use leptos_use::use_locales;
use reactive_stores::Store;
use crate::front::utils::translate::{Translate, TranslateCurrentLang, TranslateFn};
use crate::front::utils::usersData::{UserData};

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta http-equiv="content-type" content="text/html; charset=UTF-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<meta http-equiv="Referrer-Policy" content="no-referrer, strict-origin-when-cross-origin"/>
				//<meta http-equiv="Content-Security-Policy" content="script-src https: 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval'"/> // actuellement instable avec leptos ?
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

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let locales = use_locales();
	provide_context(Store::new(UserData::new(locales.get().first().unwrap_or(&"EN".to_string()))));

	let userData = expect_context::<Store<UserData>>();
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

			<header>Hyultis - <TranslateFn key= move || "hello".to_string()/></header>

			// content for this welcome page
			<Router>
				<nav>
					<ul class="menu">
						<li><A href="/"><Translate key="menu_home"/></A></li>
						<li><TranslateCurrentLang/>
							<ul>
								<li on:click=move |_| userData.write().lang_set("EN")><Translate key="swap_to_EN"/></li>
								<li on:click=move |_| userData.write().lang_set("FR")><Translate key="swap_to_FR"/></li>
							</ul>
						</li>
					</ul>
				</nav>

				<section>
					<Routes fallback=|| Page404>
						<Route path=StaticSegment("") view=Accueil/>
						<Route path=StaticSegment("GameHeatchain") view=GameHeatchain/>
					</Routes>
				</section>
			</Router>


			<footer>
				design by <a href="mailto:%68%79%75%6c%74%69%73%40%67%6d%61%69%6c%2e%63%6f%6d">Hyultis</a><br/>
				<span style="font-size: 0.5em">fully realised with <a href="https://leptos.dev/">leptos</a> in <a href="https://www.rust-lang.org/">rust</a></span>
			</footer>
		</div>
	}
}


#[component]
pub fn Page404() -> impl IntoView {
	view!{
		<h2><Translate key="page404_title"/></h2>

		<Translate key="page404_content" ><A href="/"><Translate key="menu_home"/></A></Translate>
	}
}