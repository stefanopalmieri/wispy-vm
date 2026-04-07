//! WispyScheme on Stak VM.
//!
//! Provides a [`WispyPrimitiveSet`] that extends Stak's R7RS primitives with
//! Cayley table algebra operations (`dot`, `tau`, `type-valid?`), and a
//! compiler entry point that prepends the WispyScheme prelude.

mod primitive_set;

pub use primitive_set::WispyPrimitiveSet;

use stak_compiler::CompileError;
use std::io::{Read, Write};

const WISPY_PRELUDE: &str = include_str!("prelude.scm");

/// Compiles a WispyScheme program (R7RS + algebra) into bytecode.
///
/// Prepends the `(wispy algebra)` library definition, then
/// `(import (scheme base) (scheme write) (wispy algebra))`,
/// then the user source.
pub fn compile_wispy(source: impl Read, target: impl Write) -> Result<(), CompileError> {
    let import = b"\n(import (scheme base) (scheme write) (wispy algebra))\n";
    let combined = WISPY_PRELUDE.as_bytes().chain(&import[..]).chain(source);
    stak_compiler::compile_r7rs(combined, target)
}

#[cfg(test)]
mod tests {
    use crate::WispyPrimitiveSet;
    use stak_configuration::DEFAULT_HEAP_SIZE;
    use stak_device::FixedBufferDevice;
    use stak_file::VoidFileSystem;
    use stak_process_context::VoidProcessContext;
    use stak_time::VoidClock;
    use stak_vm::Vm;

    fn run_wispy(source: &[u8]) -> Vec<u8> {
        let mut bc = vec![];
        super::compile_wispy(source, &mut bc).unwrap();

        let device = FixedBufferDevice::<4096, 0>::new(&[]);
        let mut vm = Vm::new(
            vec![Default::default(); DEFAULT_HEAP_SIZE],
            WispyPrimitiveSet::new(
                device,
                VoidFileSystem::new(),
                VoidProcessContext::new(),
                VoidClock::new(),
            ),
        )
        .unwrap();

        vm.run(bc.iter().copied()).unwrap();
        vm.primitive_set().device().output().to_vec()
    }

    #[test]
    fn display_number() {
        assert_eq!(run_wispy(b"(display 42)"), b"42");
    }

    #[test]
    fn dot_top_bot() {
        // CAYLEY[TOP][BOT] = TOP = 0
        assert_eq!(run_wispy(b"(display (dot TOP BOT))"), b"0");
    }

    #[test]
    fn dot_car_t_pair() {
        // CAYLEY[CAR][T_PAIR] = T_PAIR = 12
        assert_eq!(run_wispy(b"(display (dot CAR T_PAIR))"), b"12");
    }

    #[test]
    fn dot_car_t_str() {
        // CAYLEY[CAR][T_STR] = BOT = 1
        assert_eq!(run_wispy(b"(display (dot CAR T_STR))"), b"1");
    }

    #[test]
    fn tau_number() {
        // tau of a number = TOP = 0
        assert_eq!(run_wispy(b"(display (tau 42))"), b"0");
    }

    #[test]
    fn tau_pair() {
        // tau of a pair = T_PAIR = 12
        assert_eq!(run_wispy(b"(display (tau (cons 1 2)))"), b"12");
    }

    #[test]
    fn tau_string() {
        // tau of a string = T_STR = 15
        assert_eq!(run_wispy(b"(display (tau \"hello\"))"), b"15");
    }

    #[test]
    fn type_valid_true() {
        assert_eq!(run_wispy(b"(display (type-valid? CAR T_PAIR))"), b"#t");
    }

    #[test]
    fn type_valid_false() {
        assert_eq!(run_wispy(b"(display (type-valid? CAR T_STR))"), b"#f");
    }
}
