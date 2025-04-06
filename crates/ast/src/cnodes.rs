//! List of AST nodes to represent the Crimson IR
//!
//! Syntax Tree hirearchy
//!
//! - Function $label_ident
//!     - Block $label_ident
//!         - Statement
//!             - Instruction
//!                 - opcode ..operands
//!             - Assignment
//!                 - %var = opcode .. operands (Instruction)
//!                 - %var = operand
//!
//! Opcode
//! Mix of Crimson's opcodes + EVM opcodes
//! Crimson's opcodes serve as utilities that will eventually compile "away"
//! Final output will only contain EVM opcodes
//!
//! Operand
//! %var, 0x1234 (const), @label_ident
//!
//! Hardork supported
//! CANCUN

#![allow(unused)]

use crate::{ast_node, ext::*};

ast_node!(
    struct CFunction {
        is_entrypoint: bool,
        label_ident: String,
        blocks: Vec<CBlock>,
    }
);

ast_node!(
    struct CBlock {
        is_entrypoint: bool,
        label_ident: CLabelIdent,
        statements: Vec<CStatement>,
    }
);

ast_node!(
    struct CLabelIdent {
        name: String,
    }
);

ast_node!(
    struct CStatement {
        ty: CStmtTy,
    }
);

ast_node!(
    struct CInstruction {
        opcode: COpcode,
        operands: Vec<COperand>,
    }
);

ast_node!(
    struct CAssignment {
        var: CVarAssignee,
        expr: CAssignExpr,
    }
);

ast_node!(
    struct COpcode {
        ty: COpcodeTy,
    }
);

ast_node!(
    struct COperand {
        ty: OperandTy,
    }
);

ast_node!(
    struct CVarAssignee {
        ident: String,
    }
);

ast_node!(
    struct CAssignExpr {
        ty: CAssignExprTy,
    }
);
