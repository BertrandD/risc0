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

use risc0_bigint2::ec::{AffinePoint, Secp256k1Curve};
use risc0_zkvm::guest::env;

fn main() {
    let (lhs, rhs): ([[u32; 8]; 2], [[u32; 8]; 2]) = env::read();
    let lhs = AffinePoint::new_unchecked(lhs[0], lhs[1]);
    let rhs = AffinePoint::new_unchecked(rhs[0], rhs[1]);

    let mut result = AffinePoint::<8, Secp256k1Curve>::new_unchecked([0u32; 8], [0u32; 8]);
    lhs.add(&rhs, &mut result);

    env::commit(&(result.as_u32s(), result.is_zero()));
}
