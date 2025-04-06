use ariadne::{Label, Report, ReportKind, Source};

use crate::diag::{DiagCtx, DiagTy};

pub fn report_diagctx(diag_ctx: DiagCtx) {
    let mut reports = vec![];

    for diag in diag_ctx.diags {
        let kind = match diag.ty {
            DiagTy::Warning => ReportKind::Warning,
            DiagTy::Error => ReportKind::Error,
        };
        let start = diag.span.char_offset;
        let end = start + diag.span.char_count;
        let mut report = Report::build(kind, ("ir-content", start..end)).with_message(diag.message);
        for label in diag.labels {
            let start = label.span.char_offset;
            let end = start + label.span.char_count;
            report = report
                .with_label(Label::new(("ir-content", start..end)))
                .with_message(label.message);
        }
        if let Some(help) = diag.help {
            report = report.with_help(help);
        }
        if let Some(note) = diag.note {
            report = report.with_note(note);
        }
        reports.push(report);
    }

    for report in reports {
        report.finish().print(("ir-content", Source::from(&diag_ctx.program))).unwrap();
    }
}
