use sycamore::prelude::*;
use web_sys::Event;

#[derive(Clone)]
pub enum ButtonVariant {
    Contained,
    Outlined,
    Text,
}

pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub label: ReadSignal<String>,
    pub on_click: Box<dyn Fn(Event)>,
    pub disabled: ReadSignal<bool>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Contained,
            label: Signal::new("".to_string()).handle(),
            on_click: Box::new(|_| {}),
            disabled: Signal::new(false).handle(),
        }
    }
}

impl ButtonProps {
    pub fn label_from_str(mut self, label: &str) -> Self {
        let label = Signal::new(label.to_string());
        self.label = label.clone().handle();
        return self;
    }

    pub fn label(mut self, label: ReadSignal<String>) -> Self {
        self.label = label.clone();
        return self;
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn(Event)>) -> Self {
        self.on_click = on_click;
        return self;
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        return self;
    }

    pub fn disabled(mut self, disabled: ReadSignal<bool>) -> Self {
        self.disabled = disabled;
        return self;
    }
}

#[component(Button<G>)]
pub fn button(props: ButtonProps) -> View<G> {
    let variant = match props.variant {
        ButtonVariant::Contained => "mdc-button--raised",
        ButtonVariant::Outlined => "mdc-button--outlined",
        ButtonVariant::Text => "",
    };

    let button_classes = format!("mdc-button {} mdc-button--touch", variant);

    view! {
        button(class=button_classes, on:click=props.on_click, disabled=*props.disabled.get()) {
            span(class="mdc-button__ripple")
            span(class="mdc-button__touch")
            span(class="mdc-button__focus-ring")
            span(class="mdc-button__label") {
                (*props.label.get())
            }
        }
    }
}
