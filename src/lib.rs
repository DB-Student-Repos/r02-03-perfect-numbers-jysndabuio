#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {

    if num == 0 {
        return None;
    }

    let mut factors: Vec<u64> = Vec::new();

    for x in 1..num {
        if num % x == 0 {
            factors.push(x);
        }
    }

    let total:u64 = factors.iter().sum();

    Some(match total {
        total if total == num => Classification::Perfect,
        total if total > num => Classification::Abundant,
        total if total < num => Classification::Deficient,
        _ => Classification::Deficient, // With error if not use, to handle remaining cases.
    })

}
