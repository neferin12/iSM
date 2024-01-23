use std::fs;

#[derive(PartialEq, Debug)]
pub enum SeminarType {
    Scientific,
    Practical
}

#[derive(Debug)]
pub struct Seminar {
    name: String,
    capacity: u16,
    id: u16,
    seminar_type: SeminarType
}

pub fn import_students(students_path: &str, seminars: Vec<Seminar>) {
    println!("Importing students from {students_path}");
    let contents = fs::read_to_string(students_path)
        .expect(&*format!("Could not read file {students_path}"));
    let w_seminars: Vec<&Seminar> = seminars.iter().filter(|s| s.seminar_type == SeminarType::Scientific).collect();
    let p_seminars: Vec<&Seminar> = seminars.iter().filter(|s| s.seminar_type == SeminarType::Practical).collect();

    let mut lines = contents.lines();
    let mut id_counter: u16 = 0;

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(";").collect();

        id_counter += 1;
    }

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
        let seminar = Seminar {
            name: String::from(split[0]),
            capacity: split[1].parse::<u16>().unwrap(),
            id: id_counter,
            seminar_type,
        };
        id_counter += 1;

        seminars.push(seminar);
    }

    return seminars;
}