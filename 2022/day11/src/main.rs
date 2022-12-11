use std::fs;


fn main() {
    // Get file name from terminal argument input and read to string
    let filename = std::env::args().nth(1).expect("No file name given");
    let input = std::fs::read_to_string(filename).expect("File not found");    


    
}
