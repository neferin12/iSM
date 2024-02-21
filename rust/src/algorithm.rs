use crate::types::{Assignment, Iteration, Seminar, Student};
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::Points;
use std::cmp;

fn find_possible_assignment<'a>(wishes: &'a Vec<Seminar>, points: &Points, iteration: &Iteration) -> (Option<&'a Seminar>, u16) {
    return if iteration.get_capacity(&wishes[0]) > 0 {
        (Some(&wishes[0]), points.first_selection)
    } else if iteration.get_capacity(&wishes[1]) > 0 {
        (Some(&wishes[1]), points.second_selection)
    } else if iteration.get_capacity(&wishes[2]) > 0 {
        (Some(&wishes[2]), points.third_selection)
    } else {
        (None, points.no_selection)
    };
}

pub fn run_algorithm<'a>(students: &'a Vec<Student>, seminars: &'a Vec<Seminar>, iterations: u32, points: Points) -> Iteration<'a> {
    let mut best_iteration: Option<Iteration> = None;

    for _ in 0..iterations {
        let mut shuffled_indices: Vec<usize> = (0..students.len()).collect();
        shuffled_indices.shuffle(&mut thread_rng());

        let mut iteration: Iteration = Iteration::new(
            students.iter().map(|s| Assignment::new(s)).collect(), 
            &seminars, 
            vec![None; seminars.len()]);
        
        for i in &shuffled_indices {
            let s = &students[*i];
            let w_wishes: &Vec<Seminar> = &s.w_wishes;
            let (assigned_seminar_opt, assigned_points) = find_possible_assignment(w_wishes, &points, &iteration);

            iteration.assign_student_to_seminar(s, assigned_seminar_opt, assigned_points);
        }

        iteration.assignments.sort();
        iteration.assignments.reverse();

        for i in 0..iteration.assignments.len() {
            let s = iteration.assignments[i].student;
            let p_wishes = &s.p_wishes;
            let (assigned_seminar_opt, assigned_points) = find_possible_assignment(p_wishes, &points, &iteration);

            iteration.assignments[i].assign_seminar(assigned_seminar_opt, assigned_points);
        }

        iteration.calculate_points();

        best_iteration = match best_iteration {
            None => { Some(iteration) }
            Some(b_it_unwrap) => { Some(cmp::min(b_it_unwrap, iteration)) }
        }
    }

    return best_iteration.unwrap();
}