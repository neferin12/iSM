mod students;
mod seminars;

use std::ops::Add;
use crate::constants::Points;
use crate::types::{Seminar, Student, RismResult, SeminarType};
use z3::{Config, Context, Optimize, SatResult};
use z3::ast::{Int, Bool, Ast};
use crate::rism_model_checking::seminars::sort_votes_to_seminars;
use crate::rism_model_checking::students::{votes_to_string_id, student_to_points_id, assert_student_votes, assert_student_points};

pub fn run_model_check<'a>(students: &Vec<Student>, seminars: &Vec<Seminar>, points: Points) -> Option<RismResult<'a>> {
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    cfg.set_bool_param_value("parallel.enable", true);
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
            &Bool::pb_le(&ctx,&*t,seminar.capacity as i32)
        )
    });

    // opt.assert(&total_points.lt(&Int::from_u64(&ctx, 300)));
    opt.minimize(&total_points);

    if let SatResult::Sat = opt.check(&[]) {
        let model = opt.get_model().unwrap();
        // println!("{:?}", model);

        let s = &students[0];
        println!("Details for {}", s.name);
        println!("Practical: {:?}",
                 votes_to_string_id(s, &SeminarType::Practical)
                     .iter()
                     .map(|s| votes.get(s).unwrap())
                     .map(|v| model.get_const_interp(v).unwrap())
                     .collect::<Vec<Bool>>());
        println!("Scientific: {:?}",
                 votes_to_string_id(s, &SeminarType::Scientific)
                     .iter()
                     .map(|s| votes.get(s).unwrap())
                     .map(|v| model.get_const_interp(v).unwrap())
                     .collect::<Vec<Bool>>());
        println!("Practical points: {:?}", model.get_const_interp(student_points.get(&student_to_points_id(s, SeminarType::Practical)).unwrap()));
        println!("Scientific points: {:?}", model.get_const_interp(student_points.get(&student_to_points_id(s, SeminarType::Scientific)).unwrap()));

        println!("Total Points: {:?}", opt.get_model().unwrap().get_const_interp(&total_points));
    } else {
        // println!("{:?}", opt.get_proof().unwrap());
        println!("Unsatisfieable or unknown result!")
    };
    return None;
}
