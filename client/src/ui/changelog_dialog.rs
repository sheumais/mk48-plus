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

            <h3>{"5 Feb '23"}</h3>
            <ul>
                <li>{"Added the Liaoning Aircraft Carrier"}</li>
                <li>{"Moved the Clemenceau to level 8"}</li>
            </ul>

            <h3>{"3 Jan '23"}</h3>
            <ul>
                <li>{"Changed UI"}</li>
                <li>{"Changed background shader to show more greenery"}</li>
                <li>{"Fixed bug where bombs on the B-2 were visible"}</li>
            </ul>

            <h3>{"31 Jan '23"}</h3>

            <ul>
                <li>{"Clicking names on your team now promotes them to leader"}</li>
                <li>{"Weapon triangle indicators now scale their size with damage"}</li>
                <li>{"Started standardising naming convention across the UI"}</li>
            </ul>

            <h3>{"30 Jan '23"}</h3>

            <ul>
                <li>{"Added Horns to some ships (Press H)"}</li>
                <li>{"Increased the damage the Titanic takes from Ice"}</li>
                <li>{"Smoothened Ice in shader"}</li>
                <li>{"Changed the submarine circle function"}</li>
            </ul>

            <h3>{"28 Jan '23"}</h3>

            <ul>
                <li>{"Removed Titanic Easter Egg due to confusion"}</li>
                <li>{"Changed sounds on Aeroplanes"}</li>
                <li>{"Added scrollbar to chat"}</li>
                <li>{"Added scrollbar styling for Firefox users"}</li>
            </ul>

            <h3>{"26 Jan '23"}</h3>

            <ul>
                <li>{"Added B2 Spirit"}</li>
                <li>{"Added Nexar Vindicator"}</li>
                <li>{"Changed arctic terrain"}</li>
                <li>{"Made weapon sprites smaller"}</li>
                <li>{"Fixed bug where bot deployables wouldn't disappear"}</li>
                <li>{"Changed background shader (again..)"}</li>
                <li>{"Rebalanced Aeroplanes"}</li>
                <li>{"Changed the spawning algorithm slightly"}</li>
            </ul>

            <h3>{"24 Jan '23"}</h3>

            <ul>
                <li>{"Added Apache Helicopter"}</li>
                <li>{"Fixed Background Shader"}</li>
                <li>{"Increased bot aggression and count"}</li>
                <li>{"Decreased maximum acceleration of boats and aeroplanes to 5 gs"}</li>
                <li>{"Added epic particles when you shoot shells"}</li>
                <li>{"Reduced size of triangle indicators"}</li>
            </ul>

            <h3>{"23 Jan '23"}</h3>

            <ul>
                <li>{"Added the J-20"}</li>
                <li>{"Made improvements to the background shader"}</li>
                <li>{"Removed the emissions from the fourth funnel on the Titanic"}</li>
                <li>{"Increased bot aggression and minimum level target"}</li>
                <li>{"Increased minimum number of bots"}</li>
                <li>{"Decreased world size"}</li>
            </ul>

            <h3>{"21 Jan '23"}</h3>

            <ul>
                <li>{"Watched Titanic, got teary, added Titanic"}</li>
                <li>{"Added the X-wing"}</li>
                <li>{"Rebalanced the aeroplanes"}</li>
                <li>{"Gave the Titanic a passive oil spawning ability, exactly like a tier 3 rig"}</li>
            </ul>

            <h3>{"20 Jan '23"}</h3>

            <ul>
                <li>{"Added the Ticonderoga"}</li>
                <li>{"Moved the Kirov to level 9"}</li>
                <li>{"Added the Chinook"}</li>
                <li>{"Had a meeting with Kuat Drive Yards"}</li>
                <li>{"Added the Star Destroyer"}</li>
                <li>{"Big thanks to Kuat Drive Yards for producing them at just 135,000,000 credits per unit!"}</li>
                <li>{"Reduced the missile damage of the F-35"}</li>
            </ul>

            <h3>{"18 Jan '23"}</h3>

            <ul>
                <li>{"Fixed a crash bug"}</li>
                <li>{"Took a flight over the NW coast of Mexico"}</li>
                <li>{"Fine-tuned the Abrams Tank"}</li>
                <li>{"Changed the Admin Chat colour (again)"}</li>
            </ul>

            <h3>{"17 Jan '23"}</h3>

            <ul>
                <li>{"Added M1 Abrams Tank"}</li>
                <li>{"Changed level bar to always show highest level"}</li>
                <li>{"Changed Border"}</li>
            </ul>

            <h3>{"15 Jan '23"}</h3>

            <ul>
                <li>{"Added Virginia-class Submarine"}</li>
                <li>{"Fixed Aeroplane Collision"}</li>
                <li>{"Added F-35 Sounds"}</li>
                <li>{"Fixed Catalina"}</li>
                <li>{"Changed Team Size"}</li>
            </ul>

            <h3>{"14 Jan '23"}</h3>

            <ul>
                <li>{"Fixed insane point glitch"}</li>
                <li>{"Re-adjusted aeroplane speeds"}</li>
                <li>{"Improved F-35"}</li>
            </ul>

            <h3>{"13 Jan '23"}</h3>

            <ul>
                <li>{"Added a Landing Ship to compensate the Tank"}</li>
                <li>{"Added the F-35"}</li>
                <li>{"Reduced all aeroplane speeds by 25%"}</li>
                <li>{"Changed Admin Chat colour to Sheumais Yellow (Patent pending)"}</li>
                <li>{"Fixed teammates being killed by anti-air"}</li>
                <li>{"Slightly tweaked ocean colours"}</li>
                <li>{"Attempted to stop bots accumulating millions of points"}</li>
                <li>{"Said a couple prayers"}</li>
            </ul>

            <h3>{"11 Jan '23"}</h3>

            <ul>
                <li>{"Added the M4 Sherman Tank"}</li>
                <li>{"Gave Mac users the ability to copy text from the chat"}</li>
                <li>{"Changed how much noise ships make"}</li>
                <li>{"Boats will now shoot down player Aeroplanes with their anti-air defences"}</li>
                <li>{"Added two new death messages"}</li>
            </ul>

            <h3>{"10 Jan '23"}</h3>

            <ul>
                <li>{"Changed trail of Aeroplanes to something more suiting and unique"}</li>
            </ul>

            <h3>{"9 Jan '23"}</h3>
            
            <ul>
                <li>{"Changed status overlay and level meter"}</li>
                <li>{"Added aeroplane sounds to the playable aeroplanes"}</li>
                <li>{"Removed ridiculously annoying coin sound when losing contact of dropped items"}</li>
                <li>{"Increased border radius to 5000 (from 4000)"}</li>
                <li>{"Changed border colour for literally no reason"}</li>
                <li>{"Changed tab title"}</li>
                <li>{"Added Super Oil Rigs to the upper arctic (they found TONS of oil!!1!)"}</li>
                <li>{"Gave the Kolkata one helicopter"}</li>
                <li>{"Removed username above your own ship"}</li>
                <li>{"Changed some things about the UI"}</li>
                <li>{"The Spitfire can no-longer fire backwards"}</li>
                <li>{"Gave level 1s the ability to reverse with the mouse"}</li>
                <li>{"Deployed airplanes will now use anti-air missiles against player aeroplanes"}</li>
            </ul>

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
                <li>{"Gave Zudredger a shovel"}</li>
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
