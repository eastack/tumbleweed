use selaginella::{Config, Selaginella};

fn main() -> anyhow::Result<()> {
    let config = Config::new("config.toml");
    let selaginella = Selaginella::build(config)?;
    selaginella.resurrection();
    Ok(())
}
