use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, StyleAttribute};
use leptos_router::components::A;
use time::OffsetDateTime;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[component]
pub fn Perso() -> impl IntoView {

	let now = OffsetDateTime::now_utc();

	view! {
		<h2><Translate key="pagePerso_title"/></h2>

		<article>
			<div>
				<Translate key="pagePerso_desc_line1">{move || now.year() - 1989}</Translate><br/><br/>
				<span><Translate key="pagePerso_desc_line2"><A href="CV">Curriculum vitae</A></Translate><br/><br/></span>
				<Translate key="pagePerso_desc_line3"/>
			</div>
			<div style="text-align: left">
				<Translate key="pagePerso_desc_projetok"/><br/><br/>
				<ul>
					<li><A href="/">Home</A> " ":" " <Translate key="pagePerso_projet_enterprise_desc"/></li>
		            <li><A href="RustWebsite">Website in rust</A> " ":" "<Translate key="pagePerso_projet_rustwebsite_desc"/></li>
		            <li><A href="Htrace">Htrace</A>" ":" "<Translate key="pagePerso_projet_htrace_desc"/></li>
		            <li><A href="Hconfig">Hconfig</A>" ":" "<Translate key="pagePerso_projet_hconfig_desc"/></li>
		            <li><A href="HArcMut">HArcMut</A>" ":" "<Translate key="pagePerso_projet_harcmut_desc"/></li>
		            <li><A href="SingletonThread">singletonThread</A>" ":" "<Translate key="pagePerso_projet_singletonthread_desc"/></li>
				</ul>
			</div>
			<div style="text-align: left">
				<Translate key="pagePerso_desc_projetko"/><br/><br/>
				<ul>
		            <li><A href="Hwe">Hwe</A>" ":" "<Translate key="pagePerso_projet_hwe_desc"/></li>
		            <li><A href="ORGECO">OR.GE.CO 30</A>" ":" "<Translate key="pagePerso_projet_orgeco_desc"/></li>
					<li><A href="VidPHPConverter">VidPHPConverter</A>" ":" "<Translate key="pagePerso_projet_vidphpconverter_desc"/></li>
					<li><A href="CasseBrique"><Translate key="pagePersoCasseBrique_title"/></A>" ":" "<Translate key="pagePerso_projet_casebrique_desc"/></li>
					<li><A href="Wowmystats">Wowmystats</A>" ":" "<Translate key="pagePerso_projet_wowmystats_desc"/></li>
				</ul>
			</div>
		</article>
    }
}
