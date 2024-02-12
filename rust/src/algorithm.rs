use crate::types::{Assignment, Iteration, Seminar, Student};
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::Points;

fn try_assignment<'a, 'b: 'a>(student: &Student, seminar: &'b Seminar, iteration: &'a mut Iteration<'a>, points: u16) -> bool {
    if iteration.decrease_capacity_if_left(seminar) {
        iteration.assign_student_to_seminar(student, seminar, points);
        return true;
    }
    return false;
}

pub fn run_algorithm(students: &Vec<Student>, seminars: &Vec<Seminar>, iterations: u16, points: Points) {
    let best_iteration: Option<Iteration> = None;

    for i in 0..iterations {
        let mut sc = students.clone();
        sc.shuffle(&mut thread_rng());

        let mut iteration: Iteration = Iteration::new(Vec::new(), &seminars, Vec::new());

        for s in students {
            iteration.assignments[s.id as usize] = Assignment::new(&s);
        }

        for s in sc {
            let w_wish: &Seminar = &s.w_wishes[0];
            {
                if !try_assignment(&s, w_wish , &mut iteration, points.first_selection) {}
            }
        }
    }
}