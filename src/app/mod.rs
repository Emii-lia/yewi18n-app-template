mod routes;
mod page;
mod not_found;

use std::collections::HashMap;
use i18nrs::yew::{I18nProvider, I18nProviderConfig};
pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
  let translations = HashMap::from([
    ("en", include_str!("../i18n/en/base.json")),
    ("fr", include_str!("../i18n/fr/base.json")),
  ]);

  let config = I18nProviderConfig {
    default_language: "en".to_string(),
    translations,
    ..Default::default()
  };

  html! {
    <BrowserRouter>
      <I18nProvider ..config>
        <div class="app">
          <Switch<AppRoute> render={switch}/>
        </div>
      </I18nProvider>
    </BrowserRouter>
  }
}