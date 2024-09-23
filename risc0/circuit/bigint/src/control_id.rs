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
    digest!("b64d9520da37fe3fb06b5254065d140238bd165a7b41bf51ef3fc25433e98503");
pub const RSA_256_X1_CONTROL_ROOT: Digest =
    digest!("39b42c22d723a56dda3ab641aecbf92442a7ff4415fe800b814a3d086a159e2b");

pub const RSA_256_X2_CONTROL_ID: Digest =
    digest!("c9b17f4d82070833e6d6023304b4111ef1e48767f204d66e705eff2074c28655");
pub const RSA_256_X2_CONTROL_ROOT: Digest =
    digest!("32d34e64a3d70b09ed889d63d9df1c4ffa468b1a150fc31246bc994c73e43b14");

pub const RSA_3072_X15_CONTROL_ID: Digest =
    digest!("5b8c6338762a361b58cdf418544cb321a5b11961e6907e05759a30130f892801");
pub const RSA_3072_X15_CONTROL_ROOT: Digest =
    digest!("a5568767aa62fd111843d721c213910cdad0f03386db8269301f0009efd86041");

pub const NONDET_INV_TEST_8_CONTROL_ID: Digest =
    digest!("06b2d40f2642aa45bf36d376f8620e6cdc4e6b574924f31a1f229e6b9345422f");
pub const NONDET_INV_TEST_8_CONTROL_ROOT: Digest =
    digest!("9161051265033a043167c53126a2c052e9022d69ba54256bdd75260bfca99e37");

pub const CONST_ADD_TEST_8_CONTROL_ID: Digest =
    digest!("6a8afb2846b4af55dd68cb5b58a059493fbe0b365a21322fcd39b5308ee40917");
pub const CONST_ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("1fca6f6d47c86268fe193a123eae674f3b56e55fe454d7743583f1492e8d3865");

pub const CONST_ADD_ALT_TEST_16_CONTROL_ID: Digest =
    digest!("97a96a16ce3b5d0f84cbb4488f18a359d626ac0b74124413f679634df19b0567");
pub const CONST_ADD_ALT_TEST_16_CONTROL_ROOT: Digest =
    digest!("4bd8e046e365a53ba83f1e5860c9634379fd941863542135837faa302be8e706");

pub const CONST_MUL_TEST_8_CONTROL_ID: Digest =
    digest!("82aee8246da8193e045a901c078ee9752d742d35613f8f3e05ac1c18c6454e70");
pub const CONST_MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("28ac5116566e2b1f312d204cfd6d972e2d70a51a75d9f93b6dda101e2b513649");

pub const ADD_TEST_8_CONTROL_ID: Digest =
    digest!("7a5e693d29efca181c14524283d3c64a2e39d15b1e003744c985d46f62ccb554");
pub const ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("b6666525d30389428d32e651c6742647a1d6de69b6639e30d7c30c616c879855");

pub const ADD_TEST_16_CONTROL_ID: Digest =
    digest!("0949613353166c5c842b5a0b3fbc355f16a39b03d534652a2fbf870fa134c02a");
pub const ADD_TEST_16_CONTROL_ROOT: Digest =
    digest!("87b5c15b6f4e166af7ef4f30a9bb9319cfe07e11258a0f6c5f7e6d5f3f872c0e");

pub const ADD_TEST_128_CONTROL_ID: Digest =
    digest!("79b36c33fbb721755a0a3f1336e909712e2c6c4c4e2fab33ecfaf32d8ed32754");
pub const ADD_TEST_128_CONTROL_ROOT: Digest =
    digest!("0047b84a1718a954d33fda38e74627490ac1c60ab71f125c559d76754fe72e22");

pub const CONST_ONE_TEST_8_CONTROL_ID: Digest =
    digest!("1f8fc4087a0e1d4c2348c16cc0ce5e721776ad1803b3eb1754851a2e4e81f002");
