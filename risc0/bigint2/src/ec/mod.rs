// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests;

use include_bytes_aligned::include_bytes_aligned;

use crate::ffi::{sys_bigint2_3, sys_bigint2_4};

const ADD_BLOB: &[u8] = include_bytes_aligned!(4, "ec_add_256.blob");
const DOUBLE_BLOB: &[u8] = include_bytes_aligned!(4, "ec_double_256.blob");

pub const EC_256_WIDTH_WORDS: usize = 256 / 32;

mod curve;

pub use curve::{Curve, Secp256k1Curve, WeierstrassCurve};

/// An affine point on an elliptic curve.
#[derive(Debug, Eq, PartialEq)]
pub struct AffinePoint<const WIDTH: usize, C> {
    buffer: [[u32; WIDTH]; 2],
    is_zero: bool,
    _marker: std::marker::PhantomData<C>,
}

// Manual clone and copy implementations to not require C to be Copy/Clone
impl<const WIDTH: usize, C> Clone for AffinePoint<WIDTH, C> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const WIDTH: usize, C> Copy for AffinePoint<WIDTH, C> {}

impl<const WIDTH: usize, C> AffinePoint<WIDTH, C> {
    pub const IDENTITY: AffinePoint<WIDTH, C> = AffinePoint {
        buffer: [[0u32; WIDTH]; 2],
        is_zero: true,
        _marker: std::marker::PhantomData,
    };

    /// Constructs an affine point from x and y coordinates, without checking that it is on
    /// a specific curve.
    pub fn new_unchecked(x: [u32; WIDTH], y: [u32; WIDTH]) -> AffinePoint<WIDTH, C> {
        AffinePoint {
            buffer: [x, y],
            is_zero: false,
            _marker: std::marker::PhantomData,
        }
    }
    /// The point as concatenated u32s for x and y
    ///
    /// Little-endian, x coordinate before y coordinate
    /// TODO: Where to doc this next bit
    /// The result is returned as a [[u32; WIDTH]; 2], and the FFI with the guest expects a [u32; WIDTH * 2]. Per https://doc.rust-lang.org/reference/type-layout.html#array-layout they will be laid out the same in memory and this is acceptable.
    pub fn as_u32s(&self) -> &[[u32; WIDTH]; 2] {
        &self.buffer
    }

    /// Returns true if the point is the identity element (point at zero/infinity).
    pub fn is_zero(&self) -> bool {
        self.is_zero
    }
}

impl<const WIDTH: usize, C: Curve<WIDTH>> AffinePoint<WIDTH, C> {
    /// Elliptic curve multiplication of the point by a scalar.
    pub fn mul(&self, scalar: &[u32; WIDTH], result: &mut AffinePoint<WIDTH, C>) {
        // This assumes `pt` is actually on the curve
        // This assumption isn't checked here, so other code must ensure it's met
        // This algorithm doesn't work if `scalar` is a multiple of `pt`'s order

        // Initialize two values to alternate writes to avoid unnecessary copies.
        let mut result_flip = false;
        let mut result1 = AffinePoint::IDENTITY;
        let mut result2 = AffinePoint::IDENTITY;

        // Note: the first value can be an uninitialized value.
        let mut doubled_pt1 = *self;
        let mut doubled_pt2 = *self;

        let mut first_write = true;
        for pos in 0..bits(scalar) {
            // Alternate between the doubled value. Immutable reference is to the current value,
            // mutable reference is to the other that can be written to.
            // Note: This is not using a boolean flag because `pos%2` is less cycles.
            let (current_doubled, next_doubled) = if pos % 2 == 0 {
                (&doubled_pt2, &mut doubled_pt1)
            } else {
                (&doubled_pt1, &mut doubled_pt2)
            };

            if bit(scalar, pos) {
                // Alternate buffers to write to and use as current value.
                let (current_result, next_result) = if result_flip {
                    (&result2, &mut result1)
                } else {
                    (&result1, &mut result2)
                };

                if first_write {
                    first_write = false;
                    *next_result = *current_doubled;
                } else {
                    current_result.add(current_doubled, next_result);
                }
                result_flip = !result_flip;
            }

            current_doubled.double(next_doubled);
        }

        // Assert that some value was written to the result.
        if first_write {
            panic!("Multiplication by zero forbidden as affine coordinates can't represent the point at infinity");
        }

        // Return the result, based on which buffer was written to last.
        *result = if result_flip { result2 } else { result1 };
    }

    /// Elliptic curve doubling of the affine point.
    #[stability::unstable]
    pub fn double(&self, result: &mut Self) {
        let curve = C::CURVE;
        // If the point is zero, can short-circuit and return the identity point.
        if self.is_zero {
            *result = *self;
        } else {
            if self.buffer[1] != [0u32; WIDTH] {
                unsafe {
                    double_raw(self.as_u32s(), curve.as_u32s(), &mut result.buffer);
                }
                // DO NOT REMOVE: the result is unchecked, and only the buffer is updated above
                result.is_zero = false;
            } else {
                // DO NOT REMOVE: in this case a zero has been computed and the buffer is ignored
                result.is_zero = true;
            }
        }
    }

