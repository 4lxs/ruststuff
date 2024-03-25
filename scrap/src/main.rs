use itertools::Itertools;

fn main() {
    let mut max = 0;
    (100..1000)
        .tuple_combinations()
        .reduce(|(a, b), (c, d)| (a * b, 0));
    for i in 100..1000 {
        for j in i..1000 {
            let prod = i * j;
            let sprod = prod.to_string();
            if sprod.chars().rev().collect::<String>() == sprod {
                max = std::cmp::max(prod, max);
            }
        }
    }
    println!("{}", max);
}
