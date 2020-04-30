# rustsort
Quicksort implementations and comparisons in rust

### Building and Running
Ensure unstable features are enabled. You might need to reinstall rust with rustup.
If you don't have rust installed or need to reinstall, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Cargo:
build and run: `cargo run -- -C link-args=-Wl,-zstack-size=838860` to build and run.

just build: `cargo rustc -- -C link-args=-Wl,-zstack-size=838860`.

### Sort discussions
#### `slice::partition_at_index` quicksort
This sort was my first attempt. It uses the `slice::partition_at_index` function to partition. I was curious to try this out because I was wondering if I would get similar behavior as C++'s `std::partition`.  Turns out this is sort is slow because it doesn't actually return the index of the pivot value, but rather the left slice, the pivot value itself, and the right slice. You need to figure out the pivot index by calling `len()` on the slice, which is annoying.  This version also passes the whole slice though all the recursive calls instead of just the portion needed.

#### handwritten `partition` quicksort
When I realized I probably couldn't use rust's `std::thread` to parallelize this, I wrote a stock partition function to explore parallelizing it with Rayon using parallel iterators. Turns out the library doesn't let you do that because any closure you run with the for_each() has to be an `Fn` closure, and we would need a `FnMut` closure. So I just use a nonparallel partition routine for this quicksort.

#### parallel quicksort
I originally tried to implement this using `std::thread` by spawning a thread to run on the right half of the recursive calls, but rust's ownership rules wouldn't allow it because I was passing around the whole values vector instead of just the slices needed. So I used `rayon::join` to create two threads: One that passed only the left slice, and one that passed only the right slice.  This allowed me to treat once slice as two slices, which is a pretty neat trick.

#### problems
N can't be very large. I don't know why, but when I increase N to something at or above the 100000 range, it just overflows. It doesn't report a stack overflow though, it just freezes everything until the OS tells it to stop.  I tried manually increasing the stack size (hence the compiler flags), but it didn't help that much.

