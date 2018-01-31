fn main() {
    println!(" ");
    println!("--> TESTING STRINGS APP");
    println!(" ");

    println!("--> 8.2 - Strings Store UTF-8 Encoded Text");
    println!(" ");   

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    println!("data.to_string -  {}", s);
    println!(" "); 

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!(" ");
    println!("'initial contents'.to_string() - {}", s);
    println!(" ");
    let s = String::from("initial contents");
    println!("String::from('initial contents') - {}", s);
    println!(" ");
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    println!(" ");
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    println!(" ");    
    let hello = String::from("Hello");
    println!("{}", hello);
    println!(" ");    
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    println!(" ");    
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    println!(" ");    
    let hello = String::from("こんにちは");
    println!("{}", hello);
    println!(" ");    
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    println!(" ");    

    println!("-- Updating a String");
    println!(" ");        
 
    let mut s = String::from("foo");
    println!("printing variable - {}", s);
    s.push_str("bar");   
    println!("pushing 'bar' - {}", s);

    let mut s1 = String::from("foo");
    println!("s1 - {}", s1);
    let s2 = "bar";
    println!("s2 before pushing - {}", s2);
    s1.push_str(&s2);
    println!("s1 + s2 - {}", s1);
    println!("s2 after pushing is {}", s2);    

    let mut s = String::from("lorota");
    s.push('s');
    println!("s after pushing method 's' single character - {}", s);

    println!("-- Concatenation with the + Operator or the format! Macro");
    println!(" ");         

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
    println!("s1 + s2 = s3 - {}", s3);
    println!("s2 - {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;    
    println!("s1 + s2 + s3 = s - {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);    
    println!("s1 + s2 + s3 = s - {}", s);

    let len = String::from("Hola").len();
    println!("length of 'Hola' is - {}", len);

    // that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value takes two bytes of storage. 
    let len = String::from("Здравствуйте").len();
    println!("length of 'Здравствуйте' is - {}", len);

    let hello = "Здравствуйте";

    let s = &hello[0..12];

    println!("{}", s);

    println!("-- Methods for Iterating Over Strings");
    println!(" ");         

    println!("नमस्ते - characteres");
    println!(" ");         
    for c in "नमस्ते".chars() {
        println!("{}", c);  
    };
    println!(" ");     
    println!("नमस्ते bytes");     
    for c in "नमस्ते".bytes() {
        println!("{}", c);  
    };      
}
