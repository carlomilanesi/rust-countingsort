mod countingsort;
use countingsort::counting_sort;
extern crate rand;
use rand::Rng;

extern crate stopwatch;
use stopwatch::{Stopwatch};

#[derive(Clone)]
struct S1 {
    i1: i32,
    i2: i32,
}

#[derive(Clone)]
struct S2 {
    s: String,
    d: f64,
    b: bool,
}

fn bench<F: Fn()>(
    descr: &str, snippet: F)
{
    let ms_for_try = 1000;
    let sw = Stopwatch::start_new();
    snippet();
    let n_iter1 = if sw.elapsed_ms() < 1 { ms_for_try } else { 1 };
    let tries = 4;
    let mut maximum_iter_per_second = 0.0;
    let mut second_maximum_iter_per_second = 0.0;
    for _ in 0..tries {
        let sw = Stopwatch::start_new();
        let mut n_iter2 = 0;
        let mut elapsed_ms = sw.elapsed_ms();
        while elapsed_ms < ms_for_try {
            for _ in 0..n_iter1 { snippet() }
            n_iter2 += 1;
            elapsed_ms = sw.elapsed_ms();
        }
        let n_iter_per_second = n_iter1 as f64 * n_iter2 as f64
            * 1000.0 / sw.elapsed_ms() as f64;
        if n_iter_per_second > maximum_iter_per_second {
            second_maximum_iter_per_second = maximum_iter_per_second;
            maximum_iter_per_second = n_iter_per_second;
        }
        else if n_iter_per_second > second_maximum_iter_per_second {
            second_maximum_iter_per_second = n_iter_per_second;
        }
    }
    println!("{} performed {}-{} iterations per second.",
        descr, second_maximum_iter_per_second as i64,
        maximum_iter_per_second as i64);
}

fn is_sorted<T, Less: Fn(&T, &T) -> bool>(data: &[T], less: Less) -> bool {
    if data.len() == 0 { return true; }
    let mut prev = &data[0];
    for item in data.iter().skip(1) {
        if less(&item, &prev) { return false; }
        prev = item;
    }
    return true;
}

