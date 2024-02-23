mod results;
pub type RismResult<'b> = results::RismResult<'b>;

mod assignment;
pub type Assignment<'a> = assignment::Assignment<'a>;

mod student;
pub type Student = student::Student;

mod seminar;
pub type Seminar = seminar::Seminar;
pub type SeminarType = seminar::SeminarType;



