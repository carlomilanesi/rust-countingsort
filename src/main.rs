mod countingsort;
use countingsort::*;
extern crate rand;
use rand::Rng;

/*
struct S {
    s: String,
    d: f64,
}

fn create_struct_vect(inp: &[i32]) -> Vec<S> {
    let mut out = Vec::new();
    for i in inp {
        out.push(S {s: String::from(""), d: *i as f64});
    }
    out
}
*/

fn main() {
    let mut v1 = [59, 13, 40, -34, 6, -1, 0, 5, 55, 8, 0, 98, -1, -1, -1, 5];
//    let mut v1s = create_struct_vect(&v1);
    counting_sort(&mut v1, 200, |x| (x + 100) as usize);
    assert_eq!([-34, -1, -1, -1, -1, 0, 0, 5, 5, 6, 8, 13, 40, 55, 59, 98], v1);
    let mut error_count = 0;
    let mut v2 = vec![0; 250];
    for item in &mut v2 {
        *item = rand::thread_rng().gen_range(-100, 100);
    }
    for _ in 0..4_000 {
        let mut v3 = vec![0; v2.len()];
        for i in 0..v2.len() {
            v3[i] = v2[i];
        }
        //v3 = v2.clone();
        counting_sort(&mut v3, 200, |x| (x + 100) as usize);

        //v3.sort();
        for i in 1..v3.len() {
            if v3[i - 1] > v3[i] { error_count += 1; }
        }
    }
    println!("{} errors.", error_count);
}
