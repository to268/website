use crate::i18n::*;
use leptos::prelude::*;
use leptos_i18n::Locale;
use leptos_meta::Meta;

#[component]
pub fn OpenGraph(page_title: String) -> impl IntoView {
    let i18n = use_i18n();

    let mut available_locales =
        vec![String::from("en_US"), String::from("fr_FR")];

    let main_locale = match i18n.get_locale().as_str() {
        "en" => available_locales.remove(0),
        "fr" => available_locales.remove(1),
        _ => available_locales.remove(0),
    };
    let (alternative_locales, _) = signal(available_locales);

    let description = t!(i18n, og.description).to_html();
    let keywords = t!(i18n, og.keywords).to_html();

    view! {
        <Meta name="description" content=description />
        <Meta name="keywords" content=keywords />

        <Meta property="og:site_name" content="Tony Guillot" />
        <Meta property="og:url" content="https://tony-guillot.com" />
        <Meta property="og:title" content=page_title />
        <Meta property="og:locale" content=main_locale />
        <For
            each=alternative_locales
            key=|locale| locale.clone()
            let:locale_value
        >
            <Meta property="og:locale:alternate" content=locale_value />
        </For>
    }
}
