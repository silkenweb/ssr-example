use std::path::Path;

use app::app;
use log::LevelFilter;
use silkenweb::{router, task};
use xshell::write_file;
use xtask_wasm::{
    anyhow::Result,
    clap::{self, StructOpt},
    default_dist_dir, WasmOpt,
};

#[derive(clap::Parser)]
enum Workflow {
    Build(xtask_wasm::Dist),
    Serve(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    match Workflow::parse() {
        Workflow::Build(build) => {
            let release = build.release;
            let artifacts = build.app_name("app").run("app")?;

            if release {
                WasmOpt::level(1).shrink(2).optimize(artifacts.wasm)?;
            }

            generate_pages(&artifacts.dist_dir)?;
        }
        Workflow::Serve(server) => {
            server.arg("build").start(default_dist_dir(false))?;
        }
    }

    Ok(())
}

fn generate_pages(dist_dir: &Path) -> xshell::Result<()> {
    let app = app();

    for page in ["index", "page_1", "page_2"] {
        router::set_url_path(&format!("/{}.html", page));
        task::server::render_now_sync();

        let page_html = format!(include_str!("../../app/page.tmpl.html"), app_html = app);
        let page_path = Path::new(dist_dir).join(page).with_extension("html");

        write_file(page_path, page_html)?;
    }

    Ok(())
}
