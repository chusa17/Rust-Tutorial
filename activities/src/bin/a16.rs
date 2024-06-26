// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct student {
    name: Option<String>,
    locker: Option<i32>
}

impl student{
    fn est()->Self {
        Self{
        name: Some("Gala".to_owned()),
        locker: Some(19)
        }
    }
}

fn main() {
    let estudiant = student{
        name: Some("Josep".to_owned()),
        locker: Some(23)
    };
    let G = student::est();
    match G.locker {
        Some(algo) => println!("La taquilla es: {:?}", algo),
        None => println!("No te taquilla"),
    };

}
