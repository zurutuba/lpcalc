use std::iter::Peekable;
use std::str::Chars;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Tokens {
    Number(f64),

    Add,
    Sub,
    Mul,
    Div,

    Mod,
    Pow,

    OpenParen,
    CloseParen,
}

#[derive(Debug)]
enum Error {
    InvalidResult,
    InvalidOperation,
    DivideByZero,
    TooManyOperations,
    UnmatchedParenthesis,
}

fn main() {
    let mut inputstr = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut inputstr) {
            Err(e) => println!("Couldn't read line: {}", e),
            Ok(_) => {
                // Better way to do this?
                if inputstr == "\r\n" {
                    break;
                } else {
                    match calculate(&inputstr) {
                        Ok(Tokens::Number(a)) => println!("= {}", a),
                        Ok(_)  => println!("Invalid result."),
                        Err(e) => println!("Error: {:?}", e),
                    };
                } 
            }
        }

        inputstr = "".to_owned();
    }
}

fn calculate(input: &str) -> Result<Tokens, Error> {
    match tokenize(input) {
        Err(e) => { Err(e) }
        Ok(tokens) => {
            match shunting_yard(tokens) {
                Err(e) => { Err(e) }
                Ok(ops) => { rpn_calc(ops) }
            }
        }
    }
}

fn getstr<F>(it: &mut Peekable<Chars>, x: F) -> String 
    where F: Fn(char) -> bool {
        let mut namestr = String::new();

        while let Some(&c) = it.peek() {
            if x(c) {
                it.next().unwrap();
                namestr.push(c);
            } else {
                break;
            }
        }

        namestr
}

fn tokenize(input: &str) -> Result<Vec<Tokens>, Error> {
    let mut result : Vec<Tokens> = Vec::new();

    let mut ch = input.chars().peekable();

    loop {
        match ch.peek() {
            Some(&'+') => { ch.next(); result.push(Tokens::Add); }
            Some(&'-') => { ch.next(); result.push(Tokens::Sub); }
            Some(&'*') => { ch.next(); result.push(Tokens::Mul); }
            Some(&'/') => { ch.next(); result.push(Tokens::Div); }
            Some(&'%') => { ch.next(); result.push(Tokens::Mod); }
            Some(&'^') => { ch.next(); result.push(Tokens::Pow); }

            Some(&'(') => { ch.next(); result.push(Tokens::OpenParen); }
            Some(&')') => { ch.next(); result.push(Tokens::CloseParen); }
            
            Some(&c) => match c {
                '0' ... '9' | '.' => {
                    let numstring = getstr(&mut ch, |a| a.is_numeric() || a == '.');

                    result.push(Tokens::Number(numstring.parse::<f64>().unwrap()));
                },

                _ => break,
            },

            None => break,
        }
    }

    Ok(result)
}

fn shunting_yard(input: Vec<Tokens>) -> Result<Vec<Tokens>, Error> {
    let mut queue: Vec<Tokens> = Vec::new();
    let mut output: Vec<Tokens> = Vec::new();

    for token in input {
        match token {
            Tokens::Number{..} => { output.push(token); }

            Tokens::Add | Tokens::Sub | Tokens::Mul | Tokens::Div | Tokens::Mod | Tokens::Pow => {
                while let Some(t) = queue.pop() {
                    if precedence(&token) <= precedence(&t) {
                        queue.push(t);
                        break;
                    } else {
                        output.push(t);
                    }
                }
                queue.push(token);
            }

            Tokens::OpenParen => {
                queue.push(token);
            }

            Tokens::CloseParen => {
                loop {
                    match queue.pop() {
                        Some(Tokens::OpenParen) => break,
                        Some(token) => output.push(token),
                        None => return Err(Error::UnmatchedParenthesis),
                    };
                }
            }

        }

        //println!("queue: {:?}\noutput: {:?}\n", queue, output);
    }

    while let Some(a) = queue.pop() {
        output.push(a);
    }

    //println!("Output: {:?}", output);
    //println!("Queue: {:?}", queue);

    Ok(output)
}

fn precedence(token: & Tokens) -> i32 {
    match token {
        &Tokens::Number{..} => { 0 }

        &Tokens::Add => { 4 }
        &Tokens::Sub => { 4 }

        &Tokens::Mul => { 3 }
        &Tokens::Div => { 3 }
        &Tokens::Mod => { 3 }

        &Tokens::Pow => { 2 }

        &Tokens::OpenParen => { 10 }
        &Tokens::CloseParen => { 10 }
    }
}

fn rpn_calc(input: Vec<Tokens>) -> Result<Tokens, Error> {
    let mut stack : Vec<Tokens> = Vec::new();

    for token in input {
        match token {
            Tokens::Number{..} => {
                stack.push(token);
            }

            _ => {
                let op1 = match stack.pop() {
                    Some(Tokens::Number(x)) => x,
                    None => return Err(Error::TooManyOperations),
                    _ => return Err(Error::InvalidOperation),
                };

                let op2 = match stack.pop() {
                    Some(Tokens::Number(x)) => x,
                    None => return Err(Error::TooManyOperations),
                    _ => return Err(Error::InvalidOperation),
                };

                let value = match token {
                    Tokens::Add => { Ok(Tokens::Number(op2 + op1)) },
                    Tokens::Sub => { Ok(Tokens::Number(op2 - op1)) },
                    Tokens::Mul => { Ok(Tokens::Number(op2 * op1)) },
                    
                    Tokens::Div => {
                        match (op1, op2) {
                            (0f64, _) => return Err(Error::DivideByZero),
                            (a, b) => Ok(Tokens::Number(b / a)),
                        }
                    },
                    
                    Tokens::Mod => {
                        match (op1, op2) {
                            (0f64, _) => return Err(Error::DivideByZero),
                            (a, b) => Ok(Tokens::Number(b % a)),
                        }
                    },

                    Tokens::Pow => { Ok(Tokens::Number( op2.powf(op1) )) },

                    _ => { panic!("What happened?"); }
                };

                match value {
                    Ok(res) => { stack.push(res); },
                    Err(e)  => { return Err(e); }
                }
            }
        }
    }

    match stack.pop() {
        Some(Tokens::Number(result)) => {
            Ok(Tokens::Number(result))
        }

        _ => { 
            Err(Error::InvalidResult)
        }
    }
}
