use anyhow::Result;
use crate::frac::Frac;

pub fn rpn(str: &String) -> Result<Frac> {
    let mut stack: Vec<Frac> = Vec::new();
    let mut i = 0;
    while i < str.len() {
        let c = str.chars().nth(i).unwrap();
        if c.is_numeric() {
            stack.push(Frac::new(c.to_digit(10).unwrap() as i32, 1).unwrap());
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            match c {
                '+' => stack.push(a + b),
                '-' => stack.push(a - b),
                '*' => stack.push(a * b),
                '/' => {
                    b.throw_num_zero()?;
                    stack.push(a / b)
                },
                _ => (),
            }
        }
        i += 1;
    }
    if stack.len() != 1 {
        panic!("Error: Invalid RPN");
    }
    Ok(stack[0])
}
