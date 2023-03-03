use yew::prelude::*;

use crate::manager::{GameMode, Theme, WordList};
use crate::Msg;

const CHANGELOG_URL: &str = "https://github.com/mirokuru/maiuli/blob/master/CHANGELOG.md";
const VERSION: &str = "v1.0";

macro_rules! onmousedown {
    ( $cb:ident, $msg:expr ) => {{
        let $cb = $cb.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            $cb.emit($msg);
        })
    }};
}

#[derive(Properties, Clone, PartialEq)]
pub struct HelpModalProps {
    pub theme: Theme,
    pub callback: Callback<Msg>,
}

#[function_component(HelpModal)]
pub fn help_modal(props: &HelpModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_help = onmousedown!(callback, Msg::ToggleHelp);

    html! {
        <div class="modal">
            <span onmousedown={toggle_help} class="modal-close">{"✖"}</span>
            <p>{"Arvaa kätketty sana kuudella yrityksellä."}</p>
            <p>{"Jokaisen yrityksen jälkeen arvatut kirjaimet vaihtavat väriään."}</p>

            <div class="row-5 example">
                <div class={classes!("tile", "correct")}>{"K"}</div>
                <div class={classes!("tile", "absent")}>{"O"}</div>
                <div class={classes!("tile", "present")}>{"I"}</div>
                <div class={classes!("tile", "absent")}>{"R"}</div>
                <div class={classes!("tile", "absent")}>{"A"}</div>
            </div>

            <p>
                {
                    html! {
                        if props.theme == Theme::Colorblind {
                            <span class="present">{"Sininen"}</span>
                        } else {
                            <span class="present">{"Keltainen"}</span>
                        }
                    }
                }
                {": kirjain löytyy kätketystä sanasta, mutta on arvauksessa väärällä paikalla."}
            </p>
            <p>
                {
                    html! {
                        if props.theme == Theme::Colorblind {
                            <span class="correct">{"Oranssi"}</span>
                        } else {
                            <span class="correct">{"Pinkki"}</span>
                        }
                    }
                }
                {": kirjain on arvauksessa oikealla paikalla."}
            </p>
            <p><span class="absent">{"Harmaa"}</span>{": kirjain ei löydy sanasta."}</p>

            <p>
                {"Arvattaviin sanoihin käytetyn sanalistan vaikeusasteen voi valita asetuksista. Sanalistojen pohjana on käytetty
                Kotimaisten kielten keskuksen (Kotus) julkaiseman "}
                <a class="link" href="https://creativecommons.org/licenses/by/3.0/deed.fi" target="_blank">{"\"CC Nimeä 3.0 Muokkaamaton\""}</a>
                {"-lisensoidun nykysuomen sanalistan sanoja."}
            </p>

            <p><b>{"Tavallinen"}</b>{" lista sisältää perusmuotoisia sanoja, jotka saattavat liittyä Vasemmistoliiton ehdokkaaseen Mai Kivelään jotenkin 😊"}</p>
            <p><b>{"Helppo"}</b>{" lista on tavallisesta hieman helpotettu versio, jossa jäljellä ovat hieman yleisemmät sanat."}</p>
            <p>
                {"Sanat ovat yleensä perusmuodossa, mutta eivät välttämättä täysin pelkkää kirjakieltä. Yhdyssanojakin on seassa."}
            </p>
            <p>
                {"Maiuliketjussa jos arvaat maiulin, on se suoraan ensimmäinen arvaus seuraavaan peliin. Näin joudut sopeutumaan vaihtuviin alkuarvauksiin, ja peli on hieman vaikeampi."}
            </p>
            <p>
                {"Maiuli pohjautuu suoraan Jaakko Husson julkaisemaan MIT-lisensoituun "}
                <a class="link" href="https://github.com/Cadiac/sanuli" target="_blank">{"Sanuli"}</a>
                {"-peliin."}
            </p>
            <p>
                {"Muokkaukset on tehnyt Miro Kuru, ja "}<a class="link" href="https://github.com/mirokuru/maiuli" target="_blank">{"lähdekoodi"}</a>
                {" on saatavilla. "}
            </p>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MenuModalProps {
    pub callback: Callback<Msg>,
    pub word_length: usize,
    pub game_mode: GameMode,
    pub current_word_list: WordList,
    pub allow_profanities: bool,
    pub theme: Theme,

    pub max_streak: usize,
    pub total_played: usize,
    pub total_solved: usize,
}

#[function_component(MenuModal)]
pub fn menu_modal(props: &MenuModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_menu = onmousedown!(callback, Msg::ToggleMenu);

    let change_word_length_4 = onmousedown!(callback, Msg::ChangeWordLength(4));
    let change_word_length_5 = onmousedown!(callback, Msg::ChangeWordLength(5));
    let change_word_length_6 = onmousedown!(callback, Msg::ChangeWordLength(6));
    let change_word_length_7 = onmousedown!(callback, Msg::ChangeWordLength(7));

    let change_game_mode_classic = onmousedown!(callback, Msg::ChangeGameMode(GameMode::Classic));
    let change_game_mode_relay = onmousedown!(callback, Msg::ChangeGameMode(GameMode::Relay));

    let change_word_list_easy = onmousedown!(callback, Msg::ChangeWordList(WordList::Easy));
    let change_word_list_common = onmousedown!(callback, Msg::ChangeWordList(WordList::Common));

    let change_theme_dark = onmousedown!(callback, Msg::ChangeTheme(Theme::Dark));
    let change_theme_colorblind = onmousedown!(callback, Msg::ChangeTheme(Theme::Colorblind));

    let is_hide_settings = matches!(props.game_mode, GameMode::DailyWord(_) | GameMode::Shared);

    html! {
        <div class="modal">
            <span onmousedown={toggle_menu} class="modal-close">{"✖"}</span>
            {if !is_hide_settings {
                html! {
                    <>
                        <div>
                            <label class="label">{"Maiulien pituus:"}</label>
                            <div class="select-container">
                                <button class={classes!("select", (props.word_length == 4).then(|| Some("select-active")))}
                                    onmousedown={change_word_length_4}>
                                    {"4 merkkiä"}
                                </button>
                                <button class={classes!("select", (props.word_length == 5).then(|| Some("select-active")))}
                                    onmousedown={change_word_length_5}>
                                    {"5 merkkiä"}
                                </button>
                                <button class={classes!("select", (props.word_length == 6).then(|| Some("select-active")))}
                                    onmousedown={change_word_length_6}>
                                    {"6 merkkiä"}
                                </button>
                                <button class={classes!("select", (props.word_length == 7).then(|| Some("select-active")))}
                                    onmousedown={change_word_length_7}>
                                    {"7 merkkiä"}
                                </button>
                            </div>
                        </div>
                        <div>
                            <label class="label">{"Maiulista:"}</label>
                            <div class="select-container">
                                <button class={classes!("select", (props.current_word_list == WordList::Easy).then(|| Some("select-active")))}
                                    onmousedown={change_word_list_easy}>
                                    {"Helppo"}
                                </button>
                                <button class={classes!("select", (props.current_word_list == WordList::Common).then(|| Some("select-active")))}
                                    onmousedown={change_word_list_common}>
                                    {"Tavallinen"}
                                </button>
                            </div>
                        </div>
                    </>
                }
            } else {
                html! {}
            }}
            <div>
                <label class="label">{"Pelimuoto:"}</label>
                <div class="select-container">
                    <button class={classes!("select", (props.game_mode == GameMode::Classic).then(|| Some("select-active")))}
                        onmousedown={change_game_mode_classic}>
                        {"Peruspeli"}
                    </button>
                    <button class={classes!("select", (props.game_mode == GameMode::Relay).then(|| Some("select-active")))}
                        onmousedown={change_game_mode_relay}>
                        {"Maiuliketju"}
                    </button>
                </div>
            </div>
            <div>
                <label class="label">{"Omat tilastosi:"}</label>
                <ul>
                    <li class="statistics">{format!("Pisin putki: {}", props.max_streak)}</li>
                    <li class="statistics">{format!("Pelatut maiulit: {}", props.total_played)}</li>
                    <li class="statistics">{format!("Ratkaistut maiulit: {}", props.total_solved)}</li>
                </ul>
            </div>
            <div>
                <label class="label">{"Teema:"}</label>
                <div class="select-container">
                    <button class={classes!("select", (props.theme == Theme::Dark).then(|| Some("select-active")))}
                        onmousedown={change_theme_dark}>
                        {"Oletus"}
                    </button>
                    <button class={classes!("select", (props.theme == Theme::Colorblind).then(|| Some("select-active")))}
                        onmousedown={change_theme_colorblind}>
                        {"Värisokeille"}
                    </button>
                </div>
            </div>
            <div class="version">
                <a class="version" href={CHANGELOG_URL} target="_blank">{ VERSION }</a>
            </div>
        </div>
    }
}
