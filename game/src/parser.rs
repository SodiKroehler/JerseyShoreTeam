fn parser(input: &str) ->Vec<&str> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s);
    }  
    strings.clone()
}
fn stemmer(mut strings: Vec<&str>) ->Vec<&str>  {
    let mut i=0;
    let mut new_strings=Vec::new();
    let stopword = vec!["a","about","above","across","after","afterwards","again","against","all", "almost","purpose"];
    for s in strings{
         if stopword.contains(&&s)==false{
              new_strings.push(s);
         }
        i+=1;
    }
    new_strings
}
fn main(){
    let mut strings=parser("This is a test purpose sentence");
    strings=stemmer(strings);
    println!("{:?}", strings);
}