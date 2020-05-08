# rustsort
Quicksort implementations and comparisons in rust

### Building and Running
Ensure unstable features are enabled. You might need to reinstall rust with rustup.
If you don't have rust installed or need to reinstall, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
I'm not an expert in rustup, but reinstalling to nightly worked for me

#### Cargo:
build and run with `cargo run`

### Sort discussions

#### serial quicksort
When I realized I probably couldn't use rust's `std::thread` or `partition_at_index` to parallelize this, I wrote a stock partition function to explore parallelizing it with Rayon using parallel iterators. Turns out the library doesn't let you do that because any closure you run with the for_each() has to be an `Fn` closure, and we would need a `FnMut` closure. So I just use a nonparallel partition routine for this quicksort.

#### parallel quicksort
I originally tried to implement this using `std::thread` by spawning a thread to run on the right half of the recursive calls, but rust's ownership rules wouldn't allow it because I was passing around the whole values vector instead of just the slices needed. So I used `rayon::join` to create two threads: One that passed only the left slice, and one that passed only the right slice.  This allowed me to treat once slice as two slices while still modifying the original slice, which is a pretty neat trick.

#### partition3 sorts
These sorts use a 3-way partition scheme. It keeps track of elements equal to the pivot and returns the index range of elements equal to the pivot.  This way only the data that needs to be passed recursively is passed while the sorted equal elements are not. Play around with N and the Range inputs to find a configuration that makes the 3-way partition fast.