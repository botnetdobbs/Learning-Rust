/**
 * Primitive Types
  Integers: u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 (Number of bits they take in memory)
  Floats: f32 f64
  Boolean: (bool)
  Characters: (char)
  Tuples
  Arrays
*/

pub fn run() {
     // Find max size
     println!("Max i32: {}", std::i32::MAX);
     println!("Max i64: {}", std::i64::MAX);

     // Boolean
     let is_active = true;

     // Get boolean from an expression
     let is_greater = 10 > 9;

     let a1 = 'a';
     println!("{:?}", (is_active, is_greater, a1));
}