//! Regression tests for #3728:
//! 1. `typed_int_bound` must classify `len(s)` on non-constant `string`
//!    operands as returning `int` (`HYB-E2009`).
//! 2. `check_range_iterable` must project struct fields on composite bases
//!    when rejecting send-only channels (`HYB-E2009`).

use hy_check::{check_file, Verdict};
use hy_diag::Severity;
use hy_syntax::parse_file;

fn check(src: &str) -> hy_check::CheckResult {
    let ast = parse_file("inline.hy", src).expect("must parse");
    check_file(&ast)
}

fn assert_has_e2009(src: &str) {
    let r = check(src);
    let cs: Vec<_> = r.diagnostics.iter().filter(|d| d.severity == Severity::Error).map(|d| d.code.as_str()).collect();
    assert!(cs.contains(&"HYB-E2009"), "expected HYB-E2009, got {cs:?} for:\n{src}");
    assert_eq!(r.verdict, Verdict::Reject);
}

fn assert_accept(src: &str) {
    let r = check(src);
    let cs: Vec<_> = r.diagnostics.iter().filter(|d| d.severity == Severity::Error).map(|d| d.code.as_str()).collect();
    assert!(cs.is_empty(), "expected 0 errors, got {cs:?} for:\n{src}");
    assert_eq!(r.verdict, Verdict::Accept);
}

#[test]
fn runtime_str_len_bounds_and_composite_field_send_only_chans_reject_e2009() {
    assert_has_e2009(
        "fn main() {\n    var lo i64 = 0\n    s := \"abc\"\n    for i in lo..len(s) {\n        _ = i\n    }\n}\n",
    );
    assert_has_e2009(
        "fn run(s string) {\n    for i in len(s)..u32(5) {\n        _ = i\n    }\n}\nfn main() { run(\"abc\") }\n",
    );
    assert_has_e2009(
        "struct holder { ch chan<- int }\nfn drain(boxes []holder) {\n    for v in boxes[0:1][0].ch {\n        _ = v\n    }\n}\nfn main() {}\n",
    );
    assert_has_e2009(
        "struct holder { ch chan<- int }\nfn drain(boxes []holder, b holder) {\n    for v in append(boxes, b)[0].ch {\n        _ = v\n    }\n}\nfn main() {}\n",
    );
}

#[test]
fn const_str_len_and_recv_channel_composite_field_accept() {
    assert_accept(
        "const s = \"abc\"\nstruct holder { ch <-chan int }\nfn drain(boxes []holder) {\n    var lo i64 = 0\n    for i in lo..len(s) {\n        _ = i\n    }\n    for v in boxes[0:1][0].ch {\n        _ = v\n    }\n}\nfn main() {}\n",
    );
}