pub const CONST_ONE_TEST_8_CONTROL_ROOT: Digest =
    digest!("a067705e9fc51c2b5128843c7f2c9a10691dbe271d40fd3f3d56cf00930e7775");

pub const CONST_TWOBYTE_TEST_16_CONTROL_ID: Digest =
    digest!("ee9b1730160a75140d0d470ab20f353eff375b346647012ec8733d41e821a733");
pub const CONST_TWOBYTE_TEST_16_CONTROL_ROOT: Digest =
    digest!("9a258702faa4b75a2e7fb96a254a0e14b65619562d0b5a0aa4c80917b6cdb62b");

pub const SUB_TEST_8_CONTROL_ID: Digest =
    digest!("91afae323ffc1f227c0d731ca06ec75890875a0b8fd1211cf4e4486df2b9813d");
pub const SUB_TEST_8_CONTROL_ROOT: Digest =
    digest!("ba12ed3af526b04e5248501219fa09271cd6a705ab0ce132c329ac7777f84b02");

pub const SUB_TEST_128_CONTROL_ID: Digest =
    digest!("60e1270202c0d9453c421a2cd02c773ed8efbe2fe38c4b479729546ed8e56c51");
pub const SUB_TEST_128_CONTROL_ROOT: Digest =
    digest!("57d56a4322dadb0b86a13670e4cc114550d0480c0f143423dd1fcc1632c3bd0d");

pub const MUL_TEST_8_CONTROL_ID: Digest =
    digest!("b84be75d8dc2e6270149bd07cc3fad38d8f84226400cc85dc7b3ab72b1aa6207");
pub const MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("08be6355563b4b47ade35c1391e89a450e5bf3264f41f03abf532e746864274b");

pub const MUL_TEST_128_CONTROL_ID: Digest =
    digest!("80b4823af8eaaa1c1a844f551a4ff055e49bfc589ed8d701069b64637694f770");
pub const MUL_TEST_128_CONTROL_ROOT: Digest =
    digest!("a3c5f2432fbcf66e70d9c1770ae8bd124c435b471c124e3f249b054ed14a0249");

pub const REDUCE_TEST_8_CONTROL_ID: Digest =
    digest!("9a32102c3c7772299d47435a0c079012c41573215b678454210a045615cd0f5e");
pub const REDUCE_TEST_8_CONTROL_ROOT: Digest =
    digest!("1a23e116f3e13f02e0c0325cb37ea75979ee7947bfd8940ca798350ad5ef273c");

pub const REDUCE_TEST_128_CONTROL_ID: Digest =
    digest!("d9803b4d5d64c238de13315770ed3b372332622fedd05a0950fa5b376b368c19");
pub const REDUCE_TEST_128_CONTROL_ROOT: Digest =
    digest!("9ea6c9016ca0036732577e6e4feabf43e418b800a9bb8d6d0ed5e04c47ef2757");

pub const EC_ADD_RZ8TEST_CONTROL_ID: Digest =
    digest!("52add15e6c87854ebf70d8639de5b109df5080151c79c94e075dab4126d2d655");
pub const EC_ADD_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("bf682d4767cb4e55ea9c055116e8176cd54155079505e44908e87c045f9d4f72");

pub const EC_ADD_SECP256K1_CONTROL_ID: Digest =
    digest!("cf043b24ef46df4ccbeb4967df38a4211c337955427b3d48ad4d6c2cd98e296a");
pub const EC_ADD_SECP256K1_CONTROL_ROOT: Digest =
    digest!("1650403430ef2458b6dcf030ae9fc84de27b5175ab6cca259d76784e83181b04");

pub const EC_DOUB_RZ8TEST_CONTROL_ID: Digest =
    digest!("1f7120464fe21d3ce400ff318079e85394661267ce06a01070be113dc3e7516d");
pub const EC_DOUB_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("a06ac142a65efa3f1e3bd7161bcf41304be53913e7065c01cec1c31006447821");

pub const EC_DOUB_SECP256K1_CONTROL_ID: Digest =
    digest!("1ecc8815d86f891d758c9b329350685b359ab769141d804934b5764023bdae62");
