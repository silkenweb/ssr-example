use std::path::Path;

use app::app;
use xshell::write_file;
use xtask_wasm::{
    anyhow::Result,
    clap::{self, StructOpt},
};

#[derive(clap::Parser)]
enum Workflow {
    Build(xtask_wasm::Dist),
    Serve(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    match Workflow::parse() {
        Workflow::Build(arg) => {
            for page in ["index", "page_1", "page_2"] {
                generate_page(page)?;
            }

            let release = arg.release;
            let dist_result = arg
                .static_dir_path(STATIC_PATH)
                .app_name("app")
                .run("app")?;

            if release {
                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(dist_result.wasm)?;
            }
        }
        Workflow::Serve(arg) => {
            arg.arg("build")
                .start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}

fn generate_page(page: &str) -> xshell::Result<()> {
    let page_html = format!(include_str!("../../app/page.tmpl.html"), app_html = app());
    let page_path = Path::new(STATIC_PATH).join(page).with_extension("html");

    write_file(page_path, page_html)
}

const STATIC_PATH: &str = "packages/app/static";
