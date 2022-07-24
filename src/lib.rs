use std::{env, path::PathBuf};

pub fn home(prefix: impl AsRef<str>) -> PathBuf {
  let prefix = prefix.as_ref();
  if let Ok(dir) = env::var(prefix.to_owned() + "_home") {
    dir.into()
  } else {
    let mut rmw = match dirs::home_dir() {
      Some(dir) => dir,
      None => {
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        dir
      }
    };
    rmw.push(".".to_owned() + prefix);
    rmw
  }
}
