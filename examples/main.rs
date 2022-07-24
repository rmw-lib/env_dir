use anyhow::Result;
use env_dir::add;

fn main() -> Result<()> {
  dbg!(add(1, 2));
  Ok(())
}
