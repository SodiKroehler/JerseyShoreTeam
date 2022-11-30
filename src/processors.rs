fn TEMP_create_dictionary(){

    let file = File::open("./assets/train.jsonl").expect("Error while reading file");
    let reader = BufReader::new(file);

   
    // let mut data = fs::read_to_string("./assets/train.jsonl").expect("Error while reading file");
    let mut dict = HashMap::new();
    let mut idx :usize = 2;
    // for jline in data.iter(){
    for jline in reader.lines() {
        let point: Line = serde_json::from_str(&jline.unwrap()).unwrap();
        // println!("{:?}", point.question);
        let question = parser(point.question);
        for corp_tok in question.iter(){
            // println!("{:?}", corp_tok);
            if !dict.contains_key(corp_tok){
                dict.insert(corp_tok.to_owned(), idx);
                idx+=1;
            }
        }
        // if idx > 5 {break;}
    }
    let j = serde_json::to_string(&dict).unwrap();
    fs::write("./assets/dictionary.json", j).unwrap();
}



fn read_weights_to_json(filename: String, V: usize){

    let mut raw_weights: String = fs::read_to_string(filename.to_owned()).unwrap();

    raw_weights = raw_weights.replace(&['\n', '\r'][..], " ");
    let mut weights = vec![vec![0.0; 100]; V];
    let mut weights_raw: Vec<&str> = raw_weights.split(" ").collect();
    let mut i = 0;
    let mut j = 0;
    for w in weights_raw.iter_mut(){
        if w.is_empty() {continue;}
        let s = w.parse::<f64>().unwrap();
        weights[j][i] = s;
        i+=1;
        if (i >= 100){
            j+=1;
            i=0;
        }
    }

    let k = serde_json::to_string(&weights).unwrap();
    fs::write("./assets/weights.json", k).unwrap();
}