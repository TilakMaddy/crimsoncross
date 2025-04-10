use ariadne::{Label, Report, ReportKind, Source};

use crate::diag::{DiagCtx, DiagTy};

pub fn report_diagctx_stderr(diag_ctx: DiagCtx) {
    let mut reports = vec![];

    for diag in diag_ctx.diags {
        let kind = match diag.ty {
            DiagTy::Warning => ReportKind::Warning,
            DiagTy::Error => ReportKind::Error,
        };
        // TODO: investigate 0..0 - From what I observe, it is immaterial to the output
        let mut report = Report::build(kind, ("ir-content", 0..0)).with_message(diag.message);
        for label in diag.labels {
            let start = label.span.char_offset;
            let end = start + label.span.char_count;
            report = report
                .with_label(Label::new(("ir-content", start..end)).with_message(label.message));
        }
        if let Some(help) = diag.help {
            report = report.with_help(help);
        }
        if let Some(note) = diag.note {
            report = report.with_note(note);
        }
        reports.push(report.finish());
    }

    for report in reports {
        report.eprint(("ir-content", Source::from(&diag_ctx.program))).unwrap();
    }
}
