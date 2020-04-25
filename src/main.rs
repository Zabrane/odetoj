// https://github.com/zserge/odetoj/blob/master/src/main.rs

use std::collections::HashMap;
use std::io::BufRead;

type A = Vec<E>;

#[derive(Debug, Clone)]
enum E {Number(i64)}

fn array_from_i64(n: i64) -> A {vec![E::Number(n)]}

fn iota(a: A) -> A {
    if let E::Number(n) = a[0] {(0..n).map(|i| E::Number(i)).collect()}
    else {array_from_i64(0)}
}

fn at(a: &A, i: i64) -> i64 {
    if (i as usize) < a.len() {
        match a[i as usize] {E::Number(n) => n}}
    else {0}
}

fn plus(a: A, b: A) -> A {
    a.iter().zip(&b).map(|(E::Number(e),E::Number(f))| E::Number(e + f)).collect()
// (0..b.depth.len() as i64).map(|i| E::Number(at(&a, i) + at(&b, i))).collect(),
}

// Interpreter

#[derive(Debug, PartialEq)]
enum Token {Number(i64), Variable(String), Verb(char)}

fn parse(s: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut it = s.chars().peekable();
    while let Some(&c) = it.peek() {
        let mut lex = |f: fn(char) -> bool| {
            let mut s = String::from("");
            while let Some(&x) = it.peek() {
                if !f(x) {break}
                s.push(it.by_ref().next().unwrap())
            }
            return s;
        };
        match c {
            '0'..='9' => {result.push(Token::Number(lex(|c| c >= '0' && c <= '9').parse::<i64>().unwrap(),))}
            'a'..='z' => result.push(Token::Variable(lex(|c| c >= 'a' && c <= 'z'))),
            '+' | '^' => result.push(Token::Verb(it.next().unwrap())),
            _ => return Err(format!("unexpected {}", &c)),
        }
    }
    Ok(result)
}

fn eval(tokens: &[Token], env: &mut HashMap<String, A>) -> Result<A, String> {
    if let Some((head, tail)) = tokens.split_first() {
        let a: A = if let Token::Variable(var) = head {
            if let Some((Token::Verb('='), expr)) = tail.split_first() {
                let val = eval(expr, env)?;
                env.insert(var.to_string(), val.clone());
                return Ok(val);
            }
            env.entry(var.to_string())
                .or_insert(array_from_i64(0))
                .clone()}
        else if let Token::Number(num) = head {array_from_i64(*num)}
        else {array_from_i64(0)};

        if let Token::Verb(verb) = head {
            let x = eval(tail, env)?;
            match verb {
                '^' => Ok(iota(x)),
                _ => return Err(format!("unknown monadic verb: {}", verb)),
            }} 
        else if let Some((Token::Verb(verb), expr)) = tail.split_first() {
            let b = eval(expr, env)?;
            match verb {
                '+' => Ok(plus(a, b)),
                _ => return Err(format!("unknown dyadic verb: {}", verb)),}}
        else {Ok(a)}
    } else {Ok(array_from_i64(0))}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut env: HashMap<String, A> = HashMap::new();
        // Atoms
        println!("{:?}", eval(&parse("").unwrap(), &mut env).unwrap());
        println!("{:?}", eval(&parse("1").unwrap(), &mut env).unwrap());
        println!("{:?}", eval(&parse("123").unwrap(), &mut env).unwrap());
        println!("{:?}", eval(&parse("abc").unwrap(), &mut env).unwrap());
        // Monads
        println!("{:?}", eval(&parse("^10").unwrap(), &mut env).unwrap());
        // Dyads
        println!("{:?}", eval(&parse("1+2").unwrap(), &mut env).unwrap());
        // Variables
        println!("{:?}", eval(&parse("d+c").unwrap(), &mut env).unwrap());
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(""), Ok(vec![]));
        assert_eq!(parse("a"), Ok(vec![Token::Variable("a".to_string())]));
        assert_eq!(parse("abc"), Ok(vec![Token::Variable("abc".to_string())]));
        assert_eq!(parse("1"), Ok(vec![Token::Number(1)]));
        assert_eq!(parse("123"), Ok(vec![Token::Number(123)]));
        assert_eq!(parse("1+2"), Ok(vec![Token::Number(1), Token::Verb('+'), Token::Number(2)]));
        assert!(parse("1.2").is_err());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env: HashMap<String, A> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        println!("{:?}", eval(&parse(line?.as_str())?, &mut env)?);
    }
    Ok(())
}
