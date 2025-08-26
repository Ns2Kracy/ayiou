use mimalloc::MiMalloc;

pub mod api;
pub mod cli;
pub mod commands;
pub mod error;
pub mod file_info;
pub mod provider;
pub mod server;
pub mod shutdown;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
