/*
 * File: workout.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

use rand::Rng;

pub(crate) fn workout() {
    let intensity = 10;
    let random_number = rand::thread_rng().gen_range(0..4);
    generate_workout_simple(intensity, random_number);
    generate_workout_variable(intensity, random_number);
    generate_workout_closure_bad(intensity, random_number);
    generate_workout_closure_memorization_non_dynamic(intensity, random_number);
    generate_workout_closure_memorization_dynamic(intensity, random_number);
    mismatched_types_closure();
}
/* Cache a hashmap of key: arg, value: value to circumvent issue in CacheNonDynamic<T>.
 * Also use generics.
 */

struct CacheDynamic<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Clone,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> CacheDynamic<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Clone,
{
    fn new(calculation: T) -> CacheDynamic<T, U, V> {
        CacheDynamic {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> V {
        let v = self
            .value
            .entry(arg)
            .or_insert_with(|| (self.calculation)(arg));
        v.clone()
    }
}

fn generate_workout_closure_memorization_dynamic(intensity: u32, random_number: u32) -> u32 {
    println!();
    let mut cached_result = CacheDynamic::new(|num| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} push-ups!", cached_result.value(intensity));
        println!("Next, do {} sit-ups!", cached_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", cached_result.value(intensity));
    }
    cached_result.value(intensity)
}
/*
 * Making a cache as a struct to store closure's output. This struct has to use generics(<T>) and
 * trait bounds(where T: Fn(u32) -> u32 - here it is Fn(u32)).
 * This and the below function are used to only execute the closure/fn when necessary.
 * However this results in caching of the closure with the first specified arg.
 * This also uses hardcoded types (u32), which can also be a problem.
 */

struct CacheNonDynamic<T>
// Fn Generic types: Fn(), FnMut(), FnOnce()
where
    T: Fn(u32) -> u32,
{
    /* calculation will store the generic type. Thus it can be any closure (where T: Fn(u32) -> u32)
     * It can also store regular fn()
     */
    calculation: T,
    /* value is Option<> because when CacheNonDynamic<> is initialized it will be None; after calculation has
     * been called it will be the result of that
     */
    value: Option<u32>,
}

impl<T> CacheNonDynamic<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CacheNonDynamic<T> {
        CacheNonDynamic {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // Call closure - calculation (stored in CacheNonDynamic<T>) - inside self and parse arg
                let v = (self.calculation)(arg);
                // Cache the value of v in CacheNonDynamic<T>
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_closure_memorization_non_dynamic(intensity: u32, random_number: u32) {
    println!();
    // Has to be mut because CacheNonDynamic::value() which mutates CacheNonDynamic<T>
    let mut cached_result = CacheNonDynamic::new(|num| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} push-ups!", cached_result.value(intensity));
        println!("Next, do {} sit-ups!", cached_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", cached_result.value(intensity));
    }
}

/*
 * We want a way to define our code in one place, but only execute it when necessary.
 * This doesn't solve the problem, but gives us an idea how to define closures.
 */

fn generate_workout_closure_bad(intensity: u32, random_number: u32) {
    println!();
    /*
     * We are replicating simulated_expensive_calculation(...) here. (defining our code)
     * The input parameter is inside of the |...|. expensive_closure is only storing the closure,
     * not the result
     * Syntax for calling is similar to a fn.
     * You don't need to annotate input or result values. Functions are part of an explicit interface
     * exposed to users, but closures are short and only relevant within a narrow context.
     */
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // This is the same with explicit types.
    /*
    let expensive_closure = |num: u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
     */
    if intensity < 25 {
        println!("Today, do {} push-ups!", expensive_closure(intensity));
        println!("Next, do {} sit-ups!", expensive_closure(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
}
/*
 * This example is better because it stores the result as a variable. However, there are cases where
 * the function doesn't need to be called at all, so there are also unnecessary calls.
 */

fn generate_workout_variable(intensity: u32, random_number: u32) {
    println!();
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} push-ups!", expensive_result);
        println!("Next, do {} sit-ups!", expensive_result);
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_result);
    }
}
/*
 * This example is bad. The expensive calculation takes 2s. This function gets called multiple
 * times (also unnecessarily). Therefore it is necessary to call it as few times as possible.
 */

fn generate_workout_simple(intensity: u32, random_number: u32) {
    println!();
    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} sit-ups!",
            simulated_expensive_calculation(intensity)
        );
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            simulated_expensive_calculation(intensity)
        );
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn mismatched_types_closure() {
    println!();
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    /* ERROR: mismatched types; Closures can only have one type for input variables
     * The first type will define the type of the input parameter.
     */
    //let n = example_closure(2);
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let intensity = 1;
        let random_number = rand::thread_rng().gen_range(0..4);
        let v1 = generate_workout_closure_memorization_dynamic(intensity + 1, random_number);
        let v2 = generate_workout_closure_memorization_dynamic(intensity + 2, random_number);
        let v3 = generate_workout_closure_memorization_dynamic(intensity + 1, random_number);
        let mut c = CacheDynamic::new(|a: &str| a.len());
        let v4 = c.value("string literal");
        let v5 = c.value("another string literal");
        let v6 = c.value("string literal");
        assert_ne!(v1, v2);
        assert_eq!(v1, v3);
        assert_ne!(v4, v5);
        assert_eq!(v4, v6);
    }
}
