# rust-countingsort
Rust implementation of countingsort sorting algorithm

This is an "unsafe" implementation of a very efficient algorithm to sort
a slice of items according their "key".

A small example is the following program:

	mod countingsort;
	
	#[derive(Debug)]
	struct S1 { a: i32, b: i32 }
	
	fn main() {
		let mut v = vec!();
		for i in 0..10 {
			v.push(S1 { a: ((i + 100) * i) % 13, b: i })
		}
		println!("{:?}", v);
		countingsort::counting_sort(&mut v, 13,
			&| s: &S1 | s.a as usize);
		println!("{:?}", v);
	}

If such program is compiled together with the `countingsort.rs` source file,
when it is run it should print: 

	[S1 { a: 0, b: 0 }, S1 { a: 10, b: 1 }, S1 { a: 9, b: 2 },
	S1 { a: 10, b: 3 }, S1 { a: 0, b: 4 }, S1 { a: 5, b: 5 },
	S1 { a: 12, b: 6 }, S1 { a: 8, b: 7 }, S1 { a: 6, b: 8 }, S1 { a: 6, b: 9 }]
	[S1 { a: 0, b: 0 }, S1 { a: 0, b: 4 }, S1 { a: 5, b: 5 },
	S1 { a: 6, b: 8 }, S1 { a: 6, b: 9 }, S1 { a: 8, b: 7 },
	S1 { a: 9, b: 2 }, S1 { a: 10, b: 1 }, S1 { a: 10, b: 3 }, S1 { a: 12, b: 6 }]

As it appears, the structs have been sorted by the `a` key.

The function takes three arguments.
The first is the mutable slice to sort, that is changed by the routine.
The second is the size of the zero-based range of the keys.
The third is a reference to a closure that returns the key of the given item.
In the above example, the second argument is 13, meaning that
the third argument must returns integer numbers between 0 and 12.

This algorithm has a time complexity of *O(n * s + k)*,
where ''n'' is the number of items to sort, ''s'' is the size of each item,
and ''k'' is the size of the range of the keys.

In practice, this algorithm is better than standard `sort` if the slice
has at least one hundred items, and the range of the keys is less than
ten times the items to sort.
For example, to sort a slice of 100,000 items,
it is convenient to use countingsort only
if the possible keys are less than a million.

The algorithm assumes that the key range is already known.
If it is not, an additional pass could compute it,
but if a reasonably tight range can be estimated,
it is better to use it instead of scanning the data.
