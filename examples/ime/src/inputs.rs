use floem::{
    prelude::*,
};

pub fn text_input_view() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let text2 = RwSignal::new(String::new());
    let text3 = RwSignal::new(String::new());

    Stack::vertical((
        text_input(text)
            .placeholder("Placeholder text")
            .style(|s| s.width(250.)),
        text_input(text2)
            .placeholder("Placeholder text")
            .style(|s| s.width(250.)),

        text_input(text3)
            .placeholder("Placeholder text")
            .style(|s| s.width(250.)),

    ))
}
