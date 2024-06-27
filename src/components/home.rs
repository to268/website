use crate::components::opengraph::OpenGraph;
use crate::i18n::*;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <OpenGraph page_title=({ t!(i18n, home.og_title) })().to_string()/>

        <div id=move || "page-container">
            <AboutSection/>
            <SkillsSection/>
        </div>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section>
            <h1>{t!(i18n, home.about_title)}</h1>
            <p>{t!(i18n, home.about_dev)}</p>
            <p>{t!(i18n, home.about_other)}</p>
            <h3>{t!(i18n, home.about_notice)}</h3>
            <img
                class=move || "display-img"
                src="/assets/img/mountains.webp"
                alt=t!(i18n, home.about_mountains_alt)
            />
            <img
                class=move || "display-img"
                src="/assets/img/land.webp"
                alt=t!(i18n, home.about_land_alt)
            />
        </section>
    }
}

#[component]
fn SkillsSection() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section>
            <h1 class=move || "h1-margin">{t!(i18n, home.skills_title)}</h1>
            <div class=move || "grid-container">
                <ProgrammingLanguagesTable/>
                <DatabasesTable/>
                <OperatingSystemsTable/>
                <MiscTable/>
            </div>
        </section>
    }
}

#[component]
fn ProgrammingLanguagesTable() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <table class=move || "grid-item">
            <thead>
                <tr>
                    <th>{t!(i18n, home.skills_languages)}</th>
                    <th>{t!(i18n, home.skills_level)}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/c.svg"/>
                        <label for="c-bar">"C"</label>
                    </td>
                    <td>
                        <progress id="c-bar" max="100" value="90"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/lua.svg"/>
                        <label for="lua-bar">"Lua"</label>
                    </td>
                    <td>
                        <progress id="lua-bar" max="100" value="85"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/rust.svg"/>
                        <label for="rust-bar">"Rust"</label>
                    </td>
                    <td>
                        <progress id="rust-bar" max="100" value="80"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/python.svg"/>
                        <label for="py-bar">"Python"</label>
                    </td>
                    <td>
                        <progress id="py-bar" max="100" value="78"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img
                            class=move || "icon"
                            src="/assets/icons/cplusplus.svg"
                        />
                        <label for="cpp-bar">"C++"</label>
                    </td>
                    <td>
                        <progress id="cpp-bar" max="100" value="76"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/java.svg"/>
                        <label for="java-bar">"Java"</label>
                    </td>
                    <td>
                        <progress id="java-bar" max="100" value="72"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/csharp.svg"/>
                        <label for="cs-bar">"C#"</label>
                    </td>
                    <td>
                        <progress id="cs-bar" max="100" value="70"></progress>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn DatabasesTable() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <table class=move || "grid-item">
            <thead>
                <tr>
                    <th>{t!(i18n, home.skills_db)}</th>
                    <th>{t!(i18n, home.skills_level)}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <img
                            class=move || "icon"
                            src="/assets/icons/postgresql.svg"
                        />
                        <label for="postgresql-bar">"PostgreSQL"</label>
                    </td>
                    <td>
                        <progress
                            id="postgresql-bar"
                            max="100"
                            value="85"
                        ></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/mongodb.svg"/>
                        <label for="mongodb-bar">"MongoDB"</label>
                    </td>
                    <td>
                        <progress id="mongodb-bar" max="100" value="75"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/database.svg"/>
                        <label for="questdb-bar">"QuestDB"</label>
                    </td>
                    <td>
                        <progress id="questdb-bar" max="100" value="70"></progress>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn OperatingSystemsTable() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <table class=move || "grid-item">
            <thead>
                <tr>
                    <th>{t!(i18n, home.skills_os)}</th>
                    <th>{t!(i18n, home.skills_level)}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/linux.svg"/>
                        <label for="linux-bar">"GNU/Linux"</label>
                    </td>
                    <td>
                        <progress id="linux-bar" max="100" value="90"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/freebsd.svg"/>
                        <label for="freebsd-bar">"FreeBSD"</label>
                    </td>
                    <td>
                        <progress id="freebsd-bar" max="100" value="82"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/windows.svg"/>
                        <label for="windows-bar">"Windows"</label>
                    </td>
                    <td>
                        <progress id="windows-bar" max="100" value="75"></progress>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn MiscTable() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <table class=move || "grid-item">
            <thead>
                <tr>
                    <th>{t!(i18n, home.skills_misc)}</th>
                    <th>{t!(i18n, home.skills_level)}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/neovim.svg"/>
                        <label for="nvim-bar">"Neovim"</label>
                    </td>
                    <td>
                        <progress id="nvim-bar" max="100" value="92"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/git.svg"/>
                        <label for="git-bar">"Git"</label>
                    </td>
                    <td>
                        <progress id="git-bar" max="100" value="88"></progress>
                    </td>
                </tr>
                <tr>
                    <td>
                        <img class=move || "icon" src="/assets/icons/docker.svg"/>
                        <label for="docker-bar">"Docker"</label>
                    </td>
                    <td>
                        <progress id="docker-bar" max="100" value="80"></progress>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}
