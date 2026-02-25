use i18nrs::yew::use_translation;
use yew::{function_component, html, use_effect_with, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
  #[prop_or(String::from("en"))]
  pub(crate) locale: String
}
#[function_component(Home)]
pub(crate) fn home(props: &Props) -> Html {
  let (i18n, set_language) = use_translation();

  use_effect_with(props.locale.clone(), move |locale| {
    set_language.emit(locale.clone());
    || ()
  });

  html! {
    <div class="Home">
      <div class="yewi-container">
        <header class="yewi-header">
          <h1 class="yewi-title">{i18n.t("home.title")}</h1>
          <p class="yewi-tagline">{i18n.t("home.tagline")}</p>
        </header>

        <main class="yewi-main">
          <p class="yewi-description">
            {i18n.t("home.description")}
          </p>

          <div class="yewi-grid">
            <div class="yewi-card">
              <h3>{i18n.t("home.cards.getStarted.title")}</h3>
              <code>{"yewi new my-app"}</code>
            </div>
            <div class="yewi-card">
              <h3>{i18n.t("home.cards.addComponent.title")}</h3>
              <code>{"yewi add button"}</code>
            </div>
            <div class="yewi-card">
              <h3>{i18n.t("home.cards.documentation.title")}</h3>
              <a href="https://yewi.pages.dev" target="_blank" rel="noopener noreferrer">
                {"yewi.pages.dev"}
              </a>
            </div>
            <div class="yewi-card">
              <h3>{i18n.t("home.cards.source.title")}</h3>
              <a href="https://github.com/Emii-lia/yewi-kit" target="_blank" rel="noopener noreferrer">
                {"github.com"}
              </a>
            </div>
          </div>
        </main>
      </div>
    </div>
  }
}