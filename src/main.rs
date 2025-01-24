use std::sync::LazyLock;
use tokio::runtime::Runtime;

mod router;
mod config;
mod features;
mod model;

pub static RUNTIME:LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::new().unwrap()
});

fn main() {
    RUNTIME.block_on(features::init());
}
