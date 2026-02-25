use std::ops::Not;
use yew::{html, Html};
use yew_router::prelude::Redirect;
use yew_router::{Routable, Switch};
use crate::app::{Home};
use crate::app::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/:locale/*")]
  LocaleRoot,
  #[at("/:locale")]
  HomeLocale { locale: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum LocaleRoot {
  #[at("/:locale")]
  Home { locale: String },
  #[not_found]
  #[at("/:locale/404")]
  NotFound,
}

pub fn switch_locale(route: LocaleRoot) -> Html {
  match route {
    LocaleRoot::Home { locale } => html! {<Home locale={locale} />},
    // LocaleRoot::NotFound => html! { <Redirect<AppRoute> to={AppRoute::NotFound}/>}
    LocaleRoot::NotFound => html! { <NotFound />}
  }
}

pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::NotFound => html! {<NotFound/>},
    AppRoute::LocaleRoot => html! {<Switch<LocaleRoot> render={switch_locale}/>},
    AppRoute::HomeLocale { locale } => html! {<Home locale={locale} />},
  }
}