#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide { numerator: f64, denominator: f64 },
    SquareRoot(f64),
}

impl Operation {
    fn execute(&self) -> Option<f64> {
        match self {
            Self::Add(a, b) => Some(a + b),
            Self::Subtract(a, b) => Some(a - b),
            Self::Multiply(a, b) => Some(a * b),
            Self::Divide {
                numerator,
                denominator,
            } => {
                if *denominator == 0.0 {
                    None
                } else {
                    Some(numerator / denominator)
                }
            }
            Self::SquareRoot(a) => {
                if *a < 0.0 {
                    None
                } else {
                    Some(a.sqrt())
                }
            }
        }
    }
}

fn sum_valid_results(operations: &[Operation]) -> f64 {
    operations.iter().filter_map(|x| x.execute()).sum()
}

fn main() {
    let ops = vec![
        Operation::Add(10.0, 5.0), // 15.0
        Operation::Divide {
            numerator: 20.0,
            denominator: 0.0,
        }, // None (Ignore)
        Operation::SquareRoot(16.0), // 4.0
        Operation::SquareRoot(-9.0), // None (Ignore)
        Operation::Multiply(3.0, 2.0), // 6.0
    ];

    for op in &ops {
        match op.execute() {
            Some(res) => println!("{:?} => Valid result: {}", op, res),
            None => println!("{:?} => Invalid operation!", op),
        }
    }

    let total = sum_valid_results(&ops);
    println!("\nTotal of valid operations: {}", total);
    // Expected total: 15.0 + 4.0 + 6.0 = 25.0
}
