mod student;
mod io;
use student::Student;


fn main() {
    println!("*****************Rust study*****************");
    //creat a new struct(Student)
    let s = Student::new(1, String::from("向阳"), 15);
    let s2 = Student::new(2, String::from("欧鸿"), 12);

    //create a new vec
    let mut list : Vec<Student> = Vec::new();
    //add elment to vec
    list.push(s);
    list.push(s2);

    //vec foreach
    for i in list{
        println!("{:?}",i.formate())
    };

    //will create file
    io::create_file();

    



    
}
