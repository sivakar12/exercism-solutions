#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            &CalculatorInput::Value(v) => { stack.push(v) },
            operation => {
                let (v1, v2) = (stack.pop(), stack.pop());
                match (v1, v2) {
                    (Some(n1), Some(n2)) => {
                        match operation {
                            &CalculatorInput::Add => { stack.push(n1 + n2) },
                            &CalculatorInput::Subtract => { stack.push(n2 - n1) },
                            &CalculatorInput::Multiply => { stack.push(n1 * n2) },
                            &CalculatorInput::Divide => { stack.push(n2 / n1) },
                            &CalculatorInput::Value(v) => { stack.push(v)}
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    return match stack.len() {
        1 => stack.pop(),
        _ => None
    };
}
