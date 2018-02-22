fn main() {

    use std::collections::HashMap;

    println!(" ");
    println!("--> TESTING HASH MAPS APP");
    println!(" ");


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams  = vec![String::from("MyEggs"), String::from("Xerereca")];
    let initial_scores = vec![100, 150];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();    

    println!("{:?}", scores);
    println!(" ");

    let field_name = String::from("Favorite soccer team");
    println!("field_name - {:?}", field_name);
    let field_value = String::from("Flamengo");
    println!("field_value - {:?}", field_value);

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map - {:?}", map);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!    

    // println!("field_name - {:?}", field_name);
    // println!("field_value - {:?}", field_value);    

    println!("-- Accessing Values in a Hash Map");
    println!(" ");    

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);    

    println!("score - {:?}", score);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }        

    println!(" ");
    println!("-- Overwriting a Value");
    println!(" ");    
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    println!(" ");
    println!("-- Only Insert If the Key Has No Value");
    println!(" ");
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);    

    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);    

    // will not change the hash map because the Blue team already has the value 10.
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);    

    println!(" ");
    println!("-- Updating a Value Based on the Old Value");
    println!(" ");   

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);    


}