pub const EC_DOUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("24d66c2f3a3e914b694d2e4515f75369795f32324037425ee5242f1ff6a28f31");

pub const EC_MUL_RZ8TEST_CONTROL_ID: Digest =
    digest!("d29efc644d234c6b1ec3eb532e3ee3751e3ac145f1f6680ba05bb80c44af8c3f");
pub const EC_MUL_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("3e4d630a41d7f93e752be2536b41f536a05e286ac97589751d8a2b3aa478d331");

pub const EC_MUL_SECP256K1_CONTROL_ID: Digest =
    digest!("8872f80f638cf427e8b8d1435d9bb0387278c054a44ec16bef877226bef89731");
pub const EC_MUL_SECP256K1_CONTROL_ROOT: Digest =
    digest!("f833fd0312340f607561cd74e875ac0863ab8825dcb16e611a8bf45296b0ad58");

pub const EC_NEG_RZ8TEST_CONTROL_ID: Digest =
    digest!("bc8fd97360bb286bc468c320c435cb283c6f2400528b6a738865302da8417741");
pub const EC_NEG_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("1e92f00315ff353ce874d0711d3e3e3a5d13f368c254165e8dccb572e3c2d93a");

pub const EC_NEG_SECP256K1_CONTROL_ID: Digest =
    digest!("55494e3c5067575436d6b30b82465f60b5f7b947ec4e423eb0c0795cabd44c50");
pub const EC_NEG_SECP256K1_CONTROL_ROOT: Digest =
    digest!("6a57de42c3e336197f4ae46b183b1b57cba8090d4939a50c4dca3f0fdae7bb73");

pub const EC_SUB_RZ8TEST_CONTROL_ID: Digest =
    digest!("78c9c70e94efcc3b0a2f3a077a79581c179e3d1225747e1de9f85860865fc91d");
pub const EC_SUB_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("47d6d54a039dac132aa0a36832366166360fca04f137603653383332e019aa68");

pub const EC_SUB_SECP256K1_CONTROL_ID: Digest =
    digest!("9e57820c8b19c90b3cbe4268a096645c5997a9179367481f324d667115097318");
pub const EC_SUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("a4c5f5442b205c6afeda5c691376870460fdd01903cd9e1baf4b703a9451a959");

pub const EC_PTS_EQ_RZ8TEST_CONTROL_ID: Digest =
    digest!("23716c007eb3245bf1ca6244de8f4d090c99500cf8d2f42e9e96d763e1eff31c");
pub const EC_PTS_EQ_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("765de037db9ec4171e4d5c76e3935f0a3de2f575a7ce645c57c354212825c365");

pub const EC_PTS_EQ_SECP256K1_CONTROL_ID: Digest =
    digest!("b6f0cd6a1972504a7de3180fd9fc6b0c50f4656ae7a84a567744fd64d024dd56");
pub const EC_PTS_EQ_SECP256K1_CONTROL_ROOT: Digest =
    digest!("7b923f30d5298427e2ded313b50f1843ae73595115339b104a6fde13d42f505f");

pub const EC_ON_CURVE_RZ8TEST_CONTROL_ID: Digest =
    digest!("4944b6188bf6b4213fd42a0dadc07b2e1cc4c211c2660b6e080496505301ba1d");
pub const EC_ON_CURVE_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("189925511fd4593d75e5cf64add840260feef65112dade5600ccbf36c53e1430");

pub const EC_ON_CURVE_SECP256K1_CONTROL_ID: Digest =
    digest!("fce01c1f235ec3075d51ef33a91b6e57faa15f43dd90443aba5a930845a44548");
pub const EC_ON_CURVE_SECP256K1_CONTROL_ROOT: Digest =
    digest!("9603bb2fa098040e682d5436ff41cd0349d607337772a9021bf3664dca48370f");

