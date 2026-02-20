trait Course {
    fn get_overview(&self) -> String;

}

struct Workshop {
title: String,
instructor: String,
duration: u8,
}

struct Seminar {
title: String,
instructor: String,
location: String,
}

impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!("Workshop on {} by {} for {} hours", self.title, self.instructor, self.duration)
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!("Seminar on {} by {} at {}", self.title, self.instructor, self.location)
    }
}


fn main() {
    let workshop = Workshop {
        title: String::from("Rust Programming"),
        instructor: String::from("John Doe"),
        duration: 10,
    };
    let seminar = Seminar {
        title: String::from("Rust Programming"),
        instructor: String::from("John Doe"),
        location: String::from("New York"),

    };

    println!("Workshop overview: {}", workshop.get_overview());
    println!("Seminar overview: {}", seminar.get_overview());


}

fn display_course_overview(course: &impl Course){
    println!("Course overview: {}", course.get_overview());
}

/* 
fn display_course_overview<T: Course>(course: &T){
    println!("Course overview: {}", course.get_overview());
}
*/