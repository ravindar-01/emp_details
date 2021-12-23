// use std::env;
// use std::fs;
extern crate clap;

use clap::{Arg, App};

fn main() {
    // basic app information
    let app = App::new("hello-clap")
        .version("1.0")
        .about("Says hello")
        .author("Ravindar");

    // Define the command line option
    let emp = Arg::with_name("e")
        .long("e") // allow --name
        .takes_value(true)
        .help("The employee file path")
        .required(true);
    let dept= Arg::with_name("d")
        .long("d") // allow --name
        .takes_value(true)
        .help("The department file path")
        .required(true);
    let slr = Arg::with_name("s")
        .long("s") // allow --name
        .takes_value(true)
        .help("The salary file path")
        .required(true);
    let leave = Arg::with_name("l")
        .long("l") // allow --name
        .takes_value(true)
        .help("The leave file path")
        .required(true);
    let opt = Arg::with_name("o")
        .long("o") // allow --name
        .takes_value(true)
        .help("The output file path")
        .required(true);

    // now add in the argument we want to parse
    let app = app.args(&[emp,dept,slr,leave,opt]);
    
    // extract the matches
    let matches = app.get_matches();

    // Extract the actual name
    let emp_path = matches.value_of("e")
        .expect("This can't be None, we said it was required");
    let dept_path = matches.value_of("d")
        .expect("This can't be None, we said it was required");
    let slr_path = matches.value_of("s")
        .expect("This can't be None, we said it was required");
    let leave_path = matches.value_of("l")
        .expect("This can't be None, we said it was required");
    let output_path = matches.value_of("o")
        .expect("This can't be None, we said it was required");
    println!("This is the employee path {}",emp_path);
    println!("This is the department path {}",dept_path);
    println!("This is the salry path {}",slr_path);
    println!("This is the leave path {}",leave_path);
    println!("This is the output  file path {}",output_path);

}




// fn read_the_cmdinput()-> Vec<String> {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
//     return args;

// }
// // read the files data
// // fn read_file(path:&str)-> String{
// //   println!("{}",path);
// //     let contents = fs::read_to_string(path)
// //         .expect("Something went wrong reading the file");
// //     // println!("With text:\n{}", contents);
// //     return contents;
// // }






// fn main() {
//      // read the command line inputs
//     // println!("Hello, wlet args: Vec<String> = env::args().collect();
    
//     // let args: Vec<String> = env::args().collect();
//     // println!("{:?}", args);
//     let dpt_path:&str="/home/dell/assignment1/project files/dep.xlsx";
//     // let dpt_path:&str="/home/dell/assignment1/project files/new.txt";
//     // let leave_path:&str="/home/dell/assignment1/project files/new.txt";
//     // let slr_path:&str="/home/dell/assignment1/project files/new.txt";
//     // println!("{}",emp_data);
// }
