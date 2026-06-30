use crate::compiler::Instr;

pub fn codegen(instrs: &[Instr]) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.extend(prologue().iter().map(|s| s.to_string()));
    for instr in instrs {
        lines.extend(gen_instr(instr));
    }
    lines.extend(epilogue().iter().map(|s| s.to_string()));
    lines.join("\n") + "\n"
}

fn prologue() -> &'static [&'static str] {
    &[
        "    .section .text",
        "    .globl main",
        "main:",
        "    pushq %rbp",
        "    movq  %rsp, %rbp",
    ]
}

fn epilogue() -> &'static [&'static str] {
    &[
        "    popq  %rsi",
        "    leaq  fmt(%rip), %rdi",
        "    xorl  %eax, %eax",
        "    call  printf",
        "    xorl  %eax, %eax",
        "    popq  %rbp",
        "    ret",
        ".Ldiv_zero_error:",
        "    leaq  errmsg(%rip), %rdi",
        "    call  puts",
        "    movl  $1, %edi",
        "    call  exit",
        "    .section .rodata",
        "fmt:",
        "    .string \"%d\\n\"",
        "errmsg:",
        "    .string \"division by zero\"",
        "    .section .note.GNU-stack,\"\",@progbits",
    ]
}

fn gen_instr(instr: &Instr) -> Vec<String> {
    match instr {
        Instr::Push(n) => vec![format!("    pushq ${n}")],
        Instr::Add => vec![
            "    popq  %rax".to_string(),
            "    popq  %rbx".to_string(),
            "    addq  %rbx, %rax".to_string(),
            "    pushq %rax".to_string(),
        ],
        Instr::Sub => vec![
            "    popq  %rax".to_string(),
            "    popq  %rbx".to_string(),
            "    subq  %rax, %rbx".to_string(),
            "    pushq %rbx".to_string(),
        ],
        Instr::Mul => vec![
            "    popq  %rax".to_string(),
            "    popq  %rbx".to_string(),
            "    imulq %rbx, %rax".to_string(),
            "    pushq %rax".to_string(),
        ],
        Instr::Div => vec![
            "    popq  %rcx".to_string(),
            "    cmpq  $0, %rcx".to_string(),
            "    je    .Ldiv_zero_error".to_string(),
            "    popq  %rax".to_string(),
            "    cqto".to_string(),
            "    idivq %rcx".to_string(),
            "    pushq %rax".to_string(),
        ],
        Instr::Neg => vec![
            "    popq  %rax".to_string(),
            "    negq  %rax".to_string(),
            "    pushq %rax".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Instr;

    #[test]
    fn prologue_contains_main_label() {
        assert!(codegen(&[]).contains("main:"));
    }

    #[test]
    fn push_generates_pushq() {
        assert!(codegen(&[Instr::Push(42)]).contains("pushq $42"));
    }

    #[test]
    fn add_generates_addq() {
        assert!(codegen(&[Instr::Add]).contains("addq"));
    }

    #[test]
    fn sub_generates_subq() {
        assert!(codegen(&[Instr::Sub]).contains("subq"));
    }

    #[test]
    fn mul_generates_imulq() {
        assert!(codegen(&[Instr::Mul]).contains("imulq"));
    }

    #[test]
    fn div_generates_idivq() {
        assert!(codegen(&[Instr::Div]).contains("idivq"));
    }

    #[test]
    fn neg_generates_negq() {
        assert!(codegen(&[Instr::Neg]).contains("negq"));
    }

    #[test]
    fn div_contains_zero_check() {
        assert!(codegen(&[Instr::Div]).contains(".Ldiv_zero_error"));
    }

    #[test]
    fn epilogue_contains_printf() {
        assert!(codegen(&[]).contains("call  printf"));
    }
}
