use anyhow::Result;
use async_std::task;
use cfg_if::cfg_if;

fn main() -> Result<()> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init()?;
        } else {
            env_logger::init();
        }
    }

    task::block_on(minecrust::client::run());

    Ok(())
}
