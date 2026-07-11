use leptos::prelude::{ClassAttribute, RenderHtml};
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use leptos_router::components::A;
use time::OffsetDateTime;
use crate::front::utils::translate::Translate;

/// Renders the home page of your application.
#[island]
pub fn Perso() -> impl IntoView {

	let now = OffsetDateTime::now_utc();

	view! {
		<h2><Translate key="pagePerso_title"/></h2>

		<article>
			<div>
				<Translate key="pagePerso_desc_line1">{move || now.year() - 1989}</Translate><br/><br/>

				<img src="img/curriculum-vitae.png" class="element_perso_img_cv"/><br/>
				<span><Translate key="pagePerso_desc_line2"><A href="CV">Curriculum vitae</A></Translate><br/><br/>

				<img src="img/apprentissage-en-ligne.png" class="element_perso_img_firstbloc"/><br/></span>
				<Translate key="pagePerso_desc_line3"/>
			</div>
			<div class="element_align_left">
				<img src="img/valide.png" class="element_perso_img_other"/> <Translate key="pagePerso_desc_projetok"/><br/><br/>
				<ul>
					<li><A href="/">Home</A> " ":" " <Translate key="pagePerso_projet_enterprise_desc"/></li>
		            <li><A href="RustWebsite">Website in rust</A> " ":" "<Translate key="pagePerso_projet_rustwebsite_desc"/></li>
		            <li><A href="Htrace">Htrace</A>" ":" "<Translate key="pagePerso_projet_htrace_desc"/></li>
		            <li><A href="Hconfig">Hconfig</A>" ":" "<Translate key="pagePerso_projet_hconfig_desc"/></li>
		            <li><A href="HArcMut">HArcMut</A>" ":" "<Translate key="pagePerso_projet_harcmut_desc"/></li>
		            <li><A href="SingletonThread">singletonThread</A>" ":" "<Translate key="pagePerso_projet_singletonthread_desc"/></li>
		            <li><A href="WebHome">WebHome</A>" ":" "<Translate key="pagePerso_projet_webhome_desc"/></li>
				</ul>
			</div>
			<div class="element_align_left">
				<img src="img/diminue.png" class="element_perso_img_other"/> <Translate key="pagePerso_desc_projetko"/><br/><br/>
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
