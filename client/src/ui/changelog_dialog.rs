// SPDX-FileCopyrightText: 2021 Softbear, Inc.
// SPDX-License-Identifier: AGPL-3.0-or-later

use yew::{function_component, html, Html};
//use yew_frontend::component::link::Link;
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

            <h3>{"8 Jan '23"}</h3>

            <ul>
                <li>{"Added the Spitfire"}</li>
                <li>{"Changed Bot names to Capital Cities"}</li>
                <li>{"SAMs now target and damage all airborne entities"}</li>
                <li>{"Aeroplanes no-longer collide with launched airplanes/helicopters"}</li>
                <li>{"Increased height at which non-aircraft can launch weapons"}</li>
                <li>{"Moved Ekranoplan to level 6"}</li>
                <li>{"Removed bearing from status overlay because it was stupid"}</li>
                <li>{"Stopped spillover of land into arctic"}</li>
                <li>{"Re-added speed limitations to ships"}</li>
                <li>{"Increased noise from aeroplanes"}</li>
            </ul>

            <h3>{"7 Jan '23"}</h3>

            <ul>
                <li>{"Added Japanese Aircraft Carrier Kaga"}</li>
                <li>{"Added two new trees (which took way too long because I got side tracked)"}</li>
                <li>{"Slightly rebalanced mines and depth charges"}</li>
                <li>{"Reduced point multiplier from 5x to 3x to more accurately assess ship balance in the real game"}</li>
                <li>{"Added a twin gun to the Ekranoplan"}</li>
                <li>{"Added the legendary Zudredger!"}</li>
            </ul>

            <h3>{"4 Jan '23"}</h3>

            <ul>
                <li>{"Added PBY Catalina as a playable aircraft"}</li>
                <li>{"Increased bounce-back when ramming terrain as Ekranoplan and Aeroplanes"}</li>
                <li>{"Fixed Drone"}</li>
                <li>{"Added altitude indicator to status overlay"}</li>
                <li>{"Shells fired from an aeroplane now can hit targets at all altitudes"}</li>
                <li>{"Reduced mine reload time"}</li>
                <li>{"Removed mine attraction to nearby boats"}</li>
                <li>{"Buffed mine damage"}</li>
                <li>{"Dramatically increased mine stealth"}</li>
                <li>{"Removed ability to upgrade as a Drone"}</li>
            </ul>

            <h3>{"3 Jan '23"}</h3>

            <ul>
                <li>{"Nerfed minebombing"}</li>
                <li>{"Ekranoplan now hovers when moving, according to velocity (you're welcome dyy)"}</li>
                <li>{"Changed tropics to have more greenery"}</li>
                <li>{"Updated team size to maximum of three at all times (down from 6)"}</li>
                <li>{"Dropped coins now transfer 100% of score"}</li>
                <li>{"Disabled anti-air particles on Drone"}</li>
                <li>{"Enabled Pirate Ship"}</li>
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