/*
 * closures is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/closures/blob/main/LICENSE
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
