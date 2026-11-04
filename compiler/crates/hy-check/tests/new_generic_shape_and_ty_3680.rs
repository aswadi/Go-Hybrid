fn check(src: &str) -> Vec<hy_diag::Diagnostic> { hy_check::check_file(&hy_syntax::parse_file("t.hy", src).unwrap()).diagnostics }
#[test]
fn new_generic_shape_and_ty_3680() {
    assert!(check("struct box[t] {\n    val t\n}\nfn worker(x int) {}\nfn main() {\n    b := new(box[int])\n    task worker(b.val)\n    bs := new(box[set[int]])\n    for item in bs.val {\n        task worker(item)\n    }\n}\n").is_empty());
    assert!(check("struct box[t] {\n    val t\n}\nstruct holder {\n    opt option[int]\n}\nfn main() {\n    b := new(box[u8])\n    b.val = 300\n    p := new(holder)\n    match p.opt {\n        some(x) => {}\n    }\n}\n").len() == 2);
}