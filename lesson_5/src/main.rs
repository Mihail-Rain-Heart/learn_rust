enum MString {
    Resource(u32),
    Default(String),
}

impl MString {
    fn print(&self) {
        match self {
            MString::Resource(number) => println!("self {number}"),
            _ => println!("else"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let a = MString::Resource(12);
    let b = MString::Default(String::from("test"));
    a.print();
    b.print()
}
