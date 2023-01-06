use silkenweb::{
    dom::Dom,
    elements::{
        html::{button, div, p, Div},
        ElementEvents,
    },
    hydration::hydrate,
    prelude::{HtmlElement, ParentElement},
    router,
    task::spawn_local,
    value::Sig,
};

pub fn hydrate_app() {
    let app = app();

    spawn_local(async {
        let stats = hydrate("app", app).await;
        web_log::println!("{}", stats);
    });
}

pub fn app<D: Dom>() -> Div<D> {
    div()
        .id("app")
        .child(
            button()
                .on_click(|_, _| router::set_url_path("page_1.html"))
                .text("Go to page 1"),
        )
        .child(
            button()
                .on_click(|_, _| router::set_url_path("page_2.html"))
                .text("Go to page 2"),
        )
        .child(p().text(Sig(router::url_path().signal_ref(|url_path| {
            format!(
                "URL Path is: {}",
                match url_path.as_str() {
                    "" => "index.html",
                    path => path,
                }
            )
        }))))
}