fn main() {
    let n1 = 200;
    let n2 = 4000;
    let k1 = 10;
    let k2 = 200;
    let k3 = 4000;
    
    let mut p11 = vec!();
    for _ in 0..n1 {
        p11.push(S1 { i1: rand::thread_rng().gen_range(0, k1), i2: 23})
    }
    let mut p12 = vec!();
    for _ in 0..n1 {
        p12.push(S1 { i1: rand::thread_rng().gen_range(0, k2), i2: 23})
    }
    let mut p13 = vec!();
    for _ in 0..n1 {
        p13.push(S1 { i1: rand::thread_rng().gen_range(0, k3), i2: 23})
    }
    let mut p21 = vec!();
    for _ in 0..n2 {
        p21.push(S1 { i1: rand::thread_rng().gen_range(0, k1), i2: 23})
    }
    let mut p22 = vec!();
    for _ in 0..n2 {
        p22.push(S1 { i1: rand::thread_rng().gen_range(0, k2), i2: 23})
    }
    let mut p23 = vec!();
    for _ in 0..n2 {
        p23.push(S1 { i1: rand::thread_rng().gen_range(0, k3), i2: 23})
    }
    
    let mut v11 = vec!();
    for _ in 0..n1 {
        v11.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k1) as f64,
            b: true,
        })
    }
    let mut v12 = vec!();
    for _ in 0..n1 {
        v12.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k2) as f64,
            b: true,
        })
    }
    let mut v13 = vec!();
    for _ in 0..n1 {
        v13.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k3) as f64,
            b: true,
        })
    }
    let mut v21 = vec!();
    for _ in 0..n2 {
        v21.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k1) as f64,
            b: true,
        })
    }
    let mut v22 = vec!();
    for _ in 0..n2 {
        v22.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k2) as f64,
            b: true,
        })
    }
    let mut v23 = vec!();
    for _ in 0..n2 {
        v23.push(S2 {
            s: String::from("abcdefg"),
            d: rand::thread_rng().gen_range(0, k3) as f64,
            b: true,
        })
    }

    // Correctness test

    let get_key1 = | s: &S1 | s.i1 as usize;
    let comp1 = | s1: &S1, s2: &S1 | get_key1(s1) < get_key1(s2);
    let comp1r = | s1: &S1, s2: &S1 | get_key1(s1).cmp(&get_key1(s2));

    let mut p = p11.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k1 as usize, &get_key1);
    assert!(! is_sorted(&p11, &comp1));
    assert!(is_sorted(&p, &comp1));
    
    p = p12.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k2 as usize, &get_key1);
    assert!(! is_sorted(&p12, &comp1));
    assert!(is_sorted(&p, &comp1));

    p = p13.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k3 as usize, &get_key1);
    assert!(! is_sorted(&p13, &comp1));
    assert!(is_sorted(&p, &comp1));

    p = p21.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k1 as usize, &get_key1);
    assert!(! is_sorted(&p21, &comp1));
    assert!(is_sorted(&p, &comp1));

    p = p22.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k2 as usize, &get_key1);
    assert!(! is_sorted(&p22, &comp1));
    assert!(is_sorted(&p, &comp1));

    p = p23.clone();
    assert!(! is_sorted(&p, &comp1));
    counting_sort(&mut p, k3 as usize, &get_key1);
    assert!(! is_sorted(&p23, &comp1));
    assert!(is_sorted(&p, &comp1));

    p = p23.clone();
    assert!(! is_sorted(&p, &comp1));
    p.sort_by(&comp1r);
    assert!(! is_sorted(&p23, &comp1));
    assert!(is_sorted(&p, &comp1));
    
    let get_key2 = | s: &S2 | s.d as usize;
    let comp2 = | s1: &S2, s2: &S2 | get_key2(s1) < get_key2(s2);
    let comp2r = | s1: &S2, s2: &S2 | get_key2(s1).cmp(&get_key2(s2));
    
    let mut v = v11.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k1 as usize, &get_key2);
    assert!(! is_sorted(&v11, &comp2));
    assert!(is_sorted(&v, &comp2));
    
    v = v12.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k2 as usize, &get_key2);
    assert!(! is_sorted(&v12, &comp2));
    assert!(is_sorted(&v, &comp2));
    
    v = v13.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k3 as usize, &get_key2);
    assert!(! is_sorted(&v13, &comp2));
    assert!(is_sorted(&v, &comp2));
    
    v = v21.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k1 as usize, &get_key2);
    assert!(! is_sorted(&v21, &comp2));
    assert!(is_sorted(&v, &comp2));
    
    v = v22.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k2 as usize, &get_key2);
    assert!(! is_sorted(&v22, &comp2));
    assert!(is_sorted(&v, &comp2));
    
    v = v23.clone();
    assert!(! is_sorted(&v, &comp2));
    counting_sort(&mut v, k3 as usize, &get_key2);
    assert!(! is_sorted(&v23, &comp2));
    assert!(is_sorted(&v, &comp2));

    v = v23.clone();
    assert!(! is_sorted(&v, &comp2));
    v.sort_by(&comp2r);
    assert!(! is_sorted(&v23, &comp2));
    assert!(is_sorted(&v, &comp2));

    println!("OK");

    // Performance test
    bench(&format!("counting_sort compact {} {}", n1, k1), | | {
        counting_sort(&mut p11.clone(), k1 as usize, &get_key1);
    });
    bench(&format!("counting_sort compact {} {}", n1, k2), | | {
        counting_sort(&mut p12.clone(), k2 as usize, &get_key1);
    });
    bench(&format!("counting_sort compact {} {}", n1, k3), | | {
        counting_sort(&mut p13.clone(), k3 as usize, &get_key1);
    });
    bench(&format!("sort compact {} {}", n1, k3), | | {
        p13.clone().sort_by(&comp1r);
    });

    bench(&format!("counting_sort compact {} {}", n2, k1), | | {
        counting_sort(&mut p21.clone(), k1 as usize, &get_key1);
    });
    bench(&format!("counting_sort compact {} {}", n2, k2), | | {
        counting_sort(&mut p22.clone(), k2 as usize, &get_key1);
    });
    bench(&format!("counting_sort compact {} {}", n2, k3), | | {
        counting_sort(&mut p23.clone(), k3 as usize, &get_key1);
    });
    bench(&format!("sort compact {} {}", n2, k3), | | {
        p23.clone().sort_by(&comp1r);
    });

    
    bench(&format!("counting_sort large {} {}", n1, k1), | | {
        counting_sort(&mut v11.clone(), k1 as usize, &get_key2);
    });
    bench(&format!("counting_sort large {} {}", n1, k2), | | {
        counting_sort(&mut v12.clone(), k2 as usize, &get_key2);
    });
    bench(&format!("counting_sort large {} {}", n1, k3), | | {
        counting_sort(&mut v13.clone(), k3 as usize, &get_key2);
    });
    bench(&format!("sort large {} {}", n1, k3), | | {
        v13.clone().sort_by(&comp2r);
    });

    bench(&format!("counting_sort large {} {}", n2, k1), | | {
        counting_sort(&mut v21.clone(), k1 as usize, &get_key2);
    });
    bench(&format!("counting_sort large {} {}", n2, k2), | | {
        counting_sort(&mut v22.clone(), k2 as usize, &get_key2);
    });
    bench(&format!("counting_sort large {} {}", n2, k3), | | {
        counting_sort(&mut v23.clone(), k3 as usize, &get_key2);
    });
    bench(&format!("sort large {} {}", n2, k3), | | {
        v23.clone().sort_by(&comp2r);
    });
}
