#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

mod cost;
pub mod evolve;
pub mod layout;
mod simple_cost;
