use std::fs::File;

pub fn create_file() {
    let f = File::create("./rust.txt").unwrap();
    // match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem when create file Error: {}",error)
    //     }
    // };
}

// pub fn readText() {
//     let mut file = std::fs::File::open("./rust,txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}", contents); 
// }

/**
 * how to use unit testing ==> cargo test
 */
#[test]
pub fn unit_test(){
    println!("This content fom unit test.......")
}