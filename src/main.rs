mod cli;
mod common;

use crate::cli::parse;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL_ALLOCATOR: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let result = parse(None);

    // If there was an error, print it
    if let Err(e) = result {
        e.print();
    }
}