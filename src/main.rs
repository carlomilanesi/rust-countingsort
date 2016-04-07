fn counting_sort_copy_with_counters<T: Clone, F: Fn(&T) -> usize>(
    orig: &mut [T],
    sorted: &mut [T],
    counters: &mut [usize],
    get_key: F
    ) {
    for x in orig.iter() {
        counters[get_key(x)] += 1;
    }
    for i in 1..counters.len() {
        counters[i] += counters[i - 1];
    }
    for x in orig.iter().rev() {
        let c = &mut counters[get_key(x)];
        *c -= 1;
        sorted[*c] = x.clone();
    }
}
 
fn counting_sort_copy<T: Clone, F: Fn(&T) -> usize>(
    orig: &mut [T],
    sorted: &mut [T],
    n_keys: usize,
    get_key: F
    ) {
    let mut counters = vec![0; n_keys];
    counting_sort_copy_with_counters(orig, sorted, &mut counters, get_key);
}

fn counting_sort<T: Clone, F: Fn(&T) -> usize>(
    data: &mut [T],
    n_keys: usize,
    get_key: F) {
    let mut sorted = vec![data[0].clone(); data.len()];
    counting_sort_copy(data, &mut sorted, n_keys, get_key);
    for i in 0..data.len() {
        data[i] = sorted[i].clone();
    }
}

extern crate rand;
use rand::Rng;

fn main() {
    let mut v1 = [59, 13, 40, -34, 6, -1, 0, 5, 55, 8, 0, 98, -1, -1, -1, 5];
    counting_sort(&mut v1, 200, |x| (x + 100) as usize);
    assert_eq!([-34, -1, -1, -1, -1, 0, 0, 5, 5, 6, 8, 13, 40, 55, 59, 98], v1);
    let mut error_count = 0;
    let mut v2 = vec![0; 250];
    for item in &mut v2 {
        *item = rand::thread_rng().gen_range(-100, 100);
    }
    for _ in 0..4_000_000 {
        let mut v3 = vec![0; v2.len()];
        for i in 0..v2.len() {
            v3[i] = v2[i];
        }
        counting_sort(&mut v3, 200, |x| (x + 100) as usize);

        //v3.sort();
        for i in 1..v3.len() {
            if v3[i - 1] > v3[i] { error_count += 1; }
        }
    }
    println!("{} errors.", error_count);
}
