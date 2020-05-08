/*
 * Filename   : main.rs
 * Author     : Cole Vohs
 * Course     : CS476
 * Assignment : Project
 * Description: A program to run different versions of quicksort
 *   NOTE: rust should be configured with unstable features enabled
 *   I had to reinstall rust using rustup
 *   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 * Git        : https://github.com/ColeW-Picaro/rustsort
 */

#![feature(is_sorted)]
#![feature(slice_partition_at_index)]

extern crate rand;
extern crate rayon;
#[macro_use] extern crate text_io;

use std::time::SystemTime;
use std::io;

use rand::Rng;

/* 
 * A stock partition function
 * Partitions v such that all elements 
 * the pivot element is in its final 
 * sorted position. 
 * Returns the index of the pivot
 */
fn partition (v: &mut [isize]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
	if v[j] < v[pivot] {
	    v.swap(i, j);
	    i += 1;
	}
    }
    v.swap(i, pivot);
    return i as usize;    
}

/*
 * A 3-way partition on v
 */
fn partition3(v: &mut [isize]) -> (usize, usize) {
    let pivot = v.len() - 1;
    let mut lo = 0;
    let mut hi = pivot;
    let mut eq = lo;
    while eq != hi {
	if v[eq] < v[pivot] {
	    v.swap(lo, eq);
	    lo += 1;
	    eq += 1;
	} else if v[eq] > v[pivot] {
	    hi -= 1;
	    v.swap(eq, hi);	    
	} else {
	    eq += 1;
	}
    }
    v.swap(eq, pivot);
    return (lo, hi);
}

/* 
 * A parallel version of quicksort using rayon::join
 * v: the elements to be sorted
 * cutoff: the number of elements to switch to slice::sort
 * depth: the maximum depth 
 * level: the current depth
 */
fn quicksort_parallel(v: &mut [isize], cutoff: usize, depth: usize, level: usize) {
    if v.len() <= 1 {
	return;
    } else if v.len() <= cutoff {
	v.sort();
	return;
    } else if depth == level {
	quicksort(v, cutoff);
	return;
    } else {
	// Partition
	let pivot = partition(v);
	let (left, right) = v.split_at_mut(pivot);
	// Spawn new threads 
	rayon::join(
	    || quicksort_parallel(left, cutoff, depth, level + 1),
	    || quicksort_parallel(&mut right[1..], cutoff, depth, level + 1)
	);
    }
}


/* 
 * A parallel version of quicksort using rayon::join
 * v: the elements to be sorted
 * cutoff: the number of elements to switch to slice::sort
 * depth: the maximum depth 
 * level: the current depth
 */
fn quicksort3_parallel(v: &mut [isize], cutoff: usize, depth: usize, level: usize) {
    if v.len() <= 1 {
	return;
    } else if v.len() <= cutoff {
	v.sort();
	return;
    } else if depth == level {
	quicksort(v, cutoff);
	return;
    } else {
	// Partition
	let (lo, hi) = partition3(v);
	let (left, right) = v.split_at_mut(lo);
	// Spawn new threads 
	rayon::join(
	    || quicksort3_parallel(left, cutoff, depth, level + 1),
	    || quicksort3_parallel(&mut right[(hi - lo)..], cutoff, depth, level + 1)
	);
    }
}

/* 
 * Quicksort using a handwritten partition
 * v: the slice to be sorted
 * cutoff: the number of elemetns to switch to slice::sort
 */
fn quicksort(v: &mut [isize], cutoff: usize) {
    if v.len() <= 1 {
	return;	
    } else if v.len() <= cutoff {
	// Vec::sort does an insertion sort at N == 100
	v.sort();
	return;
    } else {
	// Partition
	let pivot = partition(v);
	let (left, right) = v.split_at_mut(pivot);
	// Recurse
	quicksort(left, cutoff);
	quicksort(&mut right[1..], cutoff);
    }
}

/* 
 * Quicksort using a handwritten partition
 * v: the slice to be sorted
 * cutoff: the number of elemetns to switch to slice::sort
 */
fn quicksort3(v: &mut [isize], cutoff: usize) {
    if v.len() <= 1 {
	return;	
    } else if v.len() <= cutoff {
	// Vec::sort does an insertion sort at N == 100
	v.sort();
	return;
    } else {
	// Partition
	let (lo, hi) = partition3(v);
	let (left, right) = v.split_at_mut(lo);
	// Recurse
	quicksort3(left, cutoff);
	quicksort3(&mut right[(hi - lo)..], cutoff);
    }
}

fn main() {
    // N, depth, and cutoff
    print!("N      ==> ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let n: usize = read!();
    print!("depth  ==> ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let depth: usize = read!();
    print!("cutoff ==> ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let cutoff: usize = read!();
    print!("range  ==> ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let range: isize = read!();
    
    // Create the vector with random numbers [0, 100000)
    let mut eng = rand::thread_rng();
    let mut v = vec![];
    for _i in 0..n {
	v.push(eng.gen_range(0, range));	
    }
    let mut v_serial = v.to_vec();
    let mut v_parallel = v.to_vec();
    let mut v_serial3 = v.to_vec();
    let mut v_parallel3 = v.to_vec();
    
    // run std::sort
    let std_sort_start = SystemTime::now();
    v.sort();
    println!("slice::sort           : {:?}", std_sort_start.elapsed());
    assert!(v.is_sorted());

    // run handwritten partition quicksort
    let serial_sort_start = SystemTime::now();
    quicksort(&mut v_serial[..], cutoff);
    println!("serial quicksort      : {:?}", serial_sort_start.elapsed());
    assert!(v_serial.is_sorted());

    // run partition3 quicksort
    let quicksort3_start = SystemTime::now();
    quicksort3(&mut v_serial3[..], cutoff);
    println!("partition3 quicksort  : {:?}", quicksort3_start.elapsed());
    assert!(v_serial3.is_sorted());

    // run parallel quicksort
    let parallel_sort_start = SystemTime::now();
    quicksort_parallel(&mut v_parallel[..], cutoff, depth, 0);
    println!("parallel quicksort    : {:?}", parallel_sort_start.elapsed());
    assert!(v_parallel.is_sorted());

    // run parallel3 quicksort
    let parallel3_sort_start = SystemTime::now();
    quicksort3_parallel(&mut v_parallel3[..], cutoff, depth, 0);
    println!("parallel quicksort3   : {:?}", parallel3_sort_start.elapsed());
    assert!(v_parallel3.is_sorted());    
}