pub const EC_ADD_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("ca7d3f1c7a3e2d728488ef5075d817161aec8f4ab7ad380bd263c65460b0672b");
pub const EC_ADD_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("f4cf263ae188624c7ece5041efe9c749705f52753c055a16f8ee05372e2c091f");

pub const EC_DOUB_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("123df152a5780f50996f132323d9f31892118e33dce5245da8837a478946d636");
pub const EC_DOUB_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("7621081779097d2908980327c992fe1ec376fe555cfb5a385978e550e5321250");

pub const EC_MUL_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("7e99005aed9736637aeec062c47cf06fdc52f32a7727d355d1de7f53ef380c73");
pub const EC_MUL_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("f4045a0323c3c840c6a7693f9fe58037b02cf701f2961335cd598e70497f2958");

pub const EC_NEG_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("6be0ed5b7d73e145e44c470f487541198ecb865294fbbd4066f9fa6884dd201c");
pub const EC_NEG_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("ada5fb4ca6f8b3029da7846eb0236b36d1284f03b6b9d277a02fa8361941200e");

pub const EC_SUB_FREELY_RZ8TEST_CONTROL_ID: Digest =
    digest!("3be263544d179c0cd067d274611f0f1bbb4a9d6e9b364c566983ba1c82ae1e13");
pub const EC_SUB_FREELY_RZ8TEST_CONTROL_ROOT: Digest =
    digest!("eca68a5f61edbe544fbd436802b4a54bd3c47b67d25e9d66e6356f1de35a4363");

pub const REP_EC_ADD_SECP256K1_R5_CONTROL_ID: Digest =
    digest!("3ed8847536118517dcb2bb4d47dfda621fa8c7610256e827f9806916df23836f");
pub const REP_EC_ADD_SECP256K1_R5_CONTROL_ROOT: Digest =
    digest!("43c5bc35ffe86c4feb7a24181a152a209818ee1606459d319f68d922949a9938");

pub const REP_EC_ADD_SECP256K1_R10_CONTROL_ID: Digest =
    digest!("949209179916ab2044f1604131f84d4b01b20d1a1122d6646e3c053fa9c3d810");
pub const REP_EC_ADD_SECP256K1_R10_CONTROL_ROOT: Digest =
    digest!("4ec2e81dbda2ee485d1f684964462c5056590859b6d1af5aebc9e950e3995d69");

pub const REP_EC_ADD_SECP256K1_R256_CONTROL_ID: Digest =
    digest!("c059be3183bea541e5afc4359b83d63c87a8fb29e640103cac7b0a6b8b16646a");
pub const REP_EC_ADD_SECP256K1_R256_CONTROL_ROOT: Digest =
    digest!("cceca96dc0077850f4ae4243a1a2fb73623a010f5971ab591def564ab5d9a43d");

pub const REP_EC_DOUB_SECP256K1_R5_CONTROL_ID: Digest =
    digest!("318a500b5ec17e47fdd1d47624cb11532b24fe609a74c2095f6e26504ed31d2c");
pub const REP_EC_DOUB_SECP256K1_R5_CONTROL_ROOT: Digest =
    digest!("df0f3010a4be6571119afd13fa2842422821440786e6be581802fa6baaa6090b");

pub const REP_EC_DOUB_SECP256K1_R10_CONTROL_ID: Digest =
    digest!("e4b22a149a8019521a7c6a3ab4fa5f15a86bae24e7041c511cb43d6e99eae521");
pub const REP_EC_DOUB_SECP256K1_R10_CONTROL_ROOT: Digest =
    digest!("895b27089623015acd4c4904b34e063fad2d853f336ca96576586e35ffb05315");

pub const REP_EC_DOUB_SECP256K1_R256_CONTROL_ID: Digest =
    digest!("3f681837c6c2375586851645e7463042cfa46e5d35908362ab5b1c4c05b7b536");
pub const REP_EC_DOUB_SECP256K1_R256_CONTROL_ROOT: Digest =
    digest!("43eeb350be7e232701203c41ca2b7039eefde1650e3113341dbcfe210cc3e670");
