use sycamore::prelude::*;

pub enum ButtonVariant {
    Contained,
    Outlined,
    Text
}

pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub label: &'static str
}


#[component(Button<G>)]
pub fn button(props: ButtonProps) -> View<G> {

    let variant = match props.variant {
        ButtonVariant::Contained => "mdc-button--raised",
        ButtonVariant::Outlined => "mdc-button--outlined",
        ButtonVariant::Text => ""
    };

    let button_classes = format!("mdc-button {} mdc-button--touch ", variant);

    view! {
        button(class=button_classes) {
            span(class="mdc-button__ripple")
            span(class="mdc-button__touch")
            span(class="mdc-button__label") {
                (props.label)
            }
        }
    }
}
