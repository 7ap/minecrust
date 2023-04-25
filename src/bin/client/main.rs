#[macro_use]
extern crate log;

use anyhow::Result;
use cfg_if::cfg_if;

#[async_std::main]
async fn main() -> Result<()> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init()?;
        } else {
            env_logger::init();
        }
    }

    info!("Hello, client!");

    Ok(())
}
