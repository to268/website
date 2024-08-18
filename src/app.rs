use crate::components::{
    blog::BlogPage, contact::ContactPage, footer::Footer, home::HomePage,
    nav::NavBar,
};
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css" />
        // FIXME: This is a temporary fix to have the scroll on animation working
        <Stylesheet id="scroll-animation" href="scroll-animation.css" />

        <Title text="Tony Guillot" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <NavBar />
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/blog" view=BlogPage />
                    <Route path="/contact" view=ContactPage />
                    <RouteEn />
                    <RouteFr />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

// Theses following routes are used for SEO purposes

#[component(transparent)]
fn RouteEn() -> impl IntoView {
    view! {
        <Route
            path="/en"
            view=move || {
                use_i18n().set_locale(Locale::en);
                view! { <HomePage /> }
            }
        />
    }
}

#[component(transparent)]
fn RouteFr() -> impl IntoView {
    view! {
        <Route
            path="/fr"
            view=move || {
                use_i18n().set_locale(Locale::fr);
                view! { <HomePage /> }
            }
        />
    }
}
