// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// I AM NOT DONE

mod math {
  #[macro_export]
  macro_rules! multiply {
    ($a: expr, $b: expr) => {
      println!("{}", $a * $b);
    };
  }
  pub(crate) use multiply;
}

fn main() {
  math::multiply!(4, 3);
}
