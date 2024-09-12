// const fn is_op(c: char) -> bool {
//     c == '+' || c == '-' || c == '*' || c == '/'
// }

#[derive(Debug)]
pub enum EvalError {
    UnexpectedChar(usize),
    BadChar(usize),
    MismatchedBrace(usize),
    DivisionByZero,
}

pub fn tokenizer(s: &str) -> Result<Vec<String>, EvalError> {
    type Ee = EvalError;
    let mut tokens = Vec::new();
    let mut chars = s.chars().enumerate();
    let mut temp = String::new();
    let mut brace_level = 0;
    let mut prev = 10; // 1 numbers ), 2 +-, 3 */, 4 , 9 (, 10 init
    // let flush = |tks: &mut _, tmp: &mut _| {
    //     tks.push(tmp.clone());
    //     tmp.clear();
    // };
    while let Some((i, c)) = chars.next() {
        if c.is_whitespace() {
            if !temp.is_empty() {
                if prev != 1 {
                    tokens.push(temp.clone());
                    temp.clear();
                } else {
                    return Err(Ee::UnexpectedChar(i));
                }
            }
        } else if c == '+' || c == '-' {
            if prev >= 2 {
                temp.push(c);
                prev = 1;
            } else {
                tokens.push(temp.clone());
                temp.clear();
                tokens.push(c.to_string());
                prev = 2;
            }
        } else if c == '*' || c == '/' {
            if prev >= 2 {
                return Err(Ee::UnexpectedChar(i));
            } else {
                tokens.push(temp.clone());
                temp.clear();
                tokens.push(c.to_string());
                prev = 3;
            }
        } else if c == '(' {
            brace_level += 1;
            tokens.push(c.to_string());
            prev = 9;
        } else if c == ')' {
            brace_level -= 1;
            if brace_level < 0 {
                return Err(Ee::MismatchedBrace(i));
            }
            if prev == 9 {
                return Err(Ee::UnexpectedChar(i));
            }
            tokens.push(temp.clone());
            temp.clear();
            tokens.push(c.to_string());
            prev = 1;
        } else if c.is_numeric() || c == '.' || c == 'e' || c == 'E' {
            temp.push(c);
            prev = 1;
        } else {
            return Err(Ee::BadChar(i));
        }
    }
    tokens.push(temp.clone());
    temp.clear();
    while brace_level > 0 {
        tokens.push(')'.to_string());
        brace_level -= 1;
    }
    Ok(tokens)
}

const fn priority(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '(' => 0,
        _ => panic!(),
    }
}

pub fn eval(expr: &str) -> Result<i32, EvalError> {
    let tokens = tokenizer(expr)?;
    let mut numbers = Vec::new();
    let mut ops = Vec::new();
    let calc = |numbers: &mut Vec<i32>, op| {
        let n2 = numbers.pop().unwrap();
        let n1 = numbers.pop().unwrap();
        match op {
            '+' => numbers.push(n1 + n2),
            '-' => numbers.push(n1 - n2),
            '*' => numbers.push(n1 * n2),
            '/' => {
                if n2 == 0{
                    return Err(EvalError::DivisionByZero);
                }
                numbers.push(n1 / n2)
            },
            _ => panic!(),
        }
        Ok(())
    };
    for token in tokens {
        if token.chars().next().unwrap().is_numeric() {
            numbers.push(token.parse::<i32>().unwrap());
        } else {
            let op = token.chars().next().unwrap();
            if op == '(' {
                ops.push(op);
            } else if op == ')' {
                while ops.last() != Some(&'(') {
                    calc(&mut numbers, ops.pop().unwrap())?;
                }
                ops.pop();
            } else {
                while !ops.is_empty() && priority(*ops.last().unwrap()) > priority(op) {
                    calc(&mut numbers, ops.pop().unwrap())?;
                }
                ops.push(op);
            }
        }
    }
    while !ops.is_empty() {
        calc(&mut numbers, ops.pop().unwrap())?;
    }
    Ok(numbers.pop().unwrap())
}