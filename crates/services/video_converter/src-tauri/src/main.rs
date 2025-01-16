// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[warn(unused_imports)]

fn main() {
    re_convertor_lib::run()
}
