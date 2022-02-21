use silkenweb::{
    elements::{
        html::{button, div, p, Div},
        ElementEvents,
    },
    macros::ElementBuilder,
    prelude::ParentBuilder,
    router::{self, Url},
};

pub fn app() -> Div {
    div()
        .child(
            button()
                .on_click(|_, _| router::set_url_path("/page_1.html"))
                .text("Go to page 1"),
        )
        .child(
            button()
                .on_click(|_, _| router::set_url_path("/page_2.html"))
                .text("Go to page 2"),
        )
        .child(p().text_signal(router::url().signal_ref(|url| {
            format!(
                "URL Path is: {}",
                match url.pathname().as_str() {
                    "/" => "/index.html",
                    path => path,
                }
            )
        })))
        .build()
}