    /// Elliptic curve addition of the affine point.
    #[stability::unstable]
    pub fn add(&self, rhs: &AffinePoint<WIDTH, C>, result: &mut AffinePoint<WIDTH, C>) {
        let curve = C::CURVE;

        // If the left or right value is zero, can return the other value.
        if self.is_zero {
            *result = *rhs;
        } else if rhs.is_zero {
            *result = *self;
        } else if self.buffer[0] == rhs.buffer[0] {
            // X coordinates are the same, so either we double the value if it's the same point,
            // or return the identity if it's different (not on the curve).
            if self.buffer[1] != rhs.buffer[1] {
                // x == x, y == -y, so the result is the identity point
                result.is_zero = true;
            } else {
                // P + P, which can be done with a double call.
                unsafe {
                    double_raw(self.as_u32s(), curve.as_u32s(), &mut result.buffer);
                }
                // DO NOT REMOVE: the result is unchecked, and only the buffer is updated above
                result.is_zero = false;
            }
        } else {
            // X coordinates are different, so we can add the points as normal.
            unsafe {
                add_raw(
                    self.as_u32s(),
                    rhs.as_u32s(),
                    curve.as_u32s(),
                    &mut result.buffer,
                );
            }
            // DO NOT REMOVE: the result is unchecked, and only the buffer is updated above
            result.is_zero = false;
        }
    }
}

unsafe fn double_raw<const WIDTH: usize>(
    point: &[[u32; WIDTH]; 2],
    curve: &[[u32; WIDTH]; 3],
    result: &mut [[u32; WIDTH]; 2],
) {
    unsafe {
        // Because [[u32; WIDTH]; 2] and [u32; WIDTH * 2] are laid out the same way, this `as` is safe
        // (and similarly with [[u32; WIDTH]; 3] and [u32; WIDTH * 3])
        // See https://doc.rust-lang.org/reference/type-layout.html#array-layout
        sys_bigint2_3(
            DOUBLE_BLOB.as_ptr(),
            point.as_ptr() as *const u32,
            curve.as_ptr() as *const u32,
            result.as_mut_ptr() as *mut u32,
        );
    }
}

unsafe fn add_raw<const WIDTH: usize>(
    lhs: &[[u32; WIDTH]; 2],
    rhs: &[[u32; WIDTH]; 2],
    curve: &[[u32; WIDTH]; 3],
    result: &mut [[u32; WIDTH]; 2],
) {
    unsafe {
        // Because [[u32; WIDTH]; 2] and [u32; WIDTH * 2] are laid out the same way, this `as` is safe
        // (and similarly with [[u32; WIDTH]; 3] and [u32; WIDTH * 3])
        // See https://doc.rust-lang.org/reference/type-layout.html#array-layout
        sys_bigint2_4(
            ADD_BLOB.as_ptr(),
            lhs.as_ptr() as *const u32,
            rhs.as_ptr() as *const u32,
            curve.as_ptr() as *const u32,
            result.as_mut_ptr() as *mut u32,
        );
    }
}

/// Checks if the bit at the position is set.
fn bit<const WIDTH: usize>(scalar: &[u32; WIDTH], bit: u32) -> bool {
    let bits_per_digit = 32u32;
    if let Ok(digit_index) = usize::try_from(bit / bits_per_digit) {
        if let Some(digit) = scalar.get(digit_index) {
            let bit_mask = (1u32) << (bit % bits_per_digit);
            return (digit & bit_mask) != 0;
        }
    }
    false
}

/// Returns the minimum number of bits required to represent the scalar
fn bits<const WIDTH: usize>(scalar: &[u32; WIDTH]) -> u32 {
    // Find the highest set bit by scanning from most significant word
    for (i, &word) in scalar.iter().rev().enumerate() {
        if word != 0 {
            // Found highest non-zero word
            // Calculate total bits based on zero words and leading zeros of non-zero word
            return (WIDTH - 1 - i) as u32 * 32 + (32 - word.leading_zeros());
        }
    }
    0
}

#[cfg(test)]
mod bit_tests {
    use super::*;

    #[test]
    fn bit_test() {
        let test = [0b00001001, 0b11110010];
        assert_eq!(bits(&test), 40);
        assert!(bit(&test, 0));
        assert!(!bit(&test, 1));
        assert!(!bit(&test, 2));
        assert!(bit(&test, 3));
        assert!(bit(&test, 33));
        assert!(bit(&test, 36));
        assert!(!bit(&test, 35));
    }
}
