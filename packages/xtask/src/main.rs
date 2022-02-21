use xtask_wasm::{
    anyhow::Result,
    clap::{self, StructOpt},
};

#[derive(clap::Parser)]
enum Workflow {
    Dist(xtask_wasm::Dist),
    Start(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    match Workflow::parse() {
        Workflow::Dist(arg) => {
            // TODO: Implement server side routing
            // TODO: Generate pages
            let release = arg.release;
            let dist_result = arg
                .static_dir_path("packages/app/static")
                .app_name("app")
                .run("app")?;

            if release {
                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(dist_result.wasm)?;
            }
        }
        Workflow::Start(arg) => {
            println!("Listening on http://{}:{}", arg.ip, arg.port);
            arg.arg("dist").start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}
