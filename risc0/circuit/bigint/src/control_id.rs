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

// Automatically generated by:
//   cargo run -p risc0-circuit-bigint -F make_control_ids --bin make_control_ids > risc0/circuit/bigint/src/control_id.rs

use risc0_zkp::{core::digest::Digest, digest};

pub const RSA_256_X1_CONTROL_ID: Digest =
    digest!("6451d473323b2161e53d6c63a75e761e22223c0b5da2750de2b7d169be50a056");
pub const RSA_256_X1_CONTROL_ROOT: Digest =
    digest!("40976e1e026381714726a1727cff59533fb2ad0c28c9106129e1924c6bc7a62e");

pub const RSA_256_X2_CONTROL_ID: Digest =
    digest!("b3894f55955d8c117c19f757f6d17921d77f5c54d53dc6038a3d15377b9f7524");
pub const RSA_256_X2_CONTROL_ROOT: Digest =
    digest!("f34d7d2cf2da97453d5f337682c81a00cf159552aae5c65be8f41e11fdb43504");

pub const RSA_3072_X1_CONTROL_ID: Digest =
    digest!("c09ea91b14fd07053137c8082473b060aaf5a13ba91299605d5f58105b56f947");
pub const RSA_3072_X1_CONTROL_ROOT: Digest =
    digest!("7f6b32295c39bb2638ffbb3e8ef9a40891bdfd349b642965ad5b3e0e38b63c5a");

pub const RSA_3072_X15_CONTROL_ID: Digest =
    digest!("d9142365cc8f841d61d6e46e6277120f9152f1213b49f26e7cd6d14fff39792b");
pub const RSA_3072_X15_CONTROL_ROOT: Digest =
    digest!("05d43273c21568119cdcf8074703cf05d035fb1c7d605c1d2f85a74944f96062");

pub const NONDET_INV_TEST_8_CONTROL_ID: Digest =
    digest!("9c6a472cd569c41d8fe6fe214fedea021ce51b5d85f10c5dac0c3e578f24390a");
pub const NONDET_INV_TEST_8_CONTROL_ROOT: Digest =
    digest!("d6c49b07e8cfff51f7f53e74a32c595e2a932e633490b3345d470a52948a0d03");

pub const CONST_ADD_TEST_8_CONTROL_ID: Digest =
    digest!("a3731076b6b04069e3b14a4441b6f61167b9154074878f0268a32c60fd8b5752");
pub const CONST_ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("38c2cb59e946ad359f2d39337da4b226f062d8211b0e7210e1bfe12b6f426201");

pub const CONST_ADD_ALT_TEST_16_CONTROL_ID: Digest =
    digest!("78313301fcac4e0e4e4e341d882730401ab743514b33670471496c11553ec835");
pub const CONST_ADD_ALT_TEST_16_CONTROL_ROOT: Digest =
    digest!("12e86020c92d9773ae24ac0cb0da6077e298d0341c752927dc9cf01aa4b34c6f");

pub const CONST_MUL_TEST_8_CONTROL_ID: Digest =
    digest!("942b6205d879826149043e26876a17392c84e36d87193f1735a9423d31b9ff53");
pub const CONST_MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("0c024e4d1fb05571576f3d5b974dc21658fe963250efcd01a326151d470a0748");

pub const ADD_TEST_8_CONTROL_ID: Digest =
    digest!("bb74140bb663752faba1330cbb09f86ffb17cb6795c5573a14ae8555f576734b");
pub const ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("3094d76091e3e66e5063a14c3a3bfb1f2f6d571a2912ec34aae0f4712d020965");

pub const ADD_TEST_16_CONTROL_ID: Digest =
    digest!("ee74170f8024032d8f8d9025410a194b5fa68d6fd33b773a3a96ab36f771214e");
pub const ADD_TEST_16_CONTROL_ROOT: Digest =
    digest!("6fe2e90830f48a1cf47f3a583c04ae51f6122827625d5433ff30c869f74c9703");

pub const ADD_TEST_128_CONTROL_ID: Digest =
    digest!("13af4b260313602ad65e0824caeae323d106d47609d95e05cbe9413bc64be05e");
pub const ADD_TEST_128_CONTROL_ROOT: Digest =
    digest!("e55d7f4ce8d3ba5fc490b342acdae105fd94d7521b047a159c0d2f7368d05d74");

pub const CONST_ONE_TEST_8_CONTROL_ID: Digest =
    digest!("1f8fc4087a0e1d4c2348c16cc0ce5e721776ad1803b3eb1754851a2e4e81f002");
