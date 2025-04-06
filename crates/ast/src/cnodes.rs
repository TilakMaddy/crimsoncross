//! List of AST nodes to represent the Crimson IR

#![allow(unused)]

use crate::{ast_node, ext::*};

// Identifies a function or a block
ast_node!(
    struct CLabelIdent {
        name: String,
    }
);

// Variable being assigned to. (unique)
ast_node!(
    struct CVarAssignee {
        ident: String,
    }
);

// Arguments to an operation Variable / Const / Label
ast_node!(
    struct COperand {
        ty: OperandTy,
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
    struct CStatement {
        ty: CStmtTy,
    }
);
