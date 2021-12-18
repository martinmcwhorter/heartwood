use log::info;
use sycamore::prelude::*;
use sycamore::rt::JsCast;
use web_sys::Event;
use web_sys::HtmlElement;

pub enum ButtonVariant {
    Contained,
    Outlined,
    Text,
}

pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub label: &'static str,
    pub on_click: Signal<bool>,
}

#[component(Button<G>)]
pub fn button(props: ButtonProps) -> View<G> {
    let handle_click = move |_| {
        (props.on_click.set(true));
    };

    let variant = match props.variant {
        ButtonVariant::Contained => "mdc-button--raised",
        ButtonVariant::Outlined => "mdc-button--outlined",
        ButtonVariant::Text => "",
    };

    let button_classes = format!("mdc-button {} mdc-button--touch", variant);

    view! {
        button(class=button_classes, on:click=handle_click) {
            span(class="mdc-button__ripple")
            span(class="mdc-button__touch")
            span(class="mdc-button__focus-ring")
            span(class="mdc-button__label") {
                (props.label)
            }
        }
    }
}
