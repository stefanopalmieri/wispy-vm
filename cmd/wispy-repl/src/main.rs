//! WispyScheme REPL — Cayley table algebra on Stak VM.

use main_error::MainError;
use stak_configuration::DEFAULT_HEAP_SIZE;
use stak_device::StdioDevice;
use stak_file::OsFileSystem;
use stak_process_context::OsProcessContext;
use stak_time::OsClock;
use stak_vm::Vm;
use wispy::WispyPrimitiveSet;

const BYTECODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/repl.bc"));

fn main() -> Result<(), MainError> {
    Vm::new(
        vec![Default::default(); DEFAULT_HEAP_SIZE],
        WispyPrimitiveSet::new(
            StdioDevice::new(),
            OsFileSystem::new(),
            OsProcessContext::new(),
            OsClock::new(),
        ),
    )?
    .run(BYTECODE.iter().copied())?;

    Ok(())
}
