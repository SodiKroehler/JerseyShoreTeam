fn parser(input: &str) ->Vec<&str> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s);
    }  
    strings
}
fn main(){
    parser("This is a test purpose sentence");
}