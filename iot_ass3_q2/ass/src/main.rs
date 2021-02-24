
// struct Students {
//     name: &'static str,
//     age: &'static str,
//     education: &'static str,
//     timing: &'static str
// }

// impl Students {
//     fn get_name(&self) -> &'static str {return self.name}
    
//     fn get_timing(&self) -> &'static str {return self.timing}
//     fn get_education(&self) -> &'static str {return self.education}
// }

// fn main() {
//     let student1 = Students {name:"Ali", age: "28", education: "BA", timing: "2"};
//     let student2 = Students {name:"Kareem", age: "29", education: "FA", timing: "3"};
//     let student3 = Students {name:"Hafeez", age: "30", education: "CA", timing: "4"};
//     let student4 = Students {name:"Saleem", age: "31", education: "MBA", timing: "5"};
//     let Student = student1.get_name();
//     println!("Name of Student1 is: {:?}", Student );
//     let Student = student2.get_name();
//     println!("Name of Student2 is: {:?}", Student );
//     let Student = student3.get_name();
//     println!("Name of Student3 is: {:?}", Student );
//     let Student = student4.get_name();
//     println!("Name of Student4 is: {:}", Student);
//     let timing = student1.get_timing();
//     println!("Timing of Student 1: {:?}", timing);
//     let timing = student2.get_timing();
//     println!("Timing of Student 2:{:?}", timing);
//     let timing = student3.get_timing();
//     println!("Timing of Student 3: {:?}", timing);
//     let timing = student4.get_timing();
//     println!("Timing of Student 4: {:?}", timing);
//     let education = student1.get_education();
//     println!("Education of Student 1: {:?}", timing);
//     let timing = student2.get_education();
//     println!("Education of Student 2: {:?}", timing);
//     let timing = student3.get_education();
//     println!("Education of Student 3: {:?}", timing);
//     let timing = student4.get_education();
//     println!("Education of Student 4: {:?}", timing);
// }

struct students {
        name: &'static str,
        age: &'static str,
        education: &'static str,
        timing: &'static str
    }
    
    impl students {
        fn get_name(&self) -> &'static str {return self.name}
        
        fn get_timing(&self) -> &'static str {return self.timing}
        fn get_education(&self) -> &'static str {return self.education}
    }
    
    // for student in self.students
    fn main() {
        let student1 = students {name:"Ali", age: "28", education: "BA", timing: "2"};
        let student2 = students {name:"Kareem", age: "29", education: "FA", timing: "3"};
        let student3 = students {name:"Hafeez", age: "30", education: "CA", timing: "4"};
        let student4 = students {name:"Saleem", age: "31", education: "MB", timing: "5"};
        let student = student1.get_name();
        println!("Name of Student1 is: {:?}", student );
        let student = student2.get_name();
        println!("Name of Student2 is: {:?}", student );
        let student = student3.get_name();
        println!("Name of Student3 is: {:?}", student );
        let student = student4.get_name();
        println!("Name of Student4 is: {:}", student);
        let timing = student1.get_timing();
        println!("Timing of Student 1: {:?}", timing);
        let timing = student2.get_timing();
        println!("Timing of Student 2:{:?}", timing);
        let timing = student3.get_timing();
        println!("Timing of Student 3: {:?}", timing);
        let timing = student4.get_timing();
        println!("Timing of Student 4: {:?}", timing);
        let education = student1.get_education();
        println!("Education of Student 1: {:?}", timing);
        let timing = student2.get_education();
        println!("Education of Student 2: {:?}", timing);
        let timing = student3.get_education();
        println!("Education of Student 3: {:?}", timing);
        let timing = student4.get_education();
        println!("Education of Student 4: {:?}", timing);
       
    }
    
    
    