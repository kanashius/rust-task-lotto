use std::env;

use rand::{prelude::IteratorRandom};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = &mut rand::thread_rng();
        let numbers = (1..from+1).choose_multiple(&mut rng, take);
        Lotto{take, from, numbers}
    }

    fn get_numbers(&self) -> Vec<usize> {
        self.numbers.clone()
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<3{
        println!("At least two parameters, the take and from parameter must be defined.");
        return;
    }
    if args.len()%2!=1{
        println!("For every take parameter there must be a from parameter.");
        return;
    }
    for i in (1..args.len()).step_by(2){
        let take = args[i].parse::<usize>().expect("One take parameter is not a positive number.");
        let from = args[i+1].parse::<usize>().expect("One from parameter is not a positive number.");
        if take>from{
            println!("The take parameter must be smaller or equal to the from value.");
            return;
        }
        let lotto = Lotto::new(take, from);
        println!("{}", format_lotto_results(&lotto));
        println!("{:?}", lotto.get_numbers());
        println!("{:?}", lotto.get_numbers());
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
