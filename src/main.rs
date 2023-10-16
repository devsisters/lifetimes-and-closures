use lifetimes_and_closures::str_split_vec::str_split;

fn main() {
    let s = String::from("a,bc,def");
    let segments = get_delimiter_and_split(&s);
    println!("{:?}", segments);
}

fn get_delimiter_and_split(s: &str) -> Vec<&str> {
    let delimiter = String::from(",");
    str_split(s, &delimiter)
}

// use lifetimes_and_closures::str_split_iterator::{str_split, StrSplit};

// fn main() {
//     let s = String::from("a,bc,def");
//     let segments = get_delimiter_and_split(&s);
//     println!("{:?}", segments.collect::<Vec<_>>());
// }

// fn get_delimiter_and_split(s: &str) -> StrSplit<'_, '_> {
//     let delimiter = String::from(",");
//     str_split(s, &delimiter)
// }
