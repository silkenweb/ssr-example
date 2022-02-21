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
        Workflow::Serve(arg) => {
            println!("Listening on http://{}:{}", arg.ip, arg.port);
            arg.arg("build").start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}
