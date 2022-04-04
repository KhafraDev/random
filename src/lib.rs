#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::Error;

#[napi]
pub fn get_random() -> Result<Vec<i32>, Error> {
  let mut buf = [0u8, 32];

  if let Err(err) = getrandom::getrandom(&mut buf) {
    return Err(Error::from_reason(err.to_string()));
  }

  return Ok(buf.iter().map(|&x| i32::from(x)).collect::<Vec<i32>>());
}