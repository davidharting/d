use rand::thread_rng;
use rand::seq::IteratorRandom;


pub struct RandConfig {
  pub length: usize
}

const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyz1234567890";

pub fn generate(config: &RandConfig) -> String {
  let mut rng = thread_rng();
  let mut result_buffer = String::with_capacity(config.length);

  for _ in 0..config.length {
    if let Some(letter) = ALPHABET.chars().choose(&mut rng) {
      result_buffer.push(letter);
    };
  };

  result_buffer
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generates_string_with_specified_length() {
    let cases = vec![1, 3, 5, 10, 25];
    for case in cases.iter() {
      let config = RandConfig { length: *case };
      let actual = generate(&config);
      let actual_length = actual.chars().count();
      assert_eq!(&actual_length, case, "Expected \"{}\" to have length of {}, but had length of {}.", actual, case, actual_length);
    }
  }
}
