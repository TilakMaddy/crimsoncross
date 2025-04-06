//! CNode extensions
//! Be careful what you put here, because it's a cyclic import in cnodes

use crate::cnodes::{CAssignment, CInstruction, COperand};

/// ## Operand Ty
///
/// ### Used in instructions and assignments
/// * VarIdent - should refer to existing initd. variable
/// * Label - should refer to label identifier with `@` prepended
/// * Const - literal whose value is a Signed Int or Hex digit
#[derive(Debug)]
pub enum OperandTy {
    VarIdent(String),
    Label(String),
    Const(String),
}

/// ## Assignment Expression
///
/// ### Appears on the R.H.S of an assignment
///
/// LHS is always a var identifier.
///
/// Format:
/// %var = instruction | operand
#[derive(Debug)]
pub enum CAssignExpr {
    Instruction(Box<CInstruction>),
    Operand(Box<COperand>),
}

/// ## Crimson Opcode
///
/// Crimson Opcodes are a mix of opcodes that includes custom opcodes for enabling easy codegen from
/// higher level languages.
///
/// Non-EVM compatible Opcodes will boil down to EVM compatible opcodes eventually in the
/// compilation pipeline
#[derive(Debug)]
pub enum COpcode {
    LOG,
    INVOKE,
}

/// ## Crimson Statement
#[derive(Debug)]
pub enum CStmtTy {
    Instruction(Box<CInstruction>),
    Assignment(Box<CAssignment>),
}
