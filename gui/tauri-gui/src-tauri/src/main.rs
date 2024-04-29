// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::OnceLock;
use std::thread::ThreadId;
use rism::constants::{Points};
use rism::io::{import_seminars, import_students};
use rism::rism_classic::run;
use rism::rism_model_checking::run_model_check;
use serde_json::to_string;
use tauri::{ Manager, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");
static WINDOW: OnceLock<Window> = OnceLock::new();


#[tauri::command]
fn get_version() -> String {
    return String::from(VERSION);
}

#[tauri::command]
fn uses_system_z3() -> bool {
    #[cfg(feature = "bundle_z3")]
    return false;

    #[cfg(not(feature = "bundle_z3"))]
    return true;
}

#[tauri::command]
async fn run_normal(votes: String, seminars: String, iterations: u32, threads: u16) -> String {
    let seminars = import_seminars(&*seminars);
    let students = import_students(&*votes, &seminars);

    let best_iteration = run(&students, &seminars, iterations, Points::default(), threads, send_progress);
    println!("Done");
    to_string(&best_iteration).unwrap()
}

#[tauri::command]
async fn run_model_checking(votes: String, seminars: String) -> String {
    let seminars = import_seminars(&*seminars);
    let students = import_students(&*votes, &seminars);

    let best_iteration = run_model_check(&students, &seminars, Points::default());
    println!("Done");
    to_string(&best_iteration).unwrap()
}

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    thread_id: u64,
    progress: u32,
    total: u32,
}

fn send_progress(t_id: ThreadId, p: u32, t: u32) {
    if p % 10000 == 0 {
        let mut hasher = DefaultHasher::new();
        t_id.hash(&mut hasher);
        WINDOW.get().expect("window is available").emit("progress", ProgressPayload { thread_id: hasher.finish(), progress: p, total: t }).expect("Send message");
    }
}

fn main() {
        tauri::Builder::default()
            .setup(|app| {
                let window = app.get_window("main").unwrap();

                _ = WINDOW.set(window);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![get_version,uses_system_z3,run_normal,run_model_checking])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}

