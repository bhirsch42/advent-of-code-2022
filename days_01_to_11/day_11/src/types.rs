use regex::Captures;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation character"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Variable {
    Old,
}

#[derive(Copy, Clone, Debug)]
pub enum OperationValue {
    Variable(Variable),
    Value(i64),
}

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub starting_items: Vec<i64>,
    pub operation: Operation,
    pub operation_value: OperationValue,
    pub test_divisor: i64,
    pub true_target: usize,
    pub false_target: usize,
}

impl From<Captures<'_>> for Monkey {
    fn from(capture: Captures) -> Self {
        let id: usize = capture.name("id").unwrap().as_str().parse().unwrap();

        let starting_items: Vec<i64> = capture
            .name("starting_items")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let operation = capture.name("operation").unwrap().as_str();
        let operation: Operation = operation.try_into().unwrap();

        let operation_value = capture.name("operation_value").unwrap().as_str();
        let operation_value = match operation_value {
            "old" => OperationValue::Variable(Variable::Old),
            value => OperationValue::Value(value.parse().unwrap()),
        };

        let test_divisor: i64 = capture
            .name("test_divisor")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let true_target: usize = capture
            .name("true_target")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let false_target: usize = capture
            .name("false_target")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        Monkey {
            id,
            starting_items,
            operation,
            operation_value,
            test_divisor,
            true_target,
            false_target,
        }
    }
}
