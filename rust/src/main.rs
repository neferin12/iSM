use clap::Parser;
use serde::Serialize;
use rism::rism_classic::run;
use rism::constants::get_default_points;
use rism::io::{import_students, import_seminars};
#[cfg(feature = "model-checking")]
use rism::rism_model_checking::run_model_check;
use console::style;
use tabled::{Table, Tabled};
use tabled::settings::Style;

#[derive(
clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq
)]
#[serde(rename_all = "kebab-case")]
enum ExecutionVariants {
    #[default]
    Classic,

    #[cfg(feature = "model-checking")]
    ModelChecking,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Students file path
    #[arg(short = 'p', long)]
    students_path: String,

    /// Seminars file path
    #[arg(short, long)]
    seminars_path: String,

    /// Number of Iterations
    #[arg(short, long)]
    iterations: u32,

    /// Number of Iterations
    #[arg(short, long, default_value = "1")]
    threads: u16,

    /// Choose variant of calculation
    #[clap(short, long, default_value_t, value_enum)]
    variant: ExecutionVariants,
}

#[derive(Tabled)]
struct PrintableStudent {
    name: String,
    points: u16,
    w_seminar: String,
    p_seminar: String,
}

fn main() {
    let args = Args::parse();
    let seminars = import_seminars(&args.seminars_path);
    let students = import_students(&args.students_path, &seminars);


    let best_iteration = match args.variant {
        ExecutionVariants::Classic => Some(run(&students, &seminars, args.iterations, get_default_points(), args.threads)),
        #[cfg(feature = "model-checking")]
        ExecutionVariants::ModelChecking => run_model_check(&students, &seminars, get_default_points())
    };
    if let Some(bi_unwr) = best_iteration {

        let mut students_table_data = Vec::new();

        for a in &bi_unwr.assignments {
            let ps = PrintableStudent {
                name: a.student.name.clone(),
                points: a.points,
                w_seminar: match a.w_seminar {
                    Some(s) => s.name.clone(),
                    None => "Null".to_string()
                },
                p_seminar: match a.p_seminar {
                    Some(s) => s.name.clone(),
                    None => "Null".to_string()
                },
            };
            students_table_data.push(ps);
        }

        let mut table = Table::new(students_table_data);
        table.with(Style::rounded());
        println!("{}", table.to_string());
        println!("Total points: {}", style(bi_unwr.total_points()).bold().green());
    } else {
        println!("{} Try the classic variant or increase the max points.", style("No result was found!").bold().yellow())
    }
}
