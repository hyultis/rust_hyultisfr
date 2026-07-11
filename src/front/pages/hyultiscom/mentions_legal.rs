use leptos::prelude::{ClassAttribute, RenderHtml};
use leptos::{island, view, IntoView};
use leptos::prelude::{ElementChild};
use crate::front::utils::dataHide::DataHideMail;
use crate::front::utils::translate::Translate;

#[island]
pub fn MentionsLegal() -> impl IntoView {
	view! {
		<h2><Translate key="mentionslegal_title"/></h2>

		<article>
			<div class="element_hide"></div>
			<div class="element_css_center"><br/>
				"Ce site est un site personnel de présentation de projets de développement et de jeux vidéo."<br/>
				"Les ventes éventuelles de jeux sont réalisées exclusivement via des plateformes tierces, notamment Steam. Ce site ne propose pas de vente directe."
				<br/><br/>
				<h2>Éditeur du site</h2>
				<ul>
					<li>"Hyultis"</li>
					<li>"Contact : "<DataHideMail mailTo="hyultis@gmail.com".to_string() text=None/></li>
				</ul><br/><br/>
				<h2>Hébergement</h2>
				"Le Site est hébergé par la société OVH SAS, situé 2 rue Kellermann, 59100 Roubaix, France."
			</div>
		</article>
    }
}
