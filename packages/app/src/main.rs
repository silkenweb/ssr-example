use app::app;
use silkenweb::{hydration::hydrate, task::spawn_local};

fn main() {
    let app = app();

    spawn_local(async {
        let stats = hydrate("app", app).await;
        web_log::println!("{}", stats);
    });
}
