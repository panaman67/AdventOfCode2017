use std::fs::File;
use std::io::prelude::*;

fn main()
{
    // get file
    // read to string
    // parse to src vector of strings, delimit by ','
    // create dest vector of strings
    // get window iterator over src vector
    // match with collapsable patterns eg. (ne,nw) -> n, or (ne,sw) -> NULL
    // push each in pattern if no match, else push simplified direction to dest
    // swap contents of vectors
    // clear dest vector
    // repeat until no longer can simplify
    // print length of vector
}
