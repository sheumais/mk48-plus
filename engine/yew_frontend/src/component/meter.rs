// SPDX-FileCopyrightText: 2021 Softbear, Inc.
// SPDX-License-Identifier: AGPL-3.0-or-later

use stylist::yew::styled_component;
use yew::{html, Children, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct MeterProps {
    pub children: Children,
    #[prop_or(0x0084b1)]
    pub color: u32,
    /// 0 to 1.
    pub value: f32,
}

#[styled_component(Meter)]
pub fn meter(props: &MeterProps) -> Html {
    let div_css_class = css!(
        r#"
        background-color: #3f3333;
        border: 2px solid #FFF;
        border-radius: 2em;
        box-sizing: border-box;
        color: white;
        font-weight: bold;
        height: min-content;
        min-height: 1.1em;
        overflow: hidden;
        padding: 0.5em;
        text-align: center;
        transition: background-size 0.5s;
        user-select: none;
        width: 100%;
    "#
    );

    let background_color: u32 = 0x3f3333;
    let percentage = (props.value.clamp(0.0, 1.0) * 100.0).round();
    let background_size = (percentage.max(1.0) * 100.0).round();

    let style = format!("background: linear-gradient(90deg, #75AAFF7F 0%, #75AAFF7F 1%, #{:06x} 1%, #{:06x} 100%); background-origin: border-box; background-size: {}%;", background_color, background_color, background_size);

    html! {
        <div class={div_css_class} style={style}>
            {props.children.clone()}
        </div>
    }
}
