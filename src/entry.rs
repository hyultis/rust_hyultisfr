use std::time::Duration;
use crate::front::pages::hyultiscom::game_heatchain::GameHeatchain;
use crate::front::pages::hyultiscom::accueil::Accueil;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use leptos_use::use_locales;
use reactive_stores::Store;
use crate::front::pages::hyultiscom::perso::Perso;
use crate::front::pages::hyultiscom::perso_cassebrique::PersoCasseBrique;
use crate::front::pages::hyultiscom::perso_cv::PersoCV;
use crate::front::pages::hyultiscom::perso_harcmut::PersoHArcMut;
use crate::front::pages::hyultiscom::perso_hconfig::PersoHconfig;
use crate::front::pages::hyultiscom::perso_htrace::PersoHtrace;
use crate::front::pages::hyultiscom::perso_hwe::PersoHwe;
use crate::front::pages::hyultiscom::perso_orgeco::PersoORGECO;
use crate::front::pages::hyultiscom::perso_rustwebsite::PersoRustWebsite;
use crate::front::pages::hyultiscom::perso_singletonthread::PersoSingletonThread;
use crate::front::pages::hyultiscom::perso_vidphpconverter::PersoVidPHPConverter;
use crate::front::pages::hyultiscom::perso_wowmystats::PersoWowMyStats;
use crate::front::utils::translate::{Translate, TranslateCurrentLang};
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
	let (email, _) = signal("hyultis@gmail.com".to_string());

	let mailto = RwSignal::new("mailto:honeypot@example.com".to_string());

	Effect::new(move |_| {
		let mail = format!("mailto:{}", email.get());
		set_timeout(move || mailto.set(mail), Duration::from_secs(3));
	});

	view! {
		// injects a stylesheet into the document <head>
		// id=leptos means cargo-leptos will hot-reload this stylesheet
		<Stylesheet id="leptos" href="/pkg/hyultisfr.css"/>

		// sets the document title
		<Title text="Hyultis"/>
		<Meta name="description" content="Site personnel de hyultis"/>

		<Link rel="preload" fetchpriority="high" as_="image" href="./img/header.png" type_="image/png"/>

		<div class="background">
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		   <span></span>
		</div>

		<div id="body">
			<div class="imgheader"></div>

			<header>Hyultis</header>

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
						<Route path=path!("/") view=Accueil/>
						<Route path=path!("/Game/Heatchain") view=GameHeatchain/>
						<Route path=path!("/Perso") view=Perso/>
						<Route path=path!("/Perso/RustWebsite") view=PersoRustWebsite/>
						<Route path=path!("/Perso/Htrace") view=PersoHtrace/>
						<Route path=path!("/Perso/Hconfig") view=PersoHconfig/>
						<Route path=path!("/Perso/HArcMut") view=PersoHArcMut/>
						<Route path=path!("/Perso/SingletonThread") view=PersoSingletonThread/>
						<Route path=path!("/Perso/CV") view=PersoCV/>
						<Route path=path!("/Perso/Hwe") view=PersoHwe/>
						<Route path=path!("/Perso/ORGECO") view=PersoORGECO/>
						<Route path=path!("/Perso/VidPHPConverter") view=PersoVidPHPConverter/>
						<Route path=path!("/Perso/CasseBrique") view=PersoCasseBrique/>
						<Route path=path!("/Perso/Wowmystats") view=PersoWowMyStats/>
					</Routes>
				</section>
			</Router>


			<footer>
				<Translate key="pageRoot_foot_design"/>" "<a class="none" href=move || mailto.get()>Hyultis</a><br/>
				<span style="font-size: 0.5em"><Translate key="pageRoot_foot"/></span>
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