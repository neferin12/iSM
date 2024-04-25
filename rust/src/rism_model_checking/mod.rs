mod students;
mod seminars;

use std::ops::Add;
use crate::constants::Points;
use crate::types::{Seminar, Student, RismResult, SeminarType, Assignment};
use z3::{Config, Context, Optimize, SatResult};
use z3::ast::{Int, Bool, Ast};
use crate::rism_model_checking::seminars::sort_votes_to_seminars;
use crate::rism_model_checking::students::{votes_to_string_id, student_to_points_id, assert_student_votes, assert_student_points};

pub fn run_model_check<'a>(students: &'a Vec<Student>, seminars: &'a Vec<Seminar>, points: Points) -> Option<RismResult<'a>> {
    z3::set_global_param("parallel.enable", "true");

    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);
    // let opt = Solver::new(&ctx);

    let mut s_points_vec: Vec<Int> = Vec::new();

    let votes = students.iter().map(|s| {
        assert_student_votes(&ctx, &opt, s)
    }).reduce(|mut acc, mut e| {
        acc.append(&mut e);
        acc
    }).unwrap();

    let student_points_map = students.iter().map(|s| {
        assert_student_points(&ctx, &opt, s, votes.clone(), &points)
    });

    student_points_map.clone().for_each(|(p, _)| s_points_vec.push(p));

    let student_points =
        student_points_map
            .map(|(_, tree)| tree)
            .reduce(|mut acc, mut e| {
                acc.append(&mut e);
                acc
            }).unwrap();

    let mut total_points_components = Int::from_u64(&ctx, 0);

    for p in s_points_vec {
        total_points_components = total_points_components.add(p);
    };

    let total_points = Int::new_const(&ctx, "total_points");

    opt.assert(&total_points._eq(&total_points_components));


    sort_votes_to_seminars(&votes, students, seminars)
        .iter()
        .for_each(|(seminar, votes)| {
            let t: Vec<(&Bool, i32)> = votes.iter().map(|(_, b)| (*b, 1)).collect();
            opt.assert(
                &Bool::pb_le(&ctx, &*t, seminar.capacity as i32)
            )
        });

    // opt.assert(&total_points.lt(&Int::from_u64(&ctx, 300)));
    opt.minimize(&total_points);

    if let SatResult::Sat = opt.check(&[]) {
        let model = opt.get_model().unwrap();
        // println!("{:?}", model);

        let capacities: Vec<Option<u16>> = seminars.iter().map(|_| Some(0)).collect();

        let assignments: Vec<Assignment> = students.iter().map(|student| {
            let p_points = model.get_const_interp(student_points.get(&student_to_points_id(student, SeminarType::Practical)).unwrap()).unwrap();
            let w_points = model.get_const_interp(student_points.get(&student_to_points_id(student, SeminarType::Scientific)).unwrap()).unwrap();

            let mut a = Assignment::new(student);
            let total_points = w_points.as_u64().unwrap() + p_points.as_u64().unwrap();
            a.points = u16::try_from(total_points).unwrap();

            let p_results =
                votes_to_string_id(student, &SeminarType::Practical)
                    .iter()
                    .map(|s| votes.get(s).unwrap())
                    .map(|v| model.get_const_interp(v).unwrap())
                    .collect::<Vec<Bool>>();
            let w_results =
                votes_to_string_id(student, &SeminarType::Scientific)
                    .iter()
                    .map(|s| votes.get(s).unwrap())
                    .map(|v| model.get_const_interp(v).unwrap())
                    .collect::<Vec<Bool>>();

            for (ind, p_result) in p_results.iter().enumerate() {
                if p_result.as_bool().unwrap() {
                    if ind < student.p_wishes.len(){
                        a.p_seminar = Some(&student.p_wishes[ind]);
                    }else {
                        a.p_seminar = None;
                    }
                    break;
                }
            }
            for (ind, w_result) in w_results.iter().enumerate() {
                if w_result.as_bool().unwrap() {
                    if ind < student.w_wishes.len() {
                        a.w_seminar = Some(&student.w_wishes[ind])
                    }else {
                        a.w_seminar = None;
                    }
                }
            }
            return a;
        }).collect();

        let mut res = RismResult::new(assignments, seminars, capacities);
        res.calculate_points();
        return Some(res);
    } else {
        // println!("{:?}", opt.get_proof().unwrap());
        println!("Unsatisfieable or unknown result!")
    };
    return None;
}
