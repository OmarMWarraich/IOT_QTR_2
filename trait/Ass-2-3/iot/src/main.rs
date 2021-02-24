#[derive(Debug)]
struct IotStudent {
    name : String,
    age: u32,
    education: String
}
struct IotInstructor {
    name: String,
    age : u32
}
pub trait Questions {
    fn ask_Question(&self, name: String) -> String {
        format!("{} Zoom session will be live, Zoom recording will not be available. quarter 2 Studio recorded videos are available", name)
    }
}

impl Questions for IotInstructor { 
    fn ask_Question(&self, name: String) -> String {
    format! ("{} In case of any issue email to education@piaic.org", name)
    }
}

impl Questions for IotStudent {}
fn main() {
    let student = IotStudent {
        name: String::from("Omar"),
        age: 38,
        education: String::from("Hobbyist"),
    };
    let student1 = IotStudent {
        name: String::from("Ali"),
        age: 33,
        education: String::from("IOT Specialist"),
    };
    let teacher = IotInstructor {
        name: String::from("Sir"),
        age: 20,
    };
    println!("{}", teacher.ask_Question(student.name));
    println!("{}", student1.ask_Question(String::from("Ali")));
}