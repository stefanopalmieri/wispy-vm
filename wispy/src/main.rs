//! WispyScheme CLI — compile and run Scheme with Cayley table algebra.
//!
//! # Usage
//!
//! ```sh
//! wispy examples/algebra-smoke.scm          # compile + run
//! wispy compile examples/fib.scm -o fib.bc  # compile to bytecode
//! wispy run fib.bc                           # run bytecode
//! ```

use clap::Parser;
use main_error::MainError;
use stak_configuration::DEFAULT_HEAP_SIZE;
use stak_device::StdioDevice;
use stak_file::OsFileSystem;
use stak_process_context::OsProcessContext;
use stak_time::OsClock;
use stak_vm::Vm;
use std::{fs, path::{Path, PathBuf}};
use wispy::WispyPrimitiveSet;

#[derive(clap::Parser)]
#[command(about = "WispyScheme — Cayley table algebra on Stak VM", version)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// Scheme source file to compile and run (shorthand for no subcommand)
    #[arg()]
    file: Option<PathBuf>,

    #[arg(short = 's', long, default_value_t = DEFAULT_HEAP_SIZE)]
    heap_size: usize,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Compile a .scm file to bytecode
    Compile {
        file: PathBuf,
        /// Output bytecode file (default: replace .scm with .bc)
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short = 's', long, default_value_t = DEFAULT_HEAP_SIZE)]
        heap_size: usize,
    },
    /// Run a pre-compiled .bc bytecode file
    Run {
        file: PathBuf,
        #[arg(short = 's', long, default_value_t = DEFAULT_HEAP_SIZE)]
        heap_size: usize,
    },
}

/// Resolve `(load "path")` directives by inlining the referenced file.
/// Tries relative to CWD first, then relative to the source file's directory.
fn resolve_loads(source: &str, base_path: &Path) -> String {
    let base_dir = base_path.parent().unwrap_or(Path::new("."));
    let mut result = String::with_capacity(source.len());

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("(load ") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let load_path = &trimmed[start + 1..start + 1 + end];
                    // Try CWD first, then source file's directory
                    let candidates = [
                        PathBuf::from(load_path),
                        base_dir.join(load_path),
                    ];
                    let mut loaded = false;
                    for candidate in &candidates {
                        if let Ok(contents) = fs::read_to_string(candidate) {
                            result.push_str(&resolve_loads(&contents, candidate));
                            result.push('\n');
                            loaded = true;
                            break;
                        }
                    }
                    if loaded {
                        continue;
                    }
                    eprintln!("warning: could not load {load_path}");
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }

    result
}

fn read_and_resolve(path: &Path) -> Result<String, MainError> {
    let source = fs::read_to_string(path)?;
    Ok(resolve_loads(&source, path))
}

fn run_bytecode(bytecode: &[u8], heap_size: usize) -> Result<(), MainError> {
    Vm::new(
        vec![Default::default(); heap_size],
        WispyPrimitiveSet::new(
            StdioDevice::new(),
            OsFileSystem::new(),
            OsProcessContext::new(),
            OsClock::new(),
        ),
    )?
    .run(bytecode.iter().copied())?;

    Ok(())
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();

    match args.command {
        Some(Command::Compile {
            file,
            output,
            heap_size: _,
        }) => {
            let out = output.unwrap_or_else(|| file.with_extension("bc"));
            let source = read_and_resolve(&file)?;
            let mut bytecode = vec![];
            wispy::compile_wispy(source.as_bytes(), &mut bytecode)?;
            fs::write(out, &bytecode)?;
        }
        Some(Command::Run { file, heap_size }) => {
            let bytecode = fs::read(&file)?;
            run_bytecode(&bytecode, heap_size)?;
        }
        None => {
            if let Some(file) = args.file {
                let source = read_and_resolve(&file)?;
                let mut bytecode = vec![];
                wispy::compile_wispy(source.as_bytes(), &mut bytecode)?;
                run_bytecode(&bytecode, args.heap_size)?;
            } else {
                eprintln!("Usage: wispy <file.scm> | wispy compile <file.scm> | wispy run <file.bc>");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
