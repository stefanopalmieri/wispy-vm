use crate::Tag;

/// A type in Scheme.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Type {
    /// A pair.
    #[default]
    Pair,
    /// A null.
    Null,
    /// A boolean.
    Boolean,
    /// A procedure.
    Procedure,
    /// A symbol.
    Symbol,
    /// A string.
    String,
    /// A character.
    Character,
    /// A vector.
    Vector,
    /// A byte vector.
    ByteVector,
    /// A record.
    Record,
    /// A foreign object
    Foreign = Tag::MAX as _,
}

// ── Cayley table (re-exported from wispy-table) ─────────────────
//
// Mapping to Stak's Type enum:
//   T_PAIR(12) ↔ Type::Pair(0),  T_SYM(13) ↔ Type::Symbol(4),
//   T_CLS(14) ↔ Type::Procedure(3),  T_STR(15) ↔ Type::String(5),
//   T_VEC(16) ↔ Type::Vector(7),  T_CHAR(17) ↔ Type::Character(6)

pub use wispy_table::*;
