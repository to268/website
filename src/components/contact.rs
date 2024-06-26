use crate::components::opengraph::OpenGraph;
use crate::i18n::*;
use leptos::*;
use leptos_router::A;

#[component]
pub fn ContactPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <OpenGraph page_title=({ t!(i18n, contact.og_title) })().to_string()/>

        <div id=move || "page-container">
            <h1>{t!(i18n, contact.title)}</h1>
            <section>
                <p>{t!(i18n, contact.description)}</p>
                <table>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/email.svg"/>
                            <A href="mailto:tony-guillot@protonmail.com">"Email"</A>
                        </td>
                        <td>{t!(i18n, contact.topics.email)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/github.svg"/>
                            <A href="https://github.com/to268">"GitHub"</A>
                        </td>
                        <td>{t!(i18n, contact.topics.github)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/linkedin.svg"/>
                            <A href="https://www.linkedin.com/in/tony-guillot">
                                "LinkedIn"
                            </A>
                        </td>
                        <td>{t!(i18n, contact.topics.linkedin)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/discord.svg"/>
                            <A href="https://discordapp.com/users/357223651327868930">
                                "Discord"
                            </A>
                        </td>
                        <td>{t!(i18n, contact.topics.discord)}</td>
                    </tr>
                </table>
            </section>
        </div>
    }
}
