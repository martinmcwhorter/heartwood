use heartwood::button::*;
use heartwood::text_field::*;
use log::info;
use log::Level;
use sycamore::prelude::*;

fn main() {
    console_log::init_with_level(Level::Debug);

    let clicked_count = Signal::new(0);
    let disable_all = Signal::new(false);

    let handle_click = Box::new(cloned!(clicked_count => move |_| {
        clicked_count.set(*clicked_count.get() + 1);
    }));

    let disable_click = Box::new(cloned!(disable_all => move |_| {
        disable_all.set(!*disable_all.get());
    }));

    let enable_disable_label = create_selector(cloned!((disable_all) => move || 
        if *disable_all.get() { "ENABLE ALL".to_string() } else { "DISABLE ALL".to_string() } ));

    let value = Signal::new("".to_string());

    sycamore::render(|| {
        view! {

            Button(ButtonProps::default()
                    .label_from_str("CONTAINED BUTTON")
                    .on_click(handle_click.clone())
                    .disabled(disable_all.handle().clone()))
            br()

            Button(ButtonProps::default()
                    .label_from_str("OUTLINED BUTTON")
                    .variant(ButtonVariant::Outlined)
                    .on_click(handle_click.clone())
                    .disabled(disable_all.handle().clone()))
            br()

            Button(ButtonProps::default()
                    .label_from_str("TEXT BUTTON")
                    .variant(ButtonVariant::Text)
                    .on_click(handle_click)
                    .disabled(disable_all.handle().clone()))
            br()

            div(class="mdc-typography--display1") {
                "clicked: "
                (clicked_count.get())
            }
            br()

            TextField(TextFieldProps::default()
                      .label_from_str("TEXT FIELD")
                      .value(value.clone())
                      .disabled(disable_all.handle().clone()))
            br()br()
            div() {
                "text field value: "
                (value.get())
            }

            br()br()
            Button(ButtonProps::default()
                    .label(enable_disable_label.clone())
                    .on_click(disable_click))
            br()

        }
    });
}
