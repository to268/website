use crate::components::opengraph::OpenGraph;
use crate::i18n::*;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <OpenGraph page_title=({ t!(i18n, home.og_title) })().to_string()/>

        <div id=move || "page-container">
            <section>
                <h1>{t!(i18n, home.about_title)}</h1>
                <p>{t!(i18n, home.about_dev)}</p>
                <p>{t!(i18n, home.about_other)}</p>
                <p>{t!(i18n, home.about_notice)}</p>
                <img
                    class=move || "display-img"
                    src="/assets/img/mountains.webp"
                />
                <img class=move || "display-img" src="/assets/img/land.webp"/>
            </section>
        </div>
    }
}
