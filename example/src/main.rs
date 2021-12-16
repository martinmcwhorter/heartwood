
use sycamore::prelude::*;
use heartwood::button::*;

fn main() {

    sycamore::render(|| view! {
        Button(ButtonProps {
            variant: ButtonVariant::Contained,
            label: "CONTAINED BUTTON"
        })
        Button(ButtonProps {
            variant: ButtonVariant::Outlined,
            label: "OUTLINED BUTTON"
        })
        Button(ButtonProps {
            variant: ButtonVariant::Text,
            label: "TEXT BUTTON"
        })
    });
}