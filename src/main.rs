use selaginella::{Config, Selaginella};

fn main() -> anyhow::Result<()> {
    let config = Config::new("config.toml");
    Selaginella::build(config)?.resurrection()?;
    Ok(())
}
