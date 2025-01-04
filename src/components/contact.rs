use crate::components::opengraph::OpenGraph;
use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <OpenGraph page_title=({ t!(i18n, contact.og_title) }).to_html() />

        <div id=move || "page-container">
            <h1>{t!(i18n, contact.title)}</h1>
            <section>
                <p>{t!(i18n, contact.description)}</p>
                <table>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/email.svg" />
                            <a rel="external" href="mailto:tony-guillot@protonmail.com">
                                "Email"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.email)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/github.svg" />
                            <a rel="external" href="https://github.com/to268">
                                "GitHub"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.github)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img
                                class=move || "icon"
                                src="/assets/icons/linkedin.svg"
                            />
                            <a
                                rel="external"
                                href="https://www.linkedin.com/in/tony-guillot"
                            >
                                "LinkedIn"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.linkedin)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/simplex.svg" />
                            <a
                                rel="external"
                                href="https://simplex.chat/contact#/?v=2-7&smp=smp%3A%2F%2Fhejn2gVIqNU6xjtGM3OwQeuk8ZEbDXVJXAlnSBJBWUA%3D%40smp16.simplex.im%2FY4Z60fYUuVStajxzTnfY-rPlCOa7O-c_%23%2F%3Fv%3D1-3%26dh%3DMCowBQYDK2VuAyEAsQ8nWLl7S94IBLBK1HV6cCrIPRnhoHBLGOVOD4Z1gTA%253D%26srv%3Dp3ktngodzi6qrf7w64mmde3syuzrv57y55hxabqcq3l5p6oi7yzze6qd.onion"
                            >
                                "SimpleX Chat"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.simplex)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/nostr.svg" />
                            <a rel="external" href="https://iris.to/tonyguillot">
                                "Nostr"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.nostr)}</td>
                    </tr>
                    <tr>
                        <td>
                            <img class=move || "icon" src="/assets/icons/discord.svg" />
                            <a
                                rel="external"
                                href="https://discordapp.com/users/357223651327868930"
                            >
                                "Discord"
                            </a>
                        </td>
                        <td>{t!(i18n, contact.topics.discord)}</td>
                    </tr>
                </table>
            </section>
        </div>
    }
}
