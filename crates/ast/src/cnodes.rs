//! List of AST nodes to represent the Crimson IR

#![allow(unused)]

pub enum CConst {
    SignedInt { sign: CConstSign, value: String },
    HexDigit(String),
}

pub enum CConstSign {
    Plus,
    Minus,
}

pub struct CLabelIdent(String);

pub struct CLabel;
