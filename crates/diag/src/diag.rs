//! # Diagnostics
//!
//! Universal diagnostics collector - to be used in `parse` and `sema`.
//!
//! Access pattern
//! ```no_run
//! ##[rustfmt::skip]
//! let x = Diag::warning()
//!     .span(elem.span)
//!     .msg("value exceeds bound - 2^256 - 1")
//!     .build();
//! ```

use crimson_ast::span::Span;

/// ## Reporter
///
/// Reports warnings and errors to the user.
pub struct DiagCtx {
    pub program: String,
    pub diags: Vec<Diag>,
}

#[derive(Debug, Clone)]
pub struct Diag {
    pub ty: DiagTy,
    pub message: String,
    pub labels: Vec<DiagLabel>,
    pub help: Option<String>,
    pub note: Option<String>,
}

// TODO: Implement error code

#[derive(Clone, Debug)]
pub enum DiagTy {
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct DiagLabel {
    pub span: Span,
    pub message: String,
}

impl DiagCtx {
    pub fn new(program: impl AsRef<str>) -> Self {
        Self { program: program.as_ref().to_string(), diags: vec![] }
    }
    pub fn emit(&mut self, diag: Diag) {
        self.diags.push(diag);
    }
}

impl Diag {
    pub fn warning() -> DiagBuilder {
        DiagBuilder { ty: Some(DiagTy::Warning), ..Default::default() }
    }
    pub fn error() -> DiagBuilder {
        DiagBuilder { ty: Some(DiagTy::Error), ..Default::default() }
    }
    pub fn emit(self, diag_ctx: &mut DiagCtx) {
        diag_ctx.emit(self);
    }
}

#[derive(Default)]
pub struct DiagBuilder {
    ty: Option<DiagTy>,
    message: Option<String>,
    labels: Vec<DiagLabel>,
    help: Option<String>,
    note: Option<String>,
}

impl DiagBuilder {
    pub fn msg(mut self, message: impl AsRef<str>) -> DiagBuilder {
        self.message = Some(message.as_ref().to_string());
        self
    }
    pub fn help(mut self, help: impl AsRef<str>) -> DiagBuilder {
        self.help = Some(help.as_ref().to_string());
        self
    }
    pub fn note(mut self, note: impl AsRef<str>) -> DiagBuilder {
        self.help = Some(note.as_ref().to_string());
        self
    }
    pub fn label(mut self, span: Span, message: impl AsRef<str>) -> DiagBuilder {
        self.labels.push(DiagLabel { span, message: message.as_ref().to_string() });
        self
    }
    pub fn build(self) -> Diag {
        Diag {
            ty: self.ty.expect("ty not provided"),
            message: self.message.expect("message not provided"),
            labels: self.labels,
            help: self.help,
            note: self.note,
        }
    }
}
