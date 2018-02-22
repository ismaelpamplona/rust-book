// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn main() { //DUPLICATING CODE =(
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn largest(list: &[i32]) -> i32 { //SOLVING THE PROBLEM
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }

// fn largest_i32(list: &[i32]) -> i32 { //DUPLICATING CODE =(
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char { // SAME BODY =/
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(result, 100);

//     let char_list = vec!['y', 'm', 'a','z', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
//     assert_eq!(result, 'z');
// }

// use std::cmp::PartialOrd;
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { //SOLVING THE PROBLEM =D
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// struct Point<T> { // WILL WORK
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// struct Point<T> { // WONT WORK
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }

// struct Point<T, U> { // WILL WORK
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());

//     let z = Point { x: 50.5, y: 100.5 };

//     println!("distance_from_origin = {}", z.distance_from_origin());    
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// #![allow(unused_variables)]
// fn main() {
//     pub trait Summarizable {
//         fn summary(&self) -> String {
//             String::from("(Read more porra...)")
//         }
//     }

//     pub struct NewsPornArticle {
//         pub headline: String,
//         pub location: String,
//         pub author: String,
//         pub content: String,
//     };

//     impl Summarizable for NewsPornArticle {
//         fn summary(&self) -> String {
//             format!("{}, by {} ({})", self.headline, self.author, self.location)
//         }
//     };

//     pub struct Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool,
//     };

//     impl Summarizable for Tweet {
//         fn summary(&self) -> String {
//             format!("{}: {}", self.username, self.content)
//         }
//     };

//     pub struct NewsArticle {
//         pub headline: String,
//         pub location: String,
//         pub author: String,
//         pub content: String,
//     };
//     impl Summarizable for NewsArticle {}

//     let xoxota = NewsPornArticle {
//         headline: String::from("Xoxota Cabeuda"),
//         location:String::from("Sede das Brasileirinhas"),
//         author:String::from("Kid"),
//         content:String::from("Teste dessa putaria toda!"),
//     };

//     let caceta = Tweet {
//         username:String::from("kidbengala"),
//         content:String::from("I am is a famous adult star..."),
//         reply: true,
//         retweet: true,
//     };    

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from("The Pittsburgh Penguins once again are the best
//         hockey team in the NHL."),
//     };    

//     println!("1 new NewsPornArticle: {}", xoxota.summary());
//     println!("1 new tweet: {}", caceta.summary());
//     println!("New article available! {}", article.summary());
// }

// fn main() {
//     {
//         let r;         // -------+-- 'a
//                     //        |
//         {              //        |
//             let x = 5; // -+-----+-- 'b
//             r = &x;    //  |     |
//         }              // -+     |
//                     //        |
//         println!("r: {}", r); // |
//                     //        |
//                     // -------+
//     }
// }


// fn main() {
//     {
//         let x = 5;            // -----+-- 'b
//                             //      |
//         let r = &x;           // --+--+-- 'a
//                             //   |  |
//         println!("r: {}", r); //   |  |
//                             // --+  |
//     }  
// }

fn main() {
    let string1 = String::from("xoxota");
    let string2 = "xoxota_cabeluda";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}