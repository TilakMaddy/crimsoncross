use crimson_diag::{
    diag::{Diag, DiagCtx},
    report_diagctx,
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
    Diag::warning()
        .msg("bad programming language")
        .span(0..0)
        .build()
        .emit(&mut diag_ctx);

    #[rustfmt::skip]
    Diag::error()
        .msg("really bad quote")
        .span(0..0)
        .build()
        .emit(&mut diag_ctx);

    report_diagctx(diag_ctx);
}
