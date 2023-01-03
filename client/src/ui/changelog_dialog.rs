// SPDX-FileCopyrightText: 2021 Softbear, Inc.
// SPDX-License-Identifier: AGPL-3.0-or-later

use yew::{function_component, html, Html};
use yew_frontend::component::link::Link;
use yew_frontend::dialog::dialog::Dialog;
use yew_frontend::frontend::use_game_id;
use yew_frontend::translation::{use_translation, Translation};

#[function_component(ChangelogDialog)]
pub fn changelog_dialog() -> Html {
    let t = use_translation();
    let game_id = use_game_id();
    html! {
        <Dialog title={t.changelog_title(game_id)}>
            <p>{"Warning: This changelog may not always be fully up to date"}</p>
            {changelog_2023()}
        </Dialog>
    }
}

#[inline(never)]
fn changelog_2023() -> Html { //<li>{""}</li>
    html! {
        <>
            <h2>{"2023"}</h2>

            <h3>{"3 Jan '23"}</h3>

            <ul>
                <li>{"Nerfed minebombing"}</li>
                <li>{"Ekranoplan now hovers when moving, according to velocity (you're welcome dyy)"}</li>
                <li>{"Changed tropics to have more greenery"}</li>
                <li>{"Updated team size to maximum of three at all times (down from 6)"}</li>
                <li>{"Dropped coins now transfer 100% of score"}</li>
                <li>{"Disabled anti-air particles on Drone"}</li>
                <li>{"Enabled Pirate Ship"}</li>
                <li>{""}</li>
            </ul>

            <h3>{"2 Jan '23"}</h3>

            <ul>
                <li>{"Set up public server with new mk48 update"}</li>
                <li>{"Ported over custom Drone & Ekranoplan"}</li>
                <li>{"Added normal maps to Drone & Ekranoplan"}</li>
                <li>{"Removed rust from Ekranoplan's sprite"}</li>
                <li>{"Changed team chat colour to Mk48 Blue"}</li>
                <li>{"Tinkered with some things"}</li>
            </ul>

        </>
    }
}
