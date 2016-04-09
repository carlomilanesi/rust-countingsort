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

pub fn counting_sort<T: Clone, F: Fn(&T) -> usize>(
    data: &mut [T],
    n_keys: usize,
    get_key: F) {
    let mut sorted = vec![data[0].clone(); data.len()];
    counting_sort_copy(data, &mut sorted, n_keys, get_key);
    for i in 0..data.len() {
        data[i] = sorted[i].clone();
    }
}
