use heartwood::button::*;
use heartwood::text_field::*;
use log::info;
use log::Level;
use sycamore::prelude::*;

fn main() {
    console_log::init_with_level(Level::Debug);

    let clicked = Signal::new(false);
    let clicked_count = Signal::new(0);
    let disable_all = Signal::new(false);
    let disable_click = Signal::new(false);

    create_effect({
        let clicked = clicked.clone();
        let clicked_count = clicked_count.clone();
        move || {
            if *clicked.get() {
                clicked_count.set(*clicked_count.get() + 1);
            }
        }
    });

    create_effect({
        let disable_click = disable_click.clone();
        let disable_all = disable_all.clone();
        move || {
            if *disable_click.get() {
                if *disable_all.get() {
                    disable_all.set(false);
                } else {
                    disable_all.set(true)
                }
            }
        }
    });

    let enable_disable_label = create_selector(cloned!((disable_all) => move || 
        if *disable_all.get() { "ENABLE ALL".to_string() } else { "DISABLE ALL".to_string() } ));

    let value = Signal::new("".to_string());

    sycamore::render(|| {
        view! {

            Button(ButtonProps::default()
                    .label_from_str("CONTAINED BUTTON")
                    .on_click(clicked.clone())
                    .disabled(disable_all.clone()))
            br()

            Button(ButtonProps::default()
                    .label_from_str("OUTLINED BUTTON")
                    .variant(ButtonVariant::Outlined)
                    .on_click(clicked.clone())
                    .disabled(disable_all.clone()))
            br()

            Button(ButtonProps::default()
                    .label_from_str("TEXT BUTTON")
                    .variant(ButtonVariant::Text)
                    .on_click(clicked.clone())
                    .disabled(disable_all.clone()))
            br()

            div(class="mdc-typography--display1") {
                "clicked: "
                (clicked_count.get())
            }
            br()

            TextField(TextFieldProps {
                label: "TEXT FIELD",
                value: value.clone(),
                disabled: disable_all.clone(),
                ..TextFieldProps::default()
            })
            br()br()
            div() {
                "text field value: "
                (value.get())
            }

            br()br()
            Button(ButtonProps::default()
                    .label(enable_disable_label.clone())
                    .on_click(disable_click.clone()))
            br()

        }
    });
}