pub const CONST_ONE_TEST_8_CONTROL_ROOT: Digest =
    digest!("a067705e9fc51c2b5128843c7f2c9a10691dbe271d40fd3f3d56cf00930e7775");

pub const CONST_TWOBYTE_TEST_16_CONTROL_ID: Digest =
    digest!("ee9b1730160a75140d0d470ab20f353eff375b346647012ec8733d41e821a733");
pub const CONST_TWOBYTE_TEST_16_CONTROL_ROOT: Digest =
    digest!("9a258702faa4b75a2e7fb96a254a0e14b65619562d0b5a0aa4c80917b6cdb62b");

pub const SUB_TEST_8_CONTROL_ID: Digest =
    digest!("2184de567a601e22fedb515d4fade66447bbe21ab9f9d472ddb81e27d40d4f0c");
pub const SUB_TEST_8_CONTROL_ROOT: Digest =
    digest!("48bd120c96894b5cf3f3df1d9f959d64e0d08430d76812762fdfbc1b84f81e02");

pub const SUB_TEST_128_CONTROL_ID: Digest =
    digest!("6cb6ac257d77fa57f67f9b5a48dad13d0117f625243e3a732fbebc1eefd81a5d");
pub const SUB_TEST_128_CONTROL_ROOT: Digest =
    digest!("2948073fe910950d41b27f0079dd5648ffbbff658b02780f4fc12e70b3caa345");

pub const MUL_TEST_8_CONTROL_ID: Digest =
    digest!("b84be75d8dc2e6270149bd07cc3fad38d8f84226400cc85dc7b3ab72b1aa6207");
pub const MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("08be6355563b4b47ade35c1391e89a450e5bf3264f41f03abf532e746864274b");

pub const MUL_TEST_128_CONTROL_ID: Digest =
    digest!("80b4823af8eaaa1c1a844f551a4ff055e49bfc589ed8d701069b64637694f770");
pub const MUL_TEST_128_CONTROL_ROOT: Digest =
    digest!("a3c5f2432fbcf66e70d9c1770ae8bd124c435b471c124e3f249b054ed14a0249");

pub const REDUCE_TEST_8_CONTROL_ID: Digest =
    digest!("296ec14e22e1af360c0ba2143a0a1d09cf6a7d564ba7c64ee3463367b52e0730");
pub const REDUCE_TEST_8_CONTROL_ROOT: Digest =
    digest!("aa00627176871a4820aec41bc4afa0549102cc3431700c46d6e6d566eaf62a2d");

pub const REDUCE_TEST_128_CONTROL_ID: Digest =
    digest!("76419c42eb6b0227ce4655309192e20e7a39575d0e87a64d0d1b1b40c2ad152f");
pub const REDUCE_TEST_128_CONTROL_ROOT: Digest =
    digest!("eedb5f396187151406919c42f76c5806079f8329e13590346dae78097fef8b34");

pub const EC_ADD_RZ8TEST_CONTROL_ID: Digest =
    digest!("3a135657b5ff081a44ba9e21caf69248a6af7434ffd5f4298d8bc5516fe03c15");
pub const EC_ADD_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("45187141b18d0841732ba01e5ad0c6429a9bdd1a1b35d1423cfe1a07b1aa3059");

pub const EC_ADD_SECP256K1_CONTROL_ID: Digest =
    digest!("92375b4770ede3768aaa1b5cde44826a88fb86284edddd773328ae5d076c656e");
pub const EC_ADD_SECP256K1_CONTROL_ROOT: Digest =
    digest!("c2ab934f7a455b4c89ff3849f721dc71375b95196e05e80c0e225e385ddcef20");

pub const EC_DOUB_RZ8TEST_CONTROL_ID: Digest =
    digest!("2d3a4a6d8504f741adb1a93e9e82b63668abb238ac2b441d09469173fe4f4367");
pub const EC_DOUB_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("083a3729fd00dc0746245622290f284ad1455b3e4f242f2a87066e5bcf793245");

pub const EC_DOUB_SECP256K1_CONTROL_ID: Digest =
    digest!("e40f564032558252767ec669360ee834036a7b2c06e01c756b46063964c03868");
pub const EC_DOUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("9ed4a2573667d40522e63b473243f5551bceb14c308ca21c6921675bfd925625");

pub const EC_MUL_RZ8TEST_CONTROL_ID: Digest =
    digest!("47f12172eae1cc640b0b07362b774e64ae80da32a78316376506a47408ac5157");
pub const EC_MUL_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("5ec55407a8e1375cf313a624cc81875584f64b5925134008f8bc954183315a41");

