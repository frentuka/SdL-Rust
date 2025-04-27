fn main() {
    println!("Hello, world! xd");
    
    // let word;
    // {
    //     let my_string = String::from("Hello world");
    //     word = test(&my_string);
    // }
    // 
    // println!("asd: {}", word);
}

// lifetime annotations test
// fn test<'a, 'b>(s: &'a str) -> &'b str {
//     let bytes = s.as_bytes();
//     
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     
//     s
// }