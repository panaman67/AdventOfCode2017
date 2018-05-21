use std::fs::File;
use std::io::prelude::*;

fn main()
{
    // get file
    let mut input_file = File::open("input_day11.txt").expect("No file exists");
    // read to string
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Read to string failed");
    input = input.trim().to_owned();
    // parse to src vector of strings, delimit by ','
    let mut src : Vec<&str> = input.split(",").collect();
    // create dest vector of strings
    let mut dest : Vec<&str> = Vec::new();
    // get window iterator over src vector

    //println!("{:?}", src);

    for slice in src.chunks(2)
    {
        //match slice
        //{
        //    &["ne", "nw"] => println!("n"),
        //    &["nw", "ne"] => println!("n"),
        //    &["se", "sw"] => println!("s"),
        //    &["sw", "se"] => println!("s"),
        //    &["nw", "se"] => println!("stay"),
        //    &["se", "nw"] => println!("stay"),
        //    &["ne", "sw"] => println!("stay"),
        //    &["sw", "ne"] => println!("stay"),
        //    &["n", "s"] => println!("stay"),
        //    &["s", "n"] => println!("stay"),
        //    &[a , b] => println!("{}\n{}", a, b),
        //    &[c] => println!("{}", c)
        //}
    }
    // match with collapsable patterns eg. (ne,nw) -> n, or (ne,sw) -> NULL

    // push each in pattern if no match, else push simplified direction to dest

    // swap contents of vectors

    // clear dest vector

    // repeat until no longer can simplify

    // print length of vector

}