pub const EC_MUL_SECP256K1_CONTROL_ID: Digest =
    digest!("5f9f26142a9783521a31720b49f3fc6b3f7a672df1e1d522009a540a01e3c73c");
pub const EC_MUL_SECP256K1_CONTROL_ROOT: Digest =
    digest!("f43e155b06f490004ba60f3405c1a11416dbf63f6e531f37104d24751929134b");

pub const EC_NEG_RZ8TEST_CONTROL_ID: Digest =
    digest!("7744c431a2e18a582d336f02b76c9730b253d74ad40ea64df14cea0c03417b07");
pub const EC_NEG_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("5787af73033c961a33fb76257e4da0233d497521dc268e48217a6931c6d4b725");

pub const EC_NEG_SECP256K1_CONTROL_ID: Digest =
    digest!("cfc08573af749117b4b5cf037025b52a1c3009592bea150b67cb813a8e3b7c31");
pub const EC_NEG_SECP256K1_CONTROL_ROOT: Digest =
    digest!("6361480d124a47455e7e980bed4917000d3b950662c73c75e7327273f425553f");

pub const EC_SUB_RZ8TEST_CONTROL_ID: Digest =
    digest!("336e421c1a7779043103295c6b6ffb6dccce516603ebe56f29baca44c466124b");
pub const EC_SUB_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("530a0b0afc3474380926ad054684bb1a20938e2944b99024af47782432f0a66f");

pub const EC_SUB_SECP256K1_CONTROL_ID: Digest =
    digest!("c3ff8207e53b3f153d0aea27b72cb00a086206395c481a21bf54221cda957a0a");
pub const EC_SUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("6f1196294afd6a2cf913c7180ca3ef74718f1b4d2e64974401722c2a4b3f6874");

pub const EC_PTS_EQ_RZ8TEST_CONTROL_ID: Digest =
    digest!("23716c007eb3245bf1ca6244de8f4d090c99500cf8d2f42e9e96d763e1eff31c");
pub const EC_PTS_EQ_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("765de037db9ec4171e4d5c76e3935f0a3de2f575a7ce645c57c354212825c365");

pub const EC_PTS_EQ_SECP256K1_CONTROL_ID: Digest =
    digest!("b6f0cd6a1972504a7de3180fd9fc6b0c50f4656ae7a84a567744fd64d024dd56");
pub const EC_PTS_EQ_SECP256K1_CONTROL_ROOT: Digest =
    digest!("7b923f30d5298427e2ded313b50f1843ae73595115339b104a6fde13d42f505f");

pub const EC_ON_CURVE_RZ8TEST_CONTROL_ID: Digest =
    digest!("42885c417fd97b48fed24e1aa9fd884e54719223def1173b98f6f2209df32662");
pub const EC_ON_CURVE_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("a116bb2de66175300164873598d32c0dc8b3da3c67df7c59b57ed50aba2a511c");

pub const EC_ON_CURVE_SECP256K1_CONTROL_ID: Digest =
    digest!("6a7ac02831f47c6c466047428bfe69442c5ae674814b345e60c5a541ce519244");
pub const EC_ON_CURVE_SECP256K1_CONTROL_ROOT: Digest =
    digest!("0eab0932d781d23398b5190d8b8a5c486dce515410278d70b273be4a26899c1e");

pub const EC_ADD_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("3ed018055dd41e2334ca78131a895503c00d2c2bc997ac6a6fefb80b6452563f");
pub const EC_ADD_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("4692bd0aff5c0e27ae7be923e62a8f1f90542c0337a0f12ba7ffc809aa55d72e");

pub const EC_DOUB_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("23c4cb5dcc447952818eb4550568d64e1ce5e81c8613211c4f187e1558f6ef68");
pub const EC_DOUB_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("24083c3f0092aa102cf8020332c48f73cfcccc463e6cb5773321064c29b24456");

pub const EC_MUL_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("200be519d598f10440735a675b409d583d45b6064557a737d4fc6d602c594d3d");
pub const EC_MUL_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("8973876928f398134917f613fdbac60bdfd5f54360d5732128e3aa585274eb63");

pub const EC_NEG_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("bb7ce806e439083adb3beb44929cfa3f41254d4e534e917392868146fadf2614");
pub const EC_NEG_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("25148b23e4ad9a0c9eb9273b1ea9c773d7a6e3646a15e62e07d31f4700cd7806");

pub const EC_SUB_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("22c7050d25669d75d8fe19240a7f325ebc530d2339ede81bac92923e8ca72f48");
pub const EC_SUB_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("999a4e307b361830f0c5fd3e81071a3fea1aa35a640ed63806a845675b1c606b");
