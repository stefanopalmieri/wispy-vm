//! Compiles the WispyScheme REPL source (with algebra prelude) into bytecode.

use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Read the wispy algebra prelude
    let prelude = include_str!("../../wispy/src/prelude.scm");

    // Read the REPL source
    let repl_source = fs::read_to_string("src/main.scm").expect("src/main.scm");

    // Chain: wispy prelude (defines the library) + REPL source (imports it)
    let combined = format!("{prelude}\n{repl_source}");

    // Compile via stak-compiler (prepends standard R7RS prelude)
    let mut bytecode = vec![];
    stak_compiler::compile_r7rs(combined.as_bytes(), &mut bytecode)
        .expect("failed to compile REPL");

    let out_path = Path::new(&out_dir).join("repl.bc");
    fs::write(&out_path, &bytecode).unwrap();

    println!("cargo::rerun-if-changed=src/main.scm");
    println!("cargo::rerun-if-changed=../../wispy/src/prelude.scm");
}
