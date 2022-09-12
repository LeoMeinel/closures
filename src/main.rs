/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Closures are like functions but anonymous(no name) - can be stored as variables and be passed
 * around. They capture the variables inside the current scope.
 */

use crate::capture_environment::capture_environment;
use crate::workout::workout;

mod capture_environment;
mod workout;

fn main() {
    workout();
    capture_environment();
}
