use std::process::ExitCode;

mod htmlgen;
mod utils;

mod flags {
    use std::path::PathBuf;

    xflags::xflags! {
        cmd app {
            cmd htmlgen {
                /// Folder that holds the HTML spec json files
                required path: PathBuf
                optional -v, --verbose
                optional --stdout
                optional -o, --output-path output_path: PathBuf
            }
        }
    }
}

fn main() -> ExitCode {
    let flags = flags::App::from_env_or_exit();

    let res = match flags.subcommand {
        flags::AppCmd::Htmlgen(htmlgen) => htmlgen::generate(htmlgen),
    };

    if let Err(e) = res {
        eprintln!("ERROR: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
