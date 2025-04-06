use crimson_diag::{
    diag::{Diag, DiagCtx},
    report_diagctx_stderr,
};
use indoc::indoc;

fn main() {
    let testing = indoc! {"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let mut diag_ctx = DiagCtx::new(testing);

    #[rustfmt::skip]
    Diag::error()
        .msg("bad programming language")
        .label(17..19, "python def sucks!")
        .build()
        .emit(&mut diag_ctx);

    report_diagctx_stderr(diag_ctx);
}
