pub fn counting_sort<T: Clone, F: Fn(&T) -> usize>(
    data: &mut [T],
    n_keys: usize,
    get_key: &F,
    ) {
    let mut buffer = Vec::<T>::with_capacity(data.len());
    unsafe {
        use std::ptr;
        buffer.set_len(data.len());
        
        let mut counters = vec![0; n_keys];
        //let mut counters = &mut vec![0; n_keys];
        for x in data.iter() {
            counters[get_key(x)] += 1;
        }
        for i in 1..counters.len() {
            counters[i] += counters[i - 1];
        }
        for x in data.iter().rev() {
            let c = &mut counters[get_key(x)];
            *c -= 1;            
            ptr::copy_nonoverlapping(x, &mut buffer[*c], 1);
        }
        
        ptr::copy_nonoverlapping(&buffer[0], &mut data[0], data.len());
        buffer.set_len(0);
    }
}
