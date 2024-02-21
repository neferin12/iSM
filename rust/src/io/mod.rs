use std::fs;
use crate::types::{Seminar, SeminarType, Student};

pub fn import_students(students_path: &str, seminars: &Vec<Seminar>) -> Vec<Student> {
    println!("Importing students from {students_path}");
    let mut students: Vec<Student> = Vec::new();

    let contents = fs::read_to_string(students_path)
        .expect(&*format!("Could not read file {students_path}"));
    let w_seminars: Vec<&Seminar> = seminars.iter().filter(|s| s.seminar_type == SeminarType::Scientific).collect();
    let p_seminars: Vec<&Seminar> = seminars.iter().filter(|s| s.seminar_type == SeminarType::Practical).collect();

    let mut lines = contents.lines();
    let mut id_counter: u16 = 0;

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(";").collect();

        let w_wishes: Vec<Seminar> = split[1..=3].iter().map(|id| w_seminars[id.parse::<usize>().unwrap()].clone()).collect();
        let p_wishes: Vec<Seminar> = split[4..=6].iter().map(|id| p_seminars[id.parse::<usize>().unwrap()].clone()).collect();

        let student = Student::new(id_counter, String::from(split[0]), w_wishes, p_wishes);
        students.push(student);

        id_counter += 1;
    }

    return students;

}

pub fn import_seminars(seminars_path: &str) -> Vec<Seminar> {
    println!("Importing seminars from {seminars_path}");
    let mut seminars: Vec<Seminar> =Vec::new();
    let contents = fs::read_to_string(seminars_path)
        .expect(&*format!("Could not read file {seminars_path}"));
    let mut lines = contents.lines();
    let mut id_counter: u16 = 0;

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(";").collect();
        let seminar_type = match split[2] {
            "W" => SeminarType::Scientific,
            "P" => SeminarType::Practical,
            _ => panic!("Invalid seminar type")
        };
        let seminar = Seminar::new(id_counter, String::from(split[0]), split[1].parse::<u16>().unwrap(), seminar_type);
        id_counter += 1;

        seminars.push(seminar);
    }

    return seminars;
}