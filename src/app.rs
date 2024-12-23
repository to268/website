use crate::components::{
    blog::BlogPage, contact::ContactPage, footer::Footer, home::HomePage,
    nav::NavBar,
};
use crate::i18n::{I18nContextProvider, Locale};
use leptos::prelude::*;
use leptos_i18n_router::I18nRoute;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    nested_router::Outlet,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css" />
        // FIXME: This is a temporary fix to have the scroll on animation working
        <Stylesheet id="scroll-animation" href="scroll-animation.css" />

        <Title text="Tony Guillot" />

        <I18nContextProvider>
            <Router>
                <NavBar />
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <I18nRoute<Locale, _, _> view=Outlet>
                            <Route path=path!("") view=HomePage />
                            <Route path=path!("blog") view=BlogPage />
                            <Route path=path!("contact") view=ContactPage />
                        </I18nRoute<Locale, _, _>>
                    </Routes>
                </main>
                <Footer />
            </Router>
        </I18nContextProvider>
    }
}
