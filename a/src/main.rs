use std::io::{stdin};  

fn main() {
    let mut stack = vec![];
    for line in stdin().lines() {
        if let Ok(l) = line {
            let words = l.split_whitespace().collect::<Vec<_>>();
            for word in words {
                if let Ok(num) = word.parse::<i32>() {
                    stack.push(num);
                } else {
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("Unknown command: {}", word),
                    }
                }
            }
        }
        println!("Stack: {:?}", stack);
        stack.clear();
    }
}

#[allow(dead_code)]
fn main_() {
    let mut stack = vec![];
    stack.push(5);
    stack.push(3);
    add(&mut stack);
    println!("Result: {:?}", stack);
}

fn add(stack: &mut Vec<i32>) {
    let lns = stack.pop().unwrap();
    let rns = stack.pop().unwrap();
    stack.push(lns + rns);
}

fn sub(stack: &mut Vec<i32>) {
    let lns = stack.pop().unwrap();
    let rns = stack.pop().unwrap();
    stack.push(lns - rns);
}

fn mul(stack: &mut Vec<i32>) {
    let lns = stack.pop().unwrap();
    let rns = stack.pop().unwrap();
    stack.push(lns * rns);
}

fn div(stack: &mut Vec<i32>) {
    let lns = stack.pop().unwrap();
    let rns = stack.pop().unwrap();
    if rns == 0 {
        panic!("Division by zero");
    }
    stack.push(lns / rns);
}