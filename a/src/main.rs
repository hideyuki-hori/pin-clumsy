fn main() {
    for line in std::io::stdin().lines() {
        if let Ok(l) = line {
            let words = l.split_whitespace().collect::<Vec<_>>();
            println!("Words: {:?}", words);
        }
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

#[allow(dead_code)]
fn add(stack: &mut Vec<i32>) {
    let lns = stack.pop().unwrap();
    let rns = stack.pop().unwrap();
    stack.push(lns + rns);
}
