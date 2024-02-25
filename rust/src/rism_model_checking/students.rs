use std::collections::BTreeMap;
use std::ops::Add;
use z3::ast::{Ast, Bool, Int};
use z3::{Context, Optimize};
use crate::constants::Points;
use crate::rism_model_checking::seminars::seminar_to_string_id;
use crate::types::{SeminarType, Student};

pub(crate) fn student_to_string_id(student: &Student) -> String {
    student.id.to_string()
}

pub(crate) fn vote_to_string_id(student: &Student, seminar_type: &SeminarType, index: usize) -> String {
    let sem;
    if index < 3 {
        sem = Some(match seminar_type {
            SeminarType::Scientific => &student.w_wishes[index],
            SeminarType::Practical => &student.p_wishes[index],
        })
    }else { 
        sem = None
    }
    format!("{0}-{1}-{2}-{3}", student_to_string_id(student), seminar_to_string_id(sem), seminar_type, index)
}

pub(crate) fn student_to_points_id(student: &Student, seminar_type: SeminarType) -> String {
    format!("{0}-{1}-points", student.id, seminar_type)
}

pub(crate) fn votes_to_string_id(student: &Student, seminar_type: &SeminarType) -> Vec<String> {
    let mut stud_votes: Vec<String> = match seminar_type {
        SeminarType::Scientific => &student.w_wishes,
        SeminarType::Practical => &student.p_wishes,
    }
        .iter()
        .enumerate()
        .map(|(i, _s)| vote_to_string_id(student, seminar_type, i))
        .collect();
    stud_votes.push(vote_to_string_id(student, seminar_type, 3));

    stud_votes
}

fn exactly_one_of<'a>(ctx: &'a Context, bools: Vec<Bool<'a>>) -> Bool<'a> {
    // let mut results = Vec::new();
    //
    // for i in 0..bools.len() {
    //     let part = bools
    //         .clone()
    //         .into_iter()
    //         .enumerate()
    //         .map(
    //             |(i2, b)| {
    //                 if i == i2 {
    //                     b
    //                 } else {
    //                     b.not()
    //                 }
    //             })
    //         .reduce(|acc, e| Bool::and(ctx, &[&acc, &e]));
    //     results.push(part.unwrap());
    // }
    //
    // return results.into_iter().reduce(|acc, e| Bool::or(ctx, &[&acc, &e])).unwrap();
    let t: Vec<(&Bool, i32)> = bools.iter().map(|b| (b, 1)).collect();
    return Bool::pb_eq(ctx, &*t, 1);
}

pub(crate) fn assert_student_votes<'a>(ctx: &'a z3::Context, opt: &Optimize, student: &Student) -> BTreeMap<String, Bool<'a>> {
    let vote_keys_p = votes_to_string_id(student, &SeminarType::Practical);
    let vote_keys_w = votes_to_string_id(student, &SeminarType::Scientific);

    let mut votes = BTreeMap::new();

    for k in &vote_keys_p {
        let v = Bool::new_const(&ctx, k.clone());
        votes.insert(k.clone(), v);
    }

    for k in &vote_keys_w {
        let v = Bool::new_const(&ctx, k.clone());
        votes.insert(k.clone(), v);
    }

    opt.assert(
        &exactly_one_of(
            &ctx,
            vote_keys_p.iter().map(|k| votes.get(k).unwrap().clone()).collect(),
        )
    );

    opt.assert(
        &exactly_one_of(
            &ctx,
            vote_keys_w.iter().map(|k| votes.get(k).unwrap().clone()).collect(),
        )
    );

    votes
}

pub(crate) fn assert_student_points<'a>(
    ctx: &'a Context,
    opt: &Optimize,
    student: &'a Student,
    votes: BTreeMap<String, Bool<'a>>,
    points: &Points,
) -> (Int<'a>, BTreeMap<String, Int<'a>>) {
    let point_values = [
        Int::from_u64(&ctx, points.first_selection as u64),
        Int::from_u64(&ctx, points.second_selection as u64),
        Int::from_u64(&ctx, points.third_selection as u64),
        Int::from_u64(&ctx, points.no_selection as u64),
    ];

    let assert_selection_points = |vote_keys: &Vec<String>, point_const: &Int| {
        for i in 0..4 {
            opt.assert(
                &votes.get(&vote_keys[i]).unwrap().implies(
                    &&point_const._eq(&point_values[i])
                )
            );
        }
    };

    let vote_keys_p = votes_to_string_id(student, &SeminarType::Practical);
    let vote_keys_w = votes_to_string_id(student, &SeminarType::Scientific);

    let mut student_points = BTreeMap::new();

    let key_w = student_to_points_id(student, SeminarType::Scientific);
    let s_points_w = Int::new_const(&ctx, key_w.clone());
    student_points.insert(key_w.clone(), s_points_w);

    let s_points_w = student_points.get(&key_w).unwrap();
    assert_selection_points(&vote_keys_w, s_points_w);

    let key_p = student_to_points_id(student, SeminarType::Practical);
    let s_points_p = Int::new_const(&ctx, key_p.clone());
    student_points.insert(key_p.clone(), s_points_p);

    let s_points_p = student_points.get(&key_p).unwrap();
    assert_selection_points(&vote_keys_p, s_points_p);

    (
        student_points
            .get(&key_p.clone())
            .unwrap()
            .add(
                student_points.get(&key_w).unwrap()
            ),
        student_points
    )
}