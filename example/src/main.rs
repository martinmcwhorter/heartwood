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
        if *disable_all.get() { "DISABLE ALL".to_string() } else { "ENABLE ALL".to_string() } ));

    let value = Signal::new("".to_string());

    sycamore::render(|| {
        view! {

            Button(ButtonProps {
                label: Signal::new("CONTAINED BUTTON".to_string()).handle(),
                on_click: clicked.clone(),
                disabled: disable_all.clone(),
                ..ButtonProps::default()
            })
            br()

            Button(ButtonProps {
                variant: ButtonVariant::Outlined,
                label: Signal::new("OUTLINED BUTTON".to_string()).handle(),
                on_click: clicked.clone(),
                disabled: disable_all.clone(),
                ..ButtonProps::default()
            })
            br()

            Button(ButtonProps {
                variant: ButtonVariant::Text,
                label: Signal::new("TEXT BUTTON".to_string()).handle(),
                on_click: clicked.clone(),
                disabled: disable_all.clone(),
                ..ButtonProps::default()
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

            br()br()
            Button(ButtonProps {
                label: enable_disable_label,
                on_click: disable_click.clone(),
                ..ButtonProps::default()
            })
            br()

        }
    });
}
