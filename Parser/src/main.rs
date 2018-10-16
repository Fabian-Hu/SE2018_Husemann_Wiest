use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::io::BufReader;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),

    }
    let mut file2 = BufReader::new(&file);
    for line in file2.lines(){
let l = line.unwrap();
println!("{}", l);    
}    


    //for word in s.split_whitespace() {
	//println!("{}", word);
    //}
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
