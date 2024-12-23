use core::panic;

use crate::i18n::*;
use leptos::prelude::*;
//use leptos_i18n::Locale;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <a
                rel="external"
                class=move || "copyright"
                href="https://creativecommons.org/licenses/by-nc-sa/4.0/"
            >
                "CC BY-NC-SA"
            </a>
            <span class=move || "copyright">"Tony Guillot"</span>
            <SelectLang />
        </footer>
    }
}

#[component]
fn SelectLang() -> impl IntoView {
    let i18n = use_i18n();

    let locale_str = i18n.get_locale().as_str();
    let (value, set_value) = signal(locale_str.to_string());

    let changed_select = move |ev| {
        let new_value = event_target_value(&ev);

        let new_locale = Locale::find_locale(&[&new_value]);
        i18n.set_locale(new_locale);

        set_value(new_value);
    };

    let lang_pairs: Vec<(&'static str, &'static str)> =
        vec![("en", "English"), ("fr", "Français")];

    view! {
        <select class=move || "lang" on:change=changed_select>
            {lang_pairs
                .into_iter()
                .map(|pair| {
                    view! { <SelectOption value is=pair.0 name=pair.1 /> }
                })
                .collect_view()}
        </select>
    }
}

#[component]
fn SelectOption(
    is: &'static str,
    name: &'static str,
    value: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <option value=is selected=move || value() == is>
            {name}
        </option>
    }
}
