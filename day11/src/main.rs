#[derive(Clone, Debug)]
enum Operand {
    Oldvalue,
    Number(u64),
}

impl Operand {
    fn try_from_str(op_str: &str) -> Option<Self> {
        match op_str {
            "old" => Some(Operand::Oldvalue),
            _ => Some(Operand::Number(op_str.parse::<u64>().ok()?)),
        }
    }

    fn apply(&self, x: u64) -> u64 {
        match self {
            Operand::Oldvalue => x,
            Operand::Number(y) => *y,
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Operation::Add(a, b) => a.apply(x) + b.apply(x),
            Operation::Multiply(a, b) => a.apply(x) * b.apply(x),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divide_by: u64,
    next_falsy: usize,
    next_truthy: usize,
}

impl Monkey {
    fn try_from_str(str: &str) -> Option<Self> {
        let mut lines = str.lines().skip(1);

        let items = lines
            .next()?
            .split_once(":")?
            .1
            .split(",")
            .filter_map(|x| x.trim().parse::<u64>().ok())
            .collect::<Vec<u64>>();

        let mut operation_str = lines.next()?.split_once("= ")?.1.split_ascii_whitespace();
        let operand_1 = operation_str.next().map(Operand::try_from_str)??;
        let operator = operation_str.next()?;
        let operand_2 = operation_str.next().map(Operand::try_from_str)??;

        println!("{}", operator);

        let operation = match operator {
            "+" => Operation::Add(operand_1, operand_2),
            "*" => Operation::Multiply(operand_1, operand_2),
            _ => unreachable!(),
        };

        let divide_by = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse::<u64>()
            .unwrap();
        let next_truthy = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse::<usize>()
            .unwrap();
        let next_falsy = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse::<usize>()
            .unwrap();

        Some(Self {
            items,
            operation,
            divide_by,
            next_falsy,
            next_truthy,
        })
    }
}

fn parse(monkey_str: &str) -> Vec<Monkey> {
    monkey_str
        .split("\n\n")
        .filter_map(Monkey::try_from_str)
        .collect()
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, func: impl Fn(u64) -> u64) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let new_items = monkey
                .items
                .drain(..)
                .map(|x| func(monkey.operation.apply(x)))
                .collect::<Vec<u64>>();

            inspections[i] += new_items.len();

            let divide_by = monkey.divide_by;
            let next_truthy = monkey.next_truthy;
            let next_falsy = monkey.next_falsy;

            for item in new_items {
                let target = if item % divide_by == 0 {
                    next_truthy
                } else {
                    next_falsy
                };
                monkeys[target].items.push(item);
            }
        }
    }
    inspections.sort_unstable();
    inspections.iter().rev().take(2).product()
}

fn part_one(monkeys: Vec<Monkey>) -> Option<usize> {
    Some(simulate(monkeys, 20, |x| x / 3))
}

fn part_two(monkeys: Vec<Monkey>) -> Option<usize> {
    let base: u64 = monkeys.iter().map(|x| x.divide_by).product();
    Some(simulate(monkeys, 10000, |x| x % base))
}

fn main() {
    //let data = include_str!("./example.txt");
    let data = include_str!("./input.txt");
    let monkeys = parse(data);
    println!("{:#?}", part_one(monkeys.clone()));
    println!("{:#?}", part_two(monkeys));
}
