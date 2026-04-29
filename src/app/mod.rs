mod routes;
mod page;
mod not_found;

use std::collections::HashMap;
use i18nrs::yew::{I18nProvider, I18nProviderConfig};
pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{component, html, use_memo, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[component(App)]
pub fn app() -> Html {

  let config = use_memo((), |_| {
    let translations = HashMap::from([
      ("en", include_str!("../i18n/en/base.json")),
      ("fr", include_str!("../i18n/fr/base.json")),
    ]);

    I18nProviderConfig {
      default_language: "en".to_string(),
      translations,
      ..Default::default()
    }
  });

  html! {
    <I18nProvider ..(*config).clone()>
      <BrowserRouter>
        <div class="app">
          <Switch<AppRoute> render={switch}/>
        </div>
      </BrowserRouter>
    </I18nProvider>
  }
}