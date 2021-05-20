use enumscribe::*;

#[derive(TryScribeCowStr, Unscribe, Eq, PartialEq, Debug)]
enum Airport {
    #[enumscribe(str = "LHR")]
    Heathrow,
    #[enumscribe(str = "LGW", case_insensitive)]
    Gatwick(),
    #[enumscribe(str = "LTN", case_insensitive)]
    Luton {},
    #[enumscribe(str = "BHX", case_insensitive, ignore)]
    BirminghamInternational,
    #[enumscribe(other)]
    Other(String),
}

fn main() {
    let luton = Airport::Luton {};
    println!("Hello, {:?}!", luton.try_scribe());

    let other = Airport::Other("Dedicated EasyJet-only airport".to_owned());
    println!("Hello, {:?}!", other.try_scribe());

    println!("{:?}", Airport::unscribe("LHR"));
    println!("{:?}", Airport::unscribe("lhr"));
    println!("{:?}", Airport::unscribe("lgw"));
}
