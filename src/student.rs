pub struct Student{
    id:i64,
    name:String,
    age:i32
}

impl Student{
    pub fn new(id:i64,name:String,age:i32) -> Student{
        Student{
            id:id,name:name,age:age
        }
    }

    pub fn formate(&self){
        println!("id:{} name:{} age:{} \n",self.id,self.name,self.age)
    }
}