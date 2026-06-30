use simplang_rust::codegen::codegen;
use simplang_rust::compiler::{compile, Expr};
use std::process::Command;
use tempfile::TempDir;

fn compile_and_run(expr: Expr) -> String {
    let tmp = TempDir::new().unwrap();
    let asm_path = tmp.path().join("out.s");
    let bin_path = tmp.path().join("out");

    std::fs::write(&asm_path, codegen(&compile(&expr))).unwrap();

    let status = Command::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .unwrap();
    assert!(status.success(), "gcc failed");

    let output = Command::new(&bin_path).output().unwrap();
    String::from_utf8(output.stdout)
        .unwrap()
        .trim_end_matches('\n')
        .to_string()
}

#[test]
fn eval_literal() {
    assert_eq!(compile_and_run(Expr::Lit(7)), "7");
}

#[test]
fn eval_add() {
    assert_eq!(
        compile_and_run(Expr::Add(Box::new(Expr::Lit(3)), Box::new(Expr::Lit(4)))),
        "7"
    );
}

#[test]
fn eval_sub() {
    assert_eq!(
        compile_and_run(Expr::Sub(Box::new(Expr::Lit(5)), Box::new(Expr::Lit(3)))),
        "2"
    );
}

#[test]
fn eval_mul() {
    assert_eq!(
        compile_and_run(Expr::Mul(Box::new(Expr::Lit(2)), Box::new(Expr::Lit(6)))),
        "12"
    );
}

#[test]
fn eval_div() {
    assert_eq!(
        compile_and_run(Expr::Div(Box::new(Expr::Lit(6)), Box::new(Expr::Lit(3)))),
        "2"
    );
}

#[test]
fn eval_neg() {
    assert_eq!(
        compile_and_run(Expr::Neg(Box::new(Expr::Lit(5)))),
        "-5"
    );
}

#[test]
fn eval_complex_expr() {
    // 1 + 2 * (3 - 4) = -1
    let expr = Expr::Add(
        Box::new(Expr::Lit(1)),
        Box::new(Expr::Mul(
            Box::new(Expr::Lit(2)),
            Box::new(Expr::Sub(Box::new(Expr::Lit(3)), Box::new(Expr::Lit(4)))),
        )),
    );
    assert_eq!(compile_and_run(expr), "-1");
}
