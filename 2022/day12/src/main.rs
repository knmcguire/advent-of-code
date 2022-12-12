fn main() {
    // Get a filename from the command line
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    // Read an input and parse it into a 2d vector of chars
    let input = std::fs::read_to_string(filename).unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

}
