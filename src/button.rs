use log::info;
use sycamore::prelude::*;
use sycamore::rt::JsCast;
use web_sys::Event;
use web_sys::HtmlElement;

#[derive(Clone)]
pub enum ButtonVariant {
    Contained,
    Outlined,
    Text,
}

#[derive(Clone)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub label: ReadSignal<String>,
    pub on_click: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Contained,
            label: Signal::new("".to_string()).handle(),
            on_click: Signal::new(false),
            disabled: Signal::new(false),
        }
    }
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
        button(class=button_classes, on:click=handle_click, disabled=*props.disabled.get()) {
            span(class="mdc-button__ripple")
            span(class="mdc-button__touch")
            span(class="mdc-button__focus-ring")
            span(class="mdc-button__label") {
                (*props.label.get())
            }
        }
    }
}
