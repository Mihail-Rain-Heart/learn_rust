#[derive(Debug)]
struct Employer {
    name: String,
    surname: String,
}

impl Employer {
    fn full_name(&self) -> String {
        self.name.clone() + " " + &self.surname
    }
}

impl Employer {
    fn no_associated() {
        println!("no_associated")
    }
}

fn main() {
    let slave = Employer {
        name: String::from("Hard"),
        surname: String::from("No name"),
    };
    println!("{}", slave.full_name());
    println!("{:?}", slave);
    Employer::no_associated()
}
