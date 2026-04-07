//! WispyScheme primitive set — R7RS + Cayley table algebra.

use stak_device::Device;
use stak_file::FileSystem;
use stak_process_context::ProcessContext;
use stak_r7rs::SmallPrimitiveSet;
use stak_time::Clock;
use stak_vm::{
    CAYLEY, Heap, Memory, Number, PrimitiveSet, Type, Value,
    TOP, BOT, T_PAIR, T_SYM, T_CLS, T_STR, T_VEC, T_CHAR,
    CAYLEY_TRUE,
};
use winter_maybe_async::{maybe_async, maybe_await};

// Primitive IDs (must match wispy/src/prelude.scm)
const PRIM_DOT: usize = 600;
const PRIM_TAU: usize = 601;
const PRIM_TYPE_VALID: usize = 602;

/// Wraps [`SmallPrimitiveSet`] with Cayley table algebra primitives.
pub struct WispyPrimitiveSet<D: Device, F: FileSystem, P: ProcessContext, C: Clock> {
    inner: SmallPrimitiveSet<D, F, P, C>,
}

impl<D: Device, F: FileSystem, P: ProcessContext, C: Clock> WispyPrimitiveSet<D, F, P, C> {
    /// Creates a new WispyScheme primitive set.
    pub fn new(device: D, file_system: F, process_context: P, clock: C) -> Self {
        Self {
            inner: SmallPrimitiveSet::new(device, file_system, process_context, clock),
        }
    }

    /// Returns a reference to the device.
    pub fn device(&self) -> &D {
        self.inner.device()
    }

    /// Returns a mutable reference to the device.
    pub fn device_mut(&mut self) -> &mut D {
        self.inner.device_mut()
    }

    /// Map a Stak value to a Cayley element.
    /// In Stak's Ribbit model, the "type" of a rib is the tag of its CDR field.
    fn stak_tag_to_cayley<H: Heap>(memory: &Memory<H>, value: Value) -> Result<u8, stak_vm::Error> {
        if value.is_number() {
            return Ok(TOP);
        }

        let cons = value.assume_cons();

        // Check for null first
        if let Ok(null) = memory.null() {
            if cons == null {
                return Ok(TOP);
            }
        }

        // The rib's type is the tag on its CDR
        let cdr = memory.cdr(cons)?;
        let tag = if let Some(cdr_cons) = cdr.to_cons() {
            cdr_cons.tag()
        } else {
            // CDR is a number — this is a pair-like rib with numeric cdr
            return Ok(T_PAIR);
        };

        Ok(match tag {
            t if t == Type::Pair as u16 => T_PAIR,
            t if t == Type::Null as u16 => TOP,
            t if t == Type::Boolean as u16 => {
                if let Ok(false_cons) = memory.boolean(false) {
                    if cons == false_cons { BOT } else { CAYLEY_TRUE }
                } else {
                    TOP
                }
            }
            t if t == Type::Procedure as u16 => T_CLS,
            t if t == Type::Symbol as u16 => T_SYM,
            t if t == Type::String as u16 => T_STR,
            t if t == Type::Character as u16 => T_CHAR,
            t if t == Type::Vector as u16 => T_VEC,
            _ => TOP,
        })
    }
}

impl<H: Heap, D: Device, F: FileSystem, P: ProcessContext, C: Clock> PrimitiveSet<H>
    for WispyPrimitiveSet<D, F, P, C>
{
    type Error = stak_r7rs::SmallError;

    #[maybe_async]
    fn operate(&mut self, memory: &mut Memory<H>, primitive: usize) -> Result<(), Self::Error> {
        match primitive {
            PRIM_DOT => {
                // (dot a b) → CAYLEY[a][b]
                let b = memory.pop()?.assume_number().to_i64() as usize;
                let a = memory.pop()?.assume_number().to_i64() as usize;
                let result = if a < 32 && b < 32 { CAYLEY[a][b] } else { BOT };
                memory.push(Number::from_i64(result as i64).into())?;
                Ok(())
            }
            PRIM_TAU => {
                // (tau x) → Cayley type tag
                let value = memory.pop()?;
                let tag = Self::stak_tag_to_cayley(memory, value)?;
                memory.push(Number::from_i64(tag as i64).into())?;
                Ok(())
            }
            PRIM_TYPE_VALID => {
                // (type-valid? op tag) → #t iff CAYLEY[op][tag] != BOT
                let tag = memory.pop()?.assume_number().to_i64() as usize;
                let op = memory.pop()?.assume_number().to_i64() as usize;
                let valid = if op < 32 && tag < 32 {
                    CAYLEY[op][tag] != BOT
                } else {
                    false
                };
                let result = memory.boolean(valid)?;
                memory.push(result.into())?;
                Ok(())
            }
            _ => {
                // Delegate to R7RS primitives
                maybe_await!(self.inner.operate(memory, primitive))
            }
        }
    }
}
