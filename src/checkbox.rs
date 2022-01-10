use sycamore::prelude::*;
use sycamore::rt::JsValue;
use uuid::Uuid;
use web_sys::HtmlInputElement;
pub struct CheckboxProps {
    pub checked: Signal<bool>,
    pub indeterminate: ReadSignal<bool>,
    pub disabled: ReadSignal<bool>,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            checked: Signal::new(false),
            indeterminate: Signal::new(false).handle(),
            disabled: Signal::new(false).handle(),
        }
    }
}

impl CheckboxProps {
    pub fn checked(mut self, checked: Signal<bool>) -> Self {
        self.checked = checked;
        return self;
    }

    pub fn indeterminate(mut self, indeterminate: ReadSignal<bool>) -> Self {
        self.indeterminate = indeterminate;
        return self;
    }

    pub fn disabled(mut self, disabled: ReadSignal<bool>) -> Self {
        self.disabled = disabled;
        return self;
    }
}

#[component(Checkbox<G>)]
pub fn checkbox(props: CheckboxProps) -> View<G> {
    let checked = props.checked.clone();
    let indeterminate = props.indeterminate.clone();

    let id = Uuid::new_v4();

    let input_ref = NodeRef::new();

    let classes = cloned!((checked, indeterminate) => move || {
        format!("mdc-checkbox  mdc-checkbox--selected{}{}",
                if *checked.get() { " mdc-checkbox--checked" } else { "" },
                if *indeterminate.get() { " mdc-checkbox--indeterminate" } else { "" }
            )
    });

    view! {
        div(class=classes()) {
            input(ref=input_ref,
                  type="checkbox",
                  class="mdc-checkbox__native-control",
                  id=id,
                  checked=*props.checked.get(),
                  disabled=*props.disabled.get(),
                )
            div(class="mdc-checkbox__background",
                dangerously_set_inner_html="<svg class=\"mdc-checkbox__checkmark\" viewBox=\"0 0 24 24\"><path class=\"mdc-checkbox__checkmark-path\" fill=\"none\" d=\"M1.73,12.91 8.1,19.28 22.79,4.59\"></path></svg><div class=\"mdc-checkbox__mixedmark\"></div>")
            div(class="mdc-checkbox__ripple")
        }
    }
}
