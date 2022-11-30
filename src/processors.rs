fn TEMP_create_dictionary(){
     let mut data = fs::read_to_string("./assets/corpus.txt").expect("Error while reading file");
     data = data.replace(&['(', ')', ',', '"', '.', ';', ':', '\''][..], "");
 
     let corpus_tokens = stemmer(parser(data));
     let mut dict = HashMap::new();
     let mut idx = 0.0;
     for corp_tok in corpus_tokens.iter(){
         if !dict.contains_key(corp_tok){
             dict.insert(corp_tok.to_owned(), idx);
             idx+=1;
         }
         // let tempvar = idx+=1;
         // *dict.entry(corp_tok.to_owned()).or_insert_with_key(|key| tempvar));
     }
     let j = serde_json::to_string(&dict).unwrap();
     fs::write("./assets/dictionary.json", j).unwrap();
 }

fn TEMP_update_questions(){
    let raw_qa_list: String = fs::read_to_string("./assets/questions_answers.json").unwrap();
    let qa_json = serde_json::from_str::<Dict<Question>>(&raw_qa_list).unwrap();
    for p in qa_json.items.iter() {
        let lil_ran_through_sentence = indexer(stemmer(parser(p.question.clone())));
    }
    let j = serde_json::to_string(&qa_json).unwrap();
    fs::write("./assets/qa_list.json", j).unwrap();
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