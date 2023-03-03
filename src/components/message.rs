use yew::prelude::*;

use crate::manager::GameMode;
use crate::Msg as GameMsg;
use crate::components::quotes::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MessageProps {
    pub message: String,
    pub is_unknown: bool,
    pub is_winner: bool,
    pub is_guessing: bool,
    pub is_hidden: bool,

    pub is_emojis_copied: bool,
    pub is_link_copied: bool,

    pub word: String,
    pub last_guess: String,
    pub game_mode: GameMode,
    pub callback: Callback<GameMsg>,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    html! {
        <div class="message">
            <div class="uppercase">
            { &props.message }
            </div>
            <div class="message-small">{
                if props.is_hidden {
                    let callback = props.callback.clone();
                    let reveal_hidden_tiles = Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        callback.emit(GameMsg::RevealHiddenTiles);
                    });
                    let callback = props.callback.clone();
                    let reset_game = Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        callback.emit(GameMsg::ResetGame);
                    });

                    html! {
                        <>
                            <a class="link" href={"javascript:void(0)"} onclick={reset_game}>
                                {"Kokeile ratkaista"}
                            </a>
                            {" | "}
                            <a class="link" href={"javascript:void(0)"} onclick={reveal_hidden_tiles}>
                                {"Paljasta"}
                            </a>
                        </>
                    }
                } else if !props.is_guessing {
                    html! {
                        <SubMessage
                            is_winner={props.is_winner}
                            is_emojis_copied={props.is_emojis_copied}
                            is_link_copied={props.is_link_copied}
                            word={props.word.clone()}
                            game_mode={props.game_mode}
                            callback={props.callback.clone()}
                        />
                    }
                } else if props.is_guessing && props.is_unknown {
                    html! {
                        { "Kokeile jotain muuta!" }
                    }
                } else {
                    html! {}
                }
            }
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct SubMessageProps {
    pub is_winner: bool,
    pub is_emojis_copied: bool,
    pub is_link_copied: bool,
    pub word: String,
    pub game_mode: GameMode,
    pub callback: Callback<GameMsg>,
}

#[function_component(SubMessage)]
fn sub_message(props: &SubMessageProps) -> Html {
    html! {
        <>
            <div class="padded">
            {format!("{} - ", get_quote(&props.word.clone() as &str))}
            <a class="link" href={format!("{}", get_url(&props.word.clone() as &str))}
                            target="_blank">{ "Lue lisää!" }
            </a>
            </div>
        </>
    }
}
