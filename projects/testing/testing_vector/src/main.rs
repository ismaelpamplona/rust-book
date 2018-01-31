fn main() {
    
    println!(" ");
    println!("--> TESTING VECTORS APP");
    println!(" ");

    println!("--> 8.1 - Vectors Store Lists of Values");
    println!(" ");
    
    let mut v = vec![1, 2, 3];

    println!("{:?}", v);
    println!(" ");

    v.push(4);
    v.push(5);
    v.push(6);

    println!("{:?}", v);
    println!(" ");

    println!("--> Dropping a Vector Drops Its Elements");
    println!(" ");

    let mut v = vec!["Buceta", "Caralho", "Puta"];

    println!("{:?}", v);
    println!(" ");

    println!("--> Using indexing syntax or the get method to access an item in a vector");
    println!(" ");

    let v = vec![1, 2, 3, 4, 5];
    let third1: &i32 = &v[2];
    let third2: Option<&i32> = v.get(2);    

    println!("&i32 --> {:?}", third1);
    println!("Option<&i32> --> {:?}", third2);
    println!(" ");


    println!("--> Attempting to access the element at index 100 in a vector containing 5 elements");
    println!(" ");

    let v = vec![1, 2, 3, 4, 5];
   
    println!("Printing nothing...");
    println!(" "); 
   
    // let does_not_exist1 = &v[100];
    // let does_not_exist2 = v.get(100);   

    println!("--> Invalid References");
    println!(" ");

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);

    // println!("{:?}", v);

    println!("--> Iterating Over the Values in a Vector");
    println!(" ");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    };

    println!(" ");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);        
    }

    println!("--> Using an Enum to Store Multiple Types");
    println!(" ");   

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];   

    // print!("{:?}", row); find a way to print row vec at terminal

}
