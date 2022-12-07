use silkenweb::mount;
use ssr_example_app::app;

pub fn main() {
    mount("app", app())
}
