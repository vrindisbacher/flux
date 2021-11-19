use std::{env::args, process::exit};

const CMD_PREFIX: &str = "-L";

/// Get the path to the sysroot of the current rustup toolchain. Return `None` if the rustup
/// environment variables are not set.
fn sysroot() -> Option<String> {
    let home = option_env!("RUSTUP_HOME")?;
    let toolchain = option_env!("RUSTUP_TOOLCHAIN")?;
    Some(format!("{}/toolchains/{}", home, toolchain))
}

fn main() {
    // Get the arguments from the command line.
    let mut args: Vec<String> = args().filter(|x| !x.starts_with(CMD_PREFIX)).collect();
    // Add the sysroot path to the arguments.
    args.push("--sysroot".into());
    args.push(sysroot().expect("Liquid Rust requires rustup to be built."));
    // Add release mode to the arguments.
    args.push("-O".into());
    // We don't support unwinding.
    args.push("-Cpanic=abort".into());
    // Run the rust compiler with the arguments.
    let exit_code = liquid_rust_driver::run_compiler(args);
    // Exit with the exit code returned by the compiler.
    exit(exit_code)
}
