use mimalloc::MiMalloc;

pub mod api;
pub mod error;
pub mod file_info;
pub mod provider;
pub mod shutdown;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
