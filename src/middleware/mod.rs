use tera::{from_value, to_value, Error, Result, Value};
use std::collections::HashMap;
use std::fmt::format;

// Asset handler
pub fn assets(args: &HashMap<String, Value>) -> Result<Value> {
  let filetype = match args.get("filetype") {
    Some(val) => match from_value::<String>(val.clone()) {
      Ok(v) => v,
      Err(_) => {
        return Err(Error::msg(format!(
          "Function `assets` received filetype={} but `filetype` can only be a string",
          val
        )));
      }
    },
    None => {
      return Err(Error::msg(
        "Function `assets` didn't receive a `filetype` argument",
      ))
    }
  };

  let mut res = vec![];
  let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());

  if env == "development" {
    if filetype == "js" {
      res.push("//localhost:8080/app.js".to_string())
    }
  } else {
    let files = std::fs::read_dir("./dist").unwrap();
    for file in files {
      if let Ok(f) = file {
        if let Ok(fname) = f.file_name().into_string() {
          if fname.ends_with(&filetype) {
            let url = format!("/assets/{}", &fname);
            res.push(url.into());
          }
        }
      }
    }
  }

  return Ok(to_value(res).unwrap());
}
