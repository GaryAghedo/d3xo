use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D3xoError {
    #[error("An error occurred")]
    EmptyName,
}

pub trait D3Core {
    fn greet(&self, name: String) -> Result<String>;
}

pub async fn run(d3core: &dyn D3Core, name: &str) -> Result<()> {
    let greeting: String = d3core.greet("D3xo".to_string())?;
    println!("{}", greeting);
    Ok(())
}

pub async fn run_check_name(d3core: &dyn D3Core, name: &str) -> Result<String> {
    let trimmed_name = name.trim();

    if trimmed_name.is_empty() {
        return Err(D3xoError::EmptyName.into());
    }

    d3core.greet(trimmed_name.to_string())
}




