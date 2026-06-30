#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Instr {
    Push(i64),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

// Lexer

pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() {
            let mut num = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Int(num.parse().unwrap()));
        } else {
            let tok = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '(' => Token::LParen,
                ')' => Token::RParen,
                other => return Err(format!("unexpected character: {other}")),
            };
            chars.next();
            tokens.push(tok);
        }
    }
    Ok(tokens)
}

// Parser
//
// expr   ::= term   (('+' | '-') term)*
// term   ::= factor (('*' | '/') factor)*
// factor ::= INT | '(' expr ')' | '-' factor

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut pos = 0;
    let expr = parse_expr(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(format!("unexpected token: {:?}", tokens[pos]));
    }
    Ok(expr)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_term(tokens, pos)?;
    loop {
        match tokens.get(*pos) {
            Some(Token::Plus) => {
                *pos += 1;
                let right = parse_term(tokens, pos)?;
                left = Expr::Add(Box::new(left), Box::new(right));
            }
            Some(Token::Minus) => {
                *pos += 1;
                let right = parse_term(tokens, pos)?;
                left = Expr::Sub(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_factor(tokens, pos)?;
    loop {
        match tokens.get(*pos) {
            Some(Token::Star) => {
                *pos += 1;
                let right = parse_factor(tokens, pos)?;
                left = Expr::Mul(Box::new(left), Box::new(right));
            }
            Some(Token::Slash) => {
                *pos += 1;
                let right = parse_factor(tokens, pos)?;
                left = Expr::Div(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    match tokens.get(*pos) {
        Some(Token::Int(n)) => {
            let n = *n;
            *pos += 1;
            Ok(Expr::Lit(n))
        }
        Some(Token::LParen) => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            match tokens.get(*pos) {
                Some(Token::RParen) => {
                    *pos += 1;
                    Ok(expr)
                }
                _ => Err("expected closing parenthesis".to_string()),
            }
        }
        Some(Token::Minus) => {
            *pos += 1;
            let expr = parse_factor(tokens, pos)?;
            Ok(Expr::Neg(Box::new(expr)))
        }
        None => Err("unexpected end of input".to_string()),
        Some(t) => Err(format!("unexpected token: {t:?}")),
    }
}

// Compiler: Expr → IR

pub fn compile(expr: &Expr) -> Vec<Instr> {
    match expr {
        Expr::Lit(n) => vec![Instr::Push(*n)],
        Expr::Add(l, r) => {
            let mut v = compile(l);
            v.extend(compile(r));
            v.push(Instr::Add);
            v
        }
        Expr::Sub(l, r) => {
            let mut v = compile(l);
            v.extend(compile(r));
            v.push(Instr::Sub);
            v
        }
        Expr::Mul(l, r) => {
            let mut v = compile(l);
            v.extend(compile(r));
            v.push(Instr::Mul);
            v
        }
        Expr::Div(l, r) => {
            let mut v = compile(l);
            v.extend(compile(r));
            v.push(Instr::Div);
            v
        }
        Expr::Neg(e) => {
            let mut v = compile(e);
            v.push(Instr::Neg);
            v
        }
    }
}

// Stack VM

pub fn run(instrs: &[Instr]) -> Result<i64, String> {
    let mut stack: Vec<i64> = Vec::new();
    for instr in instrs {
        match instr {
            Instr::Push(n) => stack.push(*n),
            Instr::Add => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(a + b);
            }
            Instr::Sub => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(a - b);
            }
            Instr::Mul => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(a * b);
            }
            Instr::Div => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a / b);
            }
            Instr::Neg => {
                let a = stack.pop().ok_or("stack underflow")?;
                stack.push(-a);
            }
        }
    }
    match stack.as_slice() {
        [v] => Ok(*v),
        [] => Err("invalid stack state after execution".to_string()),
        _ => Err("invalid stack state after execution".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer() {
        assert_eq!(tokenize("42"), Ok(vec![Token::Int(42)]));
    }

    #[test]
    fn tokenize_skips_whitespace() {
        assert_eq!(
            tokenize("1 + 2"),
            Ok(vec![Token::Int(1), Token::Plus, Token::Int(2)])
        );
    }

    #[test]
    fn tokenize_all_operators() {
        assert_eq!(
            tokenize("+-*/"),
            Ok(vec![Token::Plus, Token::Minus, Token::Star, Token::Slash])
        );
    }

    #[test]
    fn tokenize_parens() {
        assert_eq!(
            tokenize("(1)"),
            Ok(vec![Token::LParen, Token::Int(1), Token::RParen])
        );
    }

    #[test]
    fn tokenize_unknown_char_is_error() {
        assert_eq!(
            tokenize("1 + a"),
            Err("unexpected character: a".to_string())
        );
    }

    #[test]
    fn parse_literal() {
        assert_eq!(parse(vec![Token::Int(5)]), Ok(Expr::Lit(5)));
    }

    #[test]
    fn parse_add() {
        assert_eq!(
            parse(vec![Token::Int(1), Token::Plus, Token::Int(2)]),
            Ok(Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2))))
        );
    }

    #[test]
    fn parse_sub() {
        assert_eq!(
            parse(vec![Token::Int(3), Token::Minus, Token::Int(1)]),
            Ok(Expr::Sub(Box::new(Expr::Lit(3)), Box::new(Expr::Lit(1))))
        );
    }

    #[test]
    fn parse_mul_before_add() {
        assert_eq!(
            parse(vec![
                Token::Int(1),
                Token::Plus,
                Token::Int(2),
                Token::Star,
                Token::Int(3)
            ]),
            Ok(Expr::Add(
                Box::new(Expr::Lit(1)),
                Box::new(Expr::Mul(Box::new(Expr::Lit(2)), Box::new(Expr::Lit(3))))
            ))
        );
    }

    #[test]
    fn parse_parens_override_precedence() {
        assert_eq!(
            parse(vec![
                Token::LParen,
                Token::Int(1),
                Token::Plus,
                Token::Int(2),
                Token::RParen,
                Token::Star,
                Token::Int(3)
            ]),
            Ok(Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Lit(1)), Box::new(Expr::Lit(2)))),
                Box::new(Expr::Lit(3))
            ))
        );
    }

    #[test]
    fn parse_unary_minus() {
        assert_eq!(
            parse(vec![Token::Minus, Token::Int(5)]),
            Ok(Expr::Neg(Box::new(Expr::Lit(5))))
        );
    }

    #[test]
    fn parse_empty_is_error() {
        assert_eq!(parse(vec![]), Err("unexpected end of input".to_string()));
    }
}
