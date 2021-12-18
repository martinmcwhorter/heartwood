use heartwood::button::*;
use heartwood::text_field::*;
use log::info;
use log::Level;
use sycamore::prelude::*;

fn main() {
    console_log::init_with_level(Level::Debug);

    let clicked = Signal::new(false);
    let clicked_count = Signal::new(0);

    create_effect({
        let clicked = clicked.clone();
        let clicked_count = clicked_count.clone();
        move || {
            if *clicked.get() {
                clicked_count.set(*clicked_count.get() + 1);
            }
        }
    });

    let value = Signal::new("".to_string());

    sycamore::render(|| {
        view! {

            Button(ButtonProps {
                variant: ButtonVariant::Contained,
                label: "CONTAINED BUTTON",
                on_click: clicked.clone()
            })
            br()

            Button(ButtonProps {
                variant: ButtonVariant::Outlined,
                label: "OUTLINED BUTTON",
                on_click: clicked.clone()
            })
            br()

            Button(ButtonProps {
                variant: ButtonVariant::Text,
                label: "TEXT BUTTON",
                on_click: clicked.clone()
            })
            br()

            div(class="mdc-typography--display1") {
                "clicked: "
                (clicked_count.get())
            }
            br()

            TextField(TextFieldProps {
                label: "TEXT FIELD",
                value: value.clone(),
            })
            br()br()
            div() {
                "text field value: "
                (value.get())
            }

        }
    });
}
