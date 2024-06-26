use crate::components::opengraph::OpenGraph;
use crate::i18n::*;
use leptos::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <OpenGraph page_title=({ t!(i18n, blog.og_title) })().to_string()/>

        <div id=move || "page-container">
            <h1>{t!(i18n, blog.title)}</h1>
            <section>
                <h1>{t!(i18n, blog.section_title)} " \u{1f61e}"</h1>
                <p>{t!(i18n, blog.section_paragraph)} "\u{2122}"</p>
            </section>
        </div>
    }
}
