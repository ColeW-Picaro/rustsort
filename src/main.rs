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

use std::time::SystemTime;

use rand::Rng;

/* 
 * A stock partition function
 * Partitions v such that all elements 
 * the pivot element is in its final 
 * sorted position. 
 * Returns the index of the pivot
 */
fn partition (v: &mut [isize]) -> usize {
    let pivot = v.len () - 1;
    let mut i = 0;
    for j in 0..pivot {
	if v[j] < v[pivot] {
	    v.swap (i, j);
	    i += 1;
	}
    }
    v.swap (i, pivot);
    return i as usize;    
}


/* 
 * A parallel version of quicksort using rayon::join
 * v: the elements to be sorted
 * cutoff: the number of elements to switch to slice::sort
 * depth: the maximum depth 
 * level: the current depth
 */
fn quicksort_parallel (v: &mut [isize], cutoff: usize, depth: usize, level: usize) {
    if v.len() <= 1 {
	return;
    } else if v.len() <= cutoff {
	v.sort();
	return;
    } else if depth == level {
	v.sort();
	return;
    } else {
	// Partition
	let pivot = partition (v);
	let (left, right) = v.split_at_mut (pivot);
	// Spawn new threads 
	rayon::join (
	    || quicksort_parallel (left, cutoff, depth, level + 1),
	    || quicksort_parallel (&mut right[1..], cutoff, depth, level + 1)
	);
    }
}

/* 
 * Quicksort using a handwritten partition
 * v: the slice to be sorted
 * cutoff: the number of elemetns to switch to slice::sort
 */
fn quicksort (v: &mut [isize], cutoff: usize) {
    if v.len () <= 1 {
	return;	
    } else if v.len () <= cutoff {
	// Vec::sort does an insertion sort at N == 100
	v.sort();
	return;
    } else {
	// Partition
	let pivot = partition (v);
	let (left, right) = v.split_at_mut(pivot);
	// Recurse
	quicksort (left, cutoff);
	quicksort (&mut right[1..], cutoff);
    }
}

// Quicksort using Vec::partition_at_index on a vec slice
// Slower because the return value needs to be turned into a vec to get
// the position of the pivot
// I was ambitious to use partition_at_index in hopes it would behave as std::partition
// But it's more similar to std::nth_element
fn quicksort_pai (v: &mut [isize], low: usize, high: usize) {
    if high <= low {
	return;	
    } else if high - low <= 100 {
	// Vec::sort does an insertion sort at N == 100
	v.sort();
	return;
    } else {
	// Partition
	 let (left, _pivot, _right) = v[low..high].partition_at_index(high - 1);
	// Find index of pivot
	let left_vec = left.to_vec();
	let p = left_vec.len();
	// Recurse
	quicksort_pai (v, low, p - 1);
	quicksort_pai (v, p + 1, high); 
    }
}

fn main() {
    // N, depth, and cutoff
    let n = 50000;
    let depth = 4;
    let cutoff = 100;

    // Create the vector with random numbers [0, 1000)
    let mut eng = rand::thread_rng();
    let mut v = vec![];
    for _i in 0..n {
	v.push (eng.gen_range (0, 1000));
    }
    let mut v_serial = v.to_vec ();
    let mut v_pai = v.to_vec ();
    let mut v_parallel = v.to_vec();

    // run std::sort
    let std_sort_start = SystemTime::now();
    v.sort();
    println!("slice::sort : {:?}", std_sort_start.elapsed());
    assert!(v.is_sorted());

    // run partition_at_index sort 
    let pai_sort_start = SystemTime::now();
    quicksort_pai (&mut v_pai, 0, n - 1);
    println!("slice::partition_at_index partition quicksort : {:?}",
	     pai_sort_start.elapsed());
    assert!(v_pai.is_sorted());

    // run handwritten partition quicksort
    let serial_sort_start = SystemTime::now();
    quicksort (&mut v_serial, cutoff);
    println!("handwritten partition quicksort : {:?}", serial_sort_start.elapsed());
    assert!(v_serial.is_sorted());

    // run parallel quicksort
    let parallel_sort_start = SystemTime::now();
    quicksort_parallel (&mut v_parallel, cutoff, depth, 0);
    println!("parallel quicksort : {:?}", parallel_sort_start.elapsed());
    assert!(v_parallel.is_sorted());
    
}
