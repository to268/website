use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div id=move || "page-container">
            <h1>{t!(i18n, not_found.title)}</h1>
        </div>
    }
}
