/*
 * File: capture_environment.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Closures capture the environment and use extra memory to store that context.
 * FnOnce -> takes ownership of the variables inside a closures environment (not more than once)
 * FnMut -> mut borrows variables
 * Fn -> borrows variables
 * move |...| -> forces closure to take ownership
 */

pub(crate) fn capture_environment() {
    closure_that_captures_environment();
    closure_that_is_not_forced_to_take_ownership();
    closure_that_is_forced_to_take_ownership();
}

fn closure_that_is_not_forced_to_take_ownership() {
    println!();
    let x = vec![1, 2, 3];
    let equal_to_x = |z| z == x;
    println!("Can't use x here: {:#?}", x);
    let y = vec![1, 2, 3];
    println!("{}", equal_to_x(y));
}

fn closure_that_is_forced_to_take_ownership() {
    println!();
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // ERROR: borrow of moved value: `x`
    //println!("Can't use x here: {:#?}", x);
    let y = vec![1, 2, 3];
    println!("{}", equal_to_x(y));
}

fn closure_that_captures_environment() {
    println!();
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("{}", equal_to_x(y));
}

/*
fn fn_that_captures_environment() {
    let x = 4;
    // ERROR: can't capture dynamic environment in a fn item E0434
    // Help: use the `|| { ... }` closure form instead
    fn equal_to_x(z: i32) -> bool {
        z == x
    }
    let y = 4;
    println!("{:#?}", assert!(equal_to_x(y)));
}
 */
