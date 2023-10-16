use lifetimes_and_closures::str_split;

fn main() {
    let s = "a,bc,def";
    let segments = read_delimiter_and_split(s);
    println!("{:?}", segments);
}

fn read_delimiter_and_split(s: &str) -> Vec<&str> {
    let delimiter = String::from(",");
    str_split(s, &delimiter)
}
