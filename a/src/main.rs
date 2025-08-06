use std::io::{stdin};  

#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("value is not a number!"),
        }
    }
}

fn main() {
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
    }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
    let mut stack = vec![];
    let input: Vec<_> = line.split(" ").collect();
    let mut words = &input[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            stack.push(value);
        } else if let Ok(parsed) = word.parse::<i32>() {
            stack.push(Value::Num(parsed));
        } else {
            match word {
                "+" => add(&mut stack),
                "-" => sub(&mut stack),
                "*" => mul(&mut stack),
                "/" => div(&mut stack),
                _ => panic!("{word:?} could not be parsed"),
            }
        }
        words = rest;
    }

    println!("stack: {stack:?}");
    stack
}

fn parse_block<'src, 'a>(
    input: &'a [&'src str],
) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word));
        }
        words = rest;
    }
    (Value::Block(tokens), words)
}


fn add(stack: &mut Vec<Value>) {
  let rhs = stack.pop().unwrap().as_num();
  let lhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs + rhs));
}

fn sub(stack: &mut Vec<Value>) {
  let rhs = stack.pop().unwrap().as_num();
  let lhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs - rhs));
}

fn mul(stack: &mut Vec<Value>) {
  let rhs = stack.pop().unwrap().as_num();
  let lhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs * rhs));
}

fn div(stack: &mut Vec<Value>) {
  let rhs = stack.pop().unwrap().as_num();
  let lhs = stack.pop().unwrap().as_num();
  stack.push(Value::Num(lhs / rhs));
}

#[cfg(test)]
mod test {
    use super::{parse, Value::*};
    #[test]
    fn test_group() {
        assert_eq!(
            parse("1 2 + { 3 4 }"),
            vec!(Num(3), Block(vec![Num(3), Num(4)]))
        )
    }
}
// #[allow(dead_code)]
// fn main__() {
//     let mut stack = vec![];
//     for line in stdin().lines() {
//         if let Ok(l) = line {
//             let words = l.split_whitespace().collect::<Vec<_>>();
//             for word in words {
//                 if let Ok(num) = word.parse::<i32>() {
//                     stack.push(num);
//                 } else {
//                     match word {
//                         "+" => add(&mut stack),
//                         "-" => sub(&mut stack),
//                         "*" => mul(&mut stack),
//                         "/" => div(&mut stack),
//                         _ => panic!("Unknown command: {}", word),
//                     }
//                 }
//             }
//         }
//         println!("Stack: {:?}", stack);
//         stack.clear();
//     }
// }

// #[allow(dead_code)]
// fn main_() {
//     let mut stack = vec![];
//     stack.push(5);
//     stack.push(3);
//     add(&mut stack);
//     println!("Result: {:?}", stack);
// }

// fn add(stack: &mut Vec<i32>) {
//     let lns = stack.pop().unwrap();
//     let rns = stack.pop().unwrap();
//     stack.push(lns + rns);
// }

// fn sub(stack: &mut Vec<i32>) {
//     let lns = stack.pop().unwrap();
//     let rns = stack.pop().unwrap();
//     stack.push(lns - rns);
// }

// fn mul(stack: &mut Vec<i32>) {
//     let lns = stack.pop().unwrap();
//     let rns = stack.pop().unwrap();
//     stack.push(lns * rns);
// }

// fn div(stack: &mut Vec<i32>) {
//     let lns = stack.pop().unwrap();
//     let rns = stack.pop().unwrap();
//     if rns == 0 {
//         panic!("Division by zero");
//     }
//     stack.push(lns / rns);
// }