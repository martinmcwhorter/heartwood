use sycamore::prelude::*;
use sycamore::rt::JsCast;
use uuid::Uuid;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

pub struct TextFieldProps {
    pub label: &'static str,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
}

impl Default for TextFieldProps {
    fn default() -> Self {
        Self {
            label: "",
            value: Signal::new("".to_string()),
            disabled: Signal::new(false),
        }
    }
}

#[component(TextField<G>)]
pub fn text_field(props: TextFieldProps) -> View<G> {
    let value = props.value.clone();
    let disabled = props.disabled.clone();

    let on_input = cloned!((value) => move |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        value.set(target.value());
    });

    let focus = Signal::new(false);
    let focus_handler = cloned!((focus) => move |_| {
        focus.set(true);
    });
    let blur_handler = cloned!((focus) => move |_| {
        focus.set(false);
    });

    let id = Uuid::new_v4();

    let line_classes = cloned!((focus) => move || {
        format!("mdc-line-ripple{}",
        if *focus.get() { " mdc-line-ripple--active" } else { "" })
    });

    let floating_label_classes = cloned!((focus, value) => move || {
        format!("mdc-floating-label{}",
        if *focus.get() || *value.get() != "" {
            " mdc-floating-label--float-above"
        } else { "" })
    });

    let classes = cloned!((focus, value, disabled) => move || {
        format!("mdc-text-field mdc-text-field--filled{}{}",
            if *focus.get() || *value.get() != "" {
                " mdc-text-field--focused"
            } else { "" },
            if *disabled.get() {
                " mdc-text-field--disabled" } else { "" }
        )
    });

    view! {
        label(class=classes()) {
            span(class="mdc-text-field__ripple")
            span(class=floating_label_classes(), id=id) {
                (props.label)
            }
            input(class="mdc-text-field__input", type="text",
                aria-labelledby=id, bind:value=props.value, on:input=on_input, on:focus=focus_handler, on:blur=blur_handler,
                disabled=*props.disabled.get())
            span(class=line_classes())
        }
    }
}
