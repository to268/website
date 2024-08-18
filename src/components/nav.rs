use leptos::*;
use leptos_router::A;

#[component]
pub fn NavBar() -> impl IntoView {
    let (show_menu, set_show_menu) = create_signal(false);
    let toggle_menu = move |_| set_show_menu.update(|val| *val = !*val);
    let close_menu = move |_| set_show_menu.update(|val| *val = false);

    view! {
        <header>
            <h2>
                <A href="" class=move || "name nav-item" on:click=close_menu>
                    "Tony Guillot"
                </A>
            </h2>

            <div id=move || "nav-bar">
                <nav>
                    <Links set_show_menu=set_show_menu />
                </nav>
            </div>

            <div id=move || "handburger">
                <label id=move || "handburger-menu">
                    <input
                        type=move || "checkbox"
                        on:click=toggle_menu
                        prop:checked=show_menu
                    />
                </label>
                <div id=move || "menu-overlay">
                    <Links set_show_menu=set_show_menu />
                </div>
            </div>
        </header>
    }
}

#[component]
fn Links(set_show_menu: WriteSignal<bool>) -> impl IntoView {
    let close_menu = move |_| set_show_menu.update(|val| *val = false);

    view! {
        <A href="blog" class=move || "nav-item" on:click=close_menu>
            "Blog"
        </A>
        <A href="contact" class=move || "nav-item" on:click=close_menu>
            "Contact"
        </A>
    }
}
