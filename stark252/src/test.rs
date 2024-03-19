// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// BASIC ALGEBRA
// ================================================================================================


#[cfg(test)]
mod tests {
    use crate::field::{add, div, mul, neg, sub, Fp, MODULUS, TWO_ADIC_ROOT};
    use crate::scalar::{
        add as sadd, div as sdiv, mul as smul, neg as sneg, sub as ssub, Scalar, SCALAR_MODULUS,
    };


    use crypto_bigint::{U64, U256, Random};
    use rand::rngs::OsRng;

    use std::vec;
  use hash::poseidon_impl::PoseidonHash;
    use hash::rescue::{apply_inv_mds, apply_inv_sbox, apply_sbox, Rescue, apply_mds};
    use traits::traits::{Field, Hasher, PrimeField};

    #[test]
    fn exptest1() {
        let a = Fp::new(U256::from_u32(17));
        let aa = Fp::new(U256::from_u128(59));
        let aaa = aa.0.to_words();
        let bb = Fp::new(U256::from_u32(2));
        let res1 = Fp::power_by(bb, aaa); // 2^59
        let res2 = res1 + a; // 2^59 + 17
        let res3 = res2.0.to_words();
        let bbb = Fp::new(U256::from_u32(3));
        let _res4 = Fp::power_by(bbb, res3);
        // //////println!("{}", res4)
    }

    #[test]
    fn randgen() {
        for _i in 1..51 {
            let aa = Scalar::random(); // need to give some input
                                       // let b = Fp::rand();
            let _c: U64 = U64::random(&mut OsRng).into();
            println!("{}", aa);
            //attempting to write the numbers in a file
        }
    }
    // inputs
    //A
    static LLA: [&str; 9] = [
        "008B594D1921A75BB78C0FF7793DF70D2102C35E1394FDE6555EFCCDDBB0BF9B",
        "04747A444BFAB0D1D21D02202DDD7567F85D6E71008AA2E5DC83143DAB27F86E",
        "05F1A2DE978462876FAF6C16E62B977AC1CB9F60FEA7AFF4815D926046401AA0",
        "05EC813FD2213CBDF2DF3F793D92D4EEACE5A15C5D95BE8542D53CB715DC9E55",
        "0337E447AEA0F6BD8B07E32FDB2980FFF8BAC4A8EC5EEE5FC14624E1E2AB5A20",
        "02B06FF1258DA97CCE0EF9A5BEA5000229AC8D7B4A941B3B1F8CC40211F4BDD6",
        "06BB9AB4D4AA4A8E7FDC0D189BB89B0A92A25CE2DBA3F2FC2D419353FC9880DF",
        "04E7A4A56B0332BDBCC7EAA0EE136C4DA7796D0CD2F02CFF1C0B6FEEF1185DA1",
        "05F3DF7DA9CE5314ECC3100878F36915F5D4B9A110C7D0C179B9619772F03F0D",
    ];
    //B
    static LLB: [&str; 9] = [
        "043E4736351A18CDADF4E88256B709265F4B6DDDDE596C4EB6AFB0D9001365A8",
        "057A8F4C2D21D1212B3FF69FE349E8B30C17EF9BF7F10DC4936A23A87A7221C2",
        "079C8EB3E9C0760BE512E4C9A6308ED56208736D7B738A04E7E72C3468BA7A96",
        "0037B99DA5A127C90C90DD92815C3EE650110A2FA9310939F9DC905B06069198",
        "0050276A5F8373AB6FB93E19F5122B7647679BBA3D7C5762022F6B8A7819BE61",
        "01A10A90315E2A8765D30278DE0F02F5FC7980ABB42E34DE1FE8051711867B31",
        "0765CCAAB766CEC0445C9AB37F2388DC12163470DBE911EBE293865B3043A5BB",
        "042BCD3371D6F34945E2964686BE9D785BACAE82883A8BF5711B1C2F2B8302D5",
        "074D959ADBCC0C189F4F3D40FD81A2A8DDDA85EED4ABDE561DC3C6C6332B15D5",
    ];
    //A2
    static SLLA: [&str; 9] = [
        "00AA3E516998FAEDA2D1AE599DFEF562031C8F5DB52A2EB24B05FAA336216FE1",
        "031C8498A8CFE4894627495EA5F74455701EA10B80DE8E93A4C9ECA6A4670362",
        "05E5C8ABD0E285AA4A22E2FF973188F74C266F3B4A2FEDAAE6C5A0586FD799B8",
        "01BE4784E72173FA4BAC15F19CC9324860219CB4FA290431A0FF050B4984D72B",
        "063B933AA64CF5E8F8C7F413527AE43D206CD8F06F42EE2EF8F69D41599DBB35",
        "012A06DF916511CA45F06581954E21BEC2F89940F3E50FD039009D192FCC2312",
        "02B6F4ECA65A6C228C913257CF9AD22B0531AE11216E46624A78D515A43D6FD0",
        "02567FF759FFE2527CC92885A0936AF01347DB48DA992EC93E55DD5212D0E72D",
        "00DFCF9E22F054C2BA13CE20B378B4FAE34DC0BDB5CF755CB5F92C0A95A16065",
    ];
    //A3
    static SLLA1: [&str; 50] = [
        "0155659A8DE5311B9FB48FC4A557CF7572D77EF96734EACBD70DD2958207AB5D",
        "04C1F7F1BFEB1D56382FBBBEC85522E304724D50C8B7C4274AE7638F460BED26",
        "07DF5826E2AB48C1D5CFB615553067194EFE308E8850AFA3AF857C00F232B1DB",
        "07DD8FFB7BB246AC7DE178ACD3E401A10BD8CFBE380A05565F10F12D86DC2F90",
        "04FD71BF4F9E6FD2E02F8578A492B8B3F2B066D876F9AFDCEE0912F77C87BF0D",
        "02946CDD8369A24E7FD089EA925F6171929406CB05B909B6BCE33033341B500E",
        "018D70C73581BDA9F0F73F8926EBDC94871E1374441D6558EC9F62E0F1C00B44",
        "01CBB225670E85C853787E26A3B8FC9926640DE84C7570E140E595CEEE82CAFF",
        "01FD5B8081CA92EE713855541A9999D8F81B6B900D80CE7EE116B8E7FC53B6A4",
        "04E766FD5AEE8DCFC9C064C37621D78F0AF67C841E1AF2E6B689BE7862F16F83",
        "01D4BE4F27A13BC8600FEA7C126615684D59486DD1175EA8DD983EC3E2E01AD7",
        "03E2A011AEF578C1FBDB6664524CD80E5DD16B9D5BC1F10F33A894994A7A0A7D",
        "04BBFBDA41165FF1015B540AE4BC1C48014DA3102B1D1A30AB0301E42BE4B662",
        "063AE3624A2361F1778814AD84AF3CE22B609A170E0D74CA34E7C2440C93C7DD",
        "0779963850545CFA8A0F98903CD7F62DCDB60DE4DD2DC4CCE28EA8D74FD64FC0",
        "03B8BE9BCB84E8FDF29391F0554FD2721956B683D5B8A39CF7D955654F32313F",
        "00774D792289F29F481FC383CBD3B9E5F5CEE5CC9DBBCCD17AEF6C326808C83D",
        "0408481253E8E217ECECC238176F8F3DA6DA944DBF5D0258D8D8E8184938F4F9",
        "062417F512DFBAF7A71514E9727A8748442ED0EF3606FF7534D2C4486C283E1A",
        "01D44EA7C87D77EB14F6CF24934B44BE9B31360CCAA524129DF4C3D2B6E8A250",
        "032C4AB2016CE06C0D9861806C0D2F9DE45FEB057814ED8AA6A4560F70AFCC82",
        "0365C1ED36714C31AAC87565BD1288CC1CA85807B8D64274EC7A6DAEB09E5105",
        "00ABD117568886C910B8A9EEAC749DD16BEF2AB11B7ED1767007FF3672ABD703",
        "04F6FED9FCE5ED8EBF70A9F281EAABE75F58DDD8C2CBEB10D4DBE10B5583B9E4",
        "068194BEC4AC229BC4BB61DFD407262EA8826A9AE86122470A412AA67A220C5B",
        "0737BD814FFE4B16FAB147DE8526FA2D1742523E63FDE5EFDD93ADB10AF4F813",
        "00DC786A7FAEBD9BA97BC5705963E80C84F368218718BAFDA9705BA31BD85B76",
        "0153A0CF3BF6EF818AAFE7D17EAE03DB553EDF8520F28928887A1E2BD7E9D24F",
        "068F63FD684D3B0719A971D566D1208C8E37BD57C12EBC0F3DFF56426124984C",
        "03544ADCB1117A8D509C9BA7D7270EADB4F6941762B6884E0B9535AD4977085E",
        "02B73CAC28B3FFE51A00C275CF5823E17C4862D90A11569682D477ADDB7D0D1F",
        "06F396D1774C10D0B463C98A19DFC0E2A118A27FC5C21623AD85128E0CA37DA0",
        "003BBE019E16198CB015C6F6427E56014901D5A595C7F82F349E947F4E9E51D1",
        "047AFBE8C093C0840179615614B080FE46A3059479F70E96CA58F11D2F2BAB75",
        "0474E7D2E5D17EFF0AE9FE98C302A5C58F0BF556B844DF679ED0A0A66B0BEE06",
        "079B0F067743E989582EE21BEE9031A22856D6ED910020659A611993505E66FA",
        "003E9494F003E878D934435E81863FE6F5A4E3D307EE6BA24241FB61BCAB67FC",
        "0337B24F5959EC92C7A2111E75138FB212A9651AA483CD3D38D981530EB94902",
        "05816D7B9E1DB1A9C7C95221762E05165694D8650A73F2DE87C4AF153DE07D81",
        "00683A211F0B619A8D75A7824D1A71ED90061D1005E56D4A8053FEF50D147E51",
        "0073461C8B5EBC513D31E186A54EFF149FBB27B385204873830EA6058D0782FE",
        "038E0AE3CC6DE0A837996C2F9501D225A0524399B3B3F1E209287978EB03F3BB",
        "02C35013015CB9F8F42517AA3538A746EC76403C71F582520EFA809DBDDF0830",
        "06DA047A8C064D8B83B0E975F88A5DDC3FDE940A1D615899B9781012A4FD5A19",
        "0220AC14433DD7FF1EC282F49877B06E45CC5E5B65B49C73887B708EA6BB2FA7",
        "0360B11AE203B183D24E9534E0562E09327342F936A2D250D9FB25C09550678D",
        "0726D1EFE71F57D13226A3FA0DEDC46CEEC8D7072EFFD201037A7BAF6A1F6E64",
        "0335C4BA14BBD9B6C8BF14B31CD0C2ED3330ED833107C7FE8FE7F9AB201E5326",
        "018EB76691AE56D54A0CCB8833961FCDD62D94B7CD7492C48060CF9D99AC254D",
        "02E7470725255A27F9CC739C92DBD32E90BE0E83E33B0501CED9934FA617D882",
    ];
    //B2
    static SLLB: [&str; 9] = [
        "03B9D2921FE5D7B96B7BDCA65F61569A69CDC02111ABCB4E5C95AF23F6224F10",
        "07A969DE7A940DC3A779E6A1E5A41C787FEF824E08E52418506F2123A3DAE781",
        "0137BEF71AC021B412E24BA6E38ADF1224C66D69CC6C3528E79EA8E965478DC3",
        "02E735BDFE0BB8AD01B4CA14CBC01CFFE20BF2EC985062EF0AFF4406E59AB5F2",
        "068F5C7023D879FB623CD73808A3E6C31FE275505D2C6097AA1459BF1B9A43A3",
        "02B8DF146C80CFD622414C3F630A7CFA193DDD89BD08D31242D80B35CCCFE602",
        "01F203AE2D9A020ADB46D434ECB6A373A0E68362906B23F6F863EA1BB2EB090E",
        "03395745424D1C1AD36B81FDE73CDF88BD5082FA438A1FFFD406733753CFF40F",
        "07C7CDD4B7FE3177B4AAA9CA616E08741C52332E81A0022A916F78DE5C3269A2",
    ];

    // static SLLAB_ADD: [&str; 9] = ["046410e3897ed2a70e4d8afffd604bfc6cea4f7ec6d5fa00a79ba9c72c43bef1",
    //                                 "02c5ee772363f23beda130008b9b60ce388d10ebbedc0079d6d26b889a7b9db4",
    //                                 "071d87a2eba2a75e5d052ea67abc680970ecdca5169c22d3ce644941d51f277b",
    //                                 "04a57d42e52d2ca74d60e00668894f48422d8fa192796720abfe49122f1f8d1d",
    //                                 "04caefaaca256fd35b04cb4b5b1ecb0088ce3bd301879c9484a454bec771b1a9",
    //                                 "03e2e5f3fde5e1a06831b1c0f8589eb8dc3676cab0ede2e27bd8a84efc9c0914",
    //                                 "04a8f89ad3f46e2d67d8068cbc51759ea6183173b1d96a5942dcbf31572878de",
    //                                 "058fd73c9c4cfe6d5034aa8387d04a78d0985e431e234ec9125c508966a0db3c",
    //                                 "08A79D72DAEE863A6EBE77EB14E6BD6EFF9FF3EC376F77874768A4E8F1D3CA07"];
    // //sub oiutputs
    // static SLLAB_SUB: [&str; 9] = ["04f06bbf49b323453755d1b33e9d9ec750cfe1aa6e6615960cd6edc0edc56e00",
    //                                 "03731aba2e3bd6d69ead62bcc05327dca7b0312b42e11cad72c16dc4ae526910",
    //                                 "04ae09b4b62263f637409758b3a6a9e5276001d17dc3b881ff26f76f0a900bf5",
    //                                 "06d711c6e915bb5e49f74bdcd10915483596bc362cc05374b466634611b06e68",
    //                                 "07ac36ca82747bfe968b1cdb49d6fd79b80b760ddcfe3fc96d48e5c3ebc9c4c1",
    //                                 "067127cb24e4420523af19423243a4c4613bce2501c3eef0148f342510c28a3f",
    //                                 "00c4f13e78c06a17b14a5e22e2e42eb7644b2aae9103226b5214eaf9f15266c2",
    //                                 "071d28b217b2c648a95da687b9568b670d786abc61f6c0fb88b60c5c6cc7404d",
    //                                 "011801c96af2235c05692456520aac867e7c9ffcff17256442f0556de73543f2"];
    // //mul outputs
    // static SLLAB_MUL: [&str; 9] = ["062c9252d83c29b0b8979ebb859dc615ec389ba8d1f226a4a9d76abe808fff5b",
    //                                 "048cfa62badb9cc3b0d59162e504ba6584cbd6904815f55bc05e1e48758daba8",
    //                                 "0223922bfa1a30491efcb41f2a0ad2b50c5c4a8ee4401c184dbe7b9bd48023ca",
    //                                 "00e72c1b89a1f80941fcb9095bc475b62a7ea8b32896acb03a9de18155fcf72b",
    //                                 "06a3cddff176b76c1c22a7389162a1a75901f8bcb13947a54b638e4d1fc00b33",
    //                                 "018740e52f9e255fe5022666d11af272ef19702a2dcc70cbafbe9e1aac2ed711",
    //                                 "02618b6bda239b5cf68fe9efb031863482a058bdb2798c15a27ea410b214d02f",
    //                                 "0362b23227342e7b0ee0ed29c5edc7defed04abb866a7906fc4e52c06ed59633",
    //                                 "051657fb6a9914abceb7cdff1d04013934b53e11228d7b6c20997e612b82e1f1"];

    // only taken the 0 index element of SLLB for computation with SLLA1
    static SLLAB1_ADD: [&str; 50] = [
        "050F382CADCB08D50B306C6B04B9260FDCA53F1A78E0B61A33A381B97829FA6D",
        "087bca83dfd0f50fa3ab986527b6797d6e400d71da638f75a77d12b33c2e3c36",
        "0b992ab90291207b414b92bbb491bdb3b8cbf0af99fc7af20c1b2b24e85500eb",
        "0b97628d9b981e65e95d55533345583b75a68fdf49b5d0a4bba6a0517cfe7ea0",
        "08b744516f84478c4bab621f03f40f4e5c7e26f988a57b2b4a9ec21b72aa0e1d",
        "064e3f6fa34f7a07eb4c6690f1c0b80bfc61c6ec1764d5051978df572a3d9f1e",
        "05474359556795635c731c2f864d332ef0ebd39555c930a749351204e7e25a54",
        "058584b786f45d81bef45acd031a53339031ce095e213c2f9d7b44f2e4a51a0f",
        "05b72e12a1b06aa7dcb431fa79faf07361e92bb11f2c99cd3dac680bf27605b4",
        "08a1398f7ad46589353c4169d5832e2974c43ca52fc6be35131f6d9c5913be93",
        "058e90e147871381cb8bc72271c76c02b727088ee2c329f73a2dede7d90269e7",
        "079c72a3cedb507b6757430ab1ae2ea8c79f2bbe6d6dbc5d903e43bd409c598d",
        "0875ce6c60fc37aa6cd730b1441d72e26b1b63313cc8e57f0798b10822070572",
        "09f4b5f46a0939aae303f153e410937c952e5a381fb94018917d716802b616ed",
        "0b3368ca703a34b3f58b75369c394cc83783ce05eed9901b3f2457fb45f89ed0",
        "0772912deb6ac0b75e0f6e96b4b1290c832476a4e7646eeb546f04894554804f",
        "0431200b426fca58b39ba02a2b3510805f9ca5edaf67981fd7851b565e2b174d",
        "07c21aa473ceb9d158689ede76d0e5d810a8546ed108cda7356e973c3f5b4409",
        "09ddea8732c592b11290f18fd1dbdde2adfc911047b2cac39168736c624a8d2a",
        "058e2139e8634fa48072abcaf2ac9b5904fef62ddc50ef60fa8a72f6ad0af160",
        "06e61d442152b82579143e26cb6e86384e2dab2689c0b8d9033a053366d21b92",
        "071f947f565723eb1644520c1c73df6686761828ca820dc349101cd2a6c0a015",
        "0465a3a9766e5e827c3486950bd5f46bd5bcead22d2a9cc4cc9dae5a68ce2613",
        "08b0d16c1ccbc5482aec8698e14c0281c9269df9d477b65f3171902f4ba608f4",
        "0a3b6750e491fa5530373e8633687cc912502abbfa0ced9566d6d9ca70445b6b",
        "0af190136fe422d0662d2484e48850c78110125f75a9b13e3a295cd501174723",
        "04964afc9f94955514f7a216b8c53ea6eec1284298c4864c06060ac711faaa86",
        "050d73615bdcc73af62bc477de0f5a75bf0c9fa6329e5476e50fcd4fce0c215f",
        "0a49368f883312c085254e7bc6327726f8057d78d2da875d9a9505665746e75c",
        "070e1d6ed0f75246bc18784e368865481ec454387462539c682ae4d13f99576e",
        "06710f3e4899d79e857c9f1c2eb97a7be61622fa1bbd21e4df6a26d1d19f5c2f",
        "0aad69639731e88a1fdfa6307941177d0ae662a0d76de1720a1ac1b202c5ccb0",
        "03f59093bdfbf1461b91a39ca1dfac9bb2cf95c6a773c37d913443a344c0a0e1",
        "0834ce7ae079983d6cf53dfc7411d798b070c5b58ba2d9e526eea041254dfa85",
        "082eba6505b756b87665db3f2263fc5ff8d9b577c9f0aab5fb664fca612e3d16",
        "0b54e1989729c142c3aabec24df1883c9224970ea2abebb3f6f6c8b74680b60a",
        "03f867270fe9c03244b02004e0e796815f72a3f4199a36f09ed7aa85b2cdb70c",
        "06f184e1793fc44c331dedc4d474e64c7c77253bb62f988b956f307704db9812",
        "093b400dbe03896333452ec7d58f5bb0c06298861c1fbe2ce45a5e393402cc91",
        "04220cb33ef13953f8f18428ac7bc887f9d3dd3117913898dce9ae190336cd61",
        "042d18aeab44940aa8adbe2d04b055af0988e7d496cc13c1dfa455298329d20e",
        "0747dd75ec53b861a31548d5f46328c00a2003bac55fbd3065be289ce12642cb",
        "067d22a5214291b25fa0f4509499fde15644005d83a14da06b902fc1b4015740",
        "0a93d70cabec2544ef2cc61c57ebb476a9ac542b2f0d23e8160dbf369b1fa929",
        "05da7ea66323afb88a3e5f9af7d90708af9a1e7c776067c1e5111fb29cdd7eb7",
        "071a83ad01e9893d3dca71db3fb784a39c41031a484e9d9f3690d4e48b72b69d",
        "0ae0a48207052f8a9da280a06d4f1b075896972840ab9d4f60102ad36041bd74",
        "06ef974c34a1b170343af1597c3219879cfeada442b3934cec7da8cf1640a236",
        "054889f8b1942e8eb588a82e92f776683ffb54d8df205e12dcf67ec18fce745d",
        "06a11999450b31e165485042f23d29c8fa8bcea4f4e6d0502b6f42739c3a2792",
    ];

    static SLLAB1_SUB: [&str; 50] = [
        "059B93086DFF59733438B31E45F678DAC08AD1462070D1AF98DEC5B339ABA97C",
        "0108255fa005459cccb3df1868f3cc489aa48d2fb70bf8d8ee51b46b4fe99e16",
        "04258594c2c571086a53d96ef5cf107ee530706d76a4e45552efccdcfc1062cb",
        "0423bd695bcc6ef312659c067482ab06a20b0f9d265e3a08027b420990b9e080",
        "01439f2d2fb8981974b3a8d24531621988e2a6b7654de48e917363d386656ffd",
        "06da9a4b6383caa61454ad4432fe0ad6e0475917bef4f09a7eb42350ebbf4e2d",
        "05d39e35159be601857b62e2c78a85f9d4d165c0fd594c3cae7055fea9640963",
        "0611df934728ae1fe7fca1804457a5fe7417603505b157c502b688eca626c91e",
        "064388ee61e4bb4605bc78adbb38433e45cebddcc6bcb562a2e7ac05b3f7b4c3",
        "012d946b3b08b6165e44881d16c080f4a128bc630c6f279859f40f546ccf2073",
        "061aebbd07bb641ff4940dd5b304becd9b0c9aba8a53458c9f6931e19a8418f6",
        "0028cd7f8f0fa108905f89bdf2eb8173f403ab7c4a1625c0d712e5755457bb6d",
        "010229482130883795df7764855ac5ad977fe2ef19714ee24e6d52c035c26752",
        "028110d02a3d8a380c0c3807254de647c192d9f5fc61a97bd8521320167178cd",
        "03bfc3a6306e85411e93bbe9dd769f9363e84dc3cb81f97e85f8f9b359b400b0",
        "07feec09ab9f11558717b549f5ee7bd7670a08d08ef48a80b9aa488306d62f5e",
        "04bd7ae702a41af6dca3e6dd6c72634b4382381956f7b3b53cc05f501facc65c",
        "004e758034030a5e8170e591b80e38a33d0cd42cadb1370a7c4338f45316a5e9",
        "026a4562f2f9e33e3b993843131930adda6110ce245b3426d83d15247605ef0a",
        "061a7c15a897a042a97af27e33e9ee23e8e4885983e10af65fc5b6f06e8ca06f",
        "0772781fe18708c3a21c84da0cabd90332133d523150d46e6875492d2853caa1",
        "07abef5b168b74893f4c98bf5db132316a5baa5472122958ae4b60cc68424f24",
        "04f1fe8536a2af20a53ccd484d134736b9a27cfdd4bab85a31d8f2542a4fd522",
        "013d2c47dd0015d553f4cd4c2289554cf58b1db7b1201fc2784631e75f616ad4",
        "02c7c22ca4c64ae2593f853974a5cf943eb4aa79d6b556f8adab7b8283ffbd4b",
        "037deaef3018735d8f356b3825c5a392ad74921d52521aa180fdfe8d14d2a903",
        "0522a5d85fc8e5f33dffe8c9fa029171d2a6ba6e4054a1e16b414ec0d37c5995",
        "0599ce3d1c1117d91f340b2b1f4cad40a2f231d1da2e700c4a4b11498f8dd06e",
        "02d5916b4867634dae2d952f076fc9f22469fd36af82f0c0e169a71e6b02493c",
        "079a784a912ba2e4e520bf0177c5b81302a9e6641bf26f31cd6628cb011b067d",
        "06fd6a1a08ce283cae84e5cf6ff6cd46c9fbb525c34d3d7a44a56acb93210b3e",
        "0339c43f5766391748e7ece3ba7e6a48374ae25eb4164ad550ef636a16812e90",
        "0481eb6f7e3041e44499ea4fe31cff6696b527f24f03df12f66f879d06424ff0",
        "00c12956a0ade8ca95fd84afb54f2a63dcd54573684b43486dc341f939095c65",
        "00bb1540c5eba7459f6e21f263a14f2b253e3535a6991419423af18274e99ef6",
        "03e13c74575e11cfecb305758f2edb07be8916cc7f5455173dcb6a6f5a3c17ea",
        "0484c202d01e10d06db866b82224e94c4358361fc12a52860412ee7f744f661b",
        "077ddfbd397414ea5c26347815b23917605cb7675dbfb420faaa7470c65d4721",
        "01c79ae97e37d9f05c4d757b16ccae7becc71843f8c827902b2efff147be2e71",
        "04ae678eff2589f221f9cadbedb91b52ddb96f5cbf21542e4224f212c4b87c70",
        "04b9738a6b78e4a8d1b604e045eda879ed6e7a003e5c2f5744df992344ab811d",
        "07d43851ac8808ffcc1d8f8935a07b8aee0595e66cefd8c5caf96c96a2a7f1da",
        "07097d80e176e25088a93b03d5d750ac3a2992892b316935d0cb73bb7583064f",
        "032031e86c2075d218350ccf99290741d610d3e90bb58d4b5ce260eeaedb0b09",
        "0666d98223580056b346a64e391659d3937fb0a81ef083574a4c63ac5e5f2dc6",
        "07a6de88c21dd9db66d2b88e80f4d76e80269545efdeb9349bcc18de4cf465ac",
        "036cff5dc7398017c6aac753ae8c6dd284fb16e61d5406b2a6e4cc8b73fd1f54",
        "077bf227f4d6020e5d43380cbd6f6c5280e43fcfea43aee251b8ecc8d7c25145",
        "05d4e4d471c87f2cde90eee1d434c93323e0e70486b079a84231c2bb5150236c",
        "072d7475053f827f8e5096f6337a7c93de7160d09c76ebe590aa866d5dbbd6a1",
    ];

    static SLLAB1_MUL: [&str; 50] = [
        "06f936387a201b3000154aaa4a55c23eccfc0e1f31b334048a2c58270c3ed732",
        "05fc6a525e5d231ffefb5527995a97ed9a8f60378325c7cda7f2ca84c3d6d1eb",
        "075863b9de48ed5a108c8a26adc730bafc0edc9805676acc9cf991109e88e67b",
        "0198aa3aa53885cf02693e65cdbdc1dae79d4edde00d75e781a13f0b429b3646",
        "0147169b4a4439b5ab561dc43938bee42fd6e5bd351c0ae634fa36ccc7a1ff9b",
        "052c8c47161fabe5ccf5fab95e90a13e79d9a2cefd222ce930ecf2388ba7d6ff",
        "0128b3dafb38ae1affd8abddd94f79474a715fc5a79334bf0530cc690065f048",
        "05a81ed38dd94aae90ae2256af17801b9d191a0b888c07f6457c7852cf5924e5",
        "014c75618a0d04f3d39464baf2d6a3e8656fe5736a69ff5adc8f633df03433ea",
        "0657e7199662fe0adf234c5e2393c212c75d84e04071acbbd98e379917b957b9",
        "076765f6e84cdd60e6477e9b8eebffbfeb180190a90bd4480ac623a0ba3b2dc1",
        "04447ae353cc1ec4086b1776b46cb52bdd37a7ca44851082a75a31c83d664aeb",
        "05d241c1b5e9e91d7c5344247f1cfe4f9768e128c2296666d77b8ea5ee8db433",
        "029e5cee7a767b109f86f1d29c43b7b6790c8f997397b1e2e8b66a82d197f585",
        "07daa2ea41ae0b1e8e6e6d90403c61fe41c08816a81f371f97c28d66efe2947d",
        "07b6f3c4a66aeea5cdef1df270b36c1bce057fcde07e73e8c73bfc6037e4e504",
        "058b5b419aea980ed73092b8ed7fb8d78191c4fb2d85b4bcede2cb807f346884",
        "003641e7f31e8ac208ca869317bc6a05a28a572d09f41e9cd3d4fcf6ce4fc67a",
        "00f2c8e5b36200c9c6a62ea7e0a1f79045f937ed5e186280aca837923e36276b",
        "0024ded0e6fc616553eed5126ccce4b157a36550a54c703070b4ddf6c14348d8",
        "0239f7b12c52af33d09774532033fd6ba04e0e6ce2b5fbeaad8ac336b2baddbe",
        "01b553715509185abb0d09a191ba38cb91eef7f5d295a71d176c69b9e95f725c",
        "05dfab21bd715c25a91416b30e33d0ae4cd81c59c1963be9775f17313c85466a",
        "05cbe7393e2a85f3844db139b4fa9ba738edcbec8b32d24da5523edbea7c00ff",
        "050e6c10105294ee85d42f85e161c15780e6bc31ed71c3689fa1059fc622ed37",
        "075728b9f67dd848fc2b80b5d4e4511888fe52a7a4bae10b90d6493a6b8dc2a4",
        "00f603866ea0d1b5a0001d9d1681355cd500a341dc07644b144069dec7ec066e",
        "06cfe40666584f2ca30086a5a5933ba246386213710061f90f0ee844281cc1a5",
        "07a48cb19fa67425dff1f332a216fee54f59fa9e4fbe9aea69724ef53165aab9",
        "05ac65065408da6d8c37d707f0bb85871fb81b63f4d3bcdd5b5ca56ec2cf07d3",
        "00154ed74247ee3c930d27e752dbc45bf81f696908ff81537237b7cf719e6058",
        "0228c51cb2f38bec8470fce1cfaa74aa8cb1f46308503d19803260efa6b73a40",
        "04c9aaec91e5382cb1759ae569bda6607094f66baf8e2549de332b40abba2f04",
        "07393ad6ba08165b297efb33c684538d586ad7bc9ee5b7d4b5d5bac5a510e4b4",
        "05bae2683250f78a5691b903904cc03e00d6325738fea8f402357dabf236e4de",
        "011f4f1b6b9fa5d025c50e22e6ecb90ab423c801b38835f4aed5696fb60493ad",
        "04bfc2eeced8087f4e66bdb88584a385f82152930ad752bb48ebde1856d8ef60",
        "0613590f804a541735a08e9724e9c939786de331ed64bfd15ba64484e6f02639",
        "056a46a5629afe0f969df4928e61ade65345c97c9d8dccc40967fa88b98c77a9",
        "057e196e3595c454c1faec8eda1b7f7972d3da848aa016591ae0626e3a5f5462",
        "0586fc2c392f4de2953e85f08963905da382e2679eb41bd4221b230fdc198957",
        "000f912d3211dd7054419cd8cc888938f41e5802113decb20a7ae2b2c8e87b87",
        "05c9cf85767f2087cb7dd092824b1c7b244e8796a1a9ed3b4b827c50731c9b65",
        "07affb5b5555df64c397b0a9ac8434520b12f9a5cb30821d28adc3c29f93bdc6",
        "043b4976332efd74b891c990575dc8fbef48c06a18446aebf93b9b03c171b905",
        "047e5d86f8b187c2e7af9095a99c8c30de2042af98198c9679706cea04d271a9",
        "052c6c85928e7699671ca7da888ce308efa4bf8c1fc6a7a5ce52bb1900fd1412",
        "02bfeab89a8071730c62c11a05dfca3c54ae3fdb7644a69d78f99478ae1fe0e0",
        "074a41e4b5c32127d9ccadfc429ccede4bff2fa8e27e8a21c60f248252fe1d23",
        "01d69af08587c218257aedadb10ce24364e3dae459093ced5b32d51995b5f068",
    ];

    // LLA + LLB result 256 bit
    static LLAB_ADD: [&str; 9] = [
        "04c9a0834e3bc0296580f879cff50033804e313bf1ee6a350c0eada6dbc42543",
        "09ef0990791c81f2fd5cf8c011275e1b04755e0cf87bb0aa6fed37e6259a1a30",
        "0d8e31928144d89354c250e08c5c265023d412ce7a1b39f96944be94aefa9536",
        "06243add77c26486ff701d0bbeef13d4fcf6ab8c06c6c7bf3cb1cd121be32fed",
        "03880bb20e246a68fac12149d03bac764022606329db45c1c375906c5ac51881",
        "04517a8156ebd40433e1fc1e9cb402f826260e26fec250193f74c919237b3907",
        "0e21675f8c11194ec438a7cc1adc23e6a4b89153b78d04e80fd519af2cdc269a",
        "091371d8dcda260702aa80e774d209c603261b8f5b2ab8f48d268c1e1c9b6076",
        "0d417518859a5f2d8c124d4976750bbed3af3f8fe573af17977d285da61b54e2",
    ];
    // 64 bit size input
    static LLA64: [u64; 9] = [
        14732985017244624348,
        10339731117559391004,
        17616807185780637841,
        14807429074202401624,
        7358515811370321878,
        17677992502088555874,
        1635918360738187706,
        13051435627668120240,
        16701880237493775153,
    ];
    // LLA+LLA64
    static LLA64_ADD: [&str; 9] = [
        "008B594D1921A75BB78C0FF7793DF70D2102C35E1394FDE721D5109E5DB56577",
        "04747A444BFAB0D1D21D02202DDD7567F85D6E71008AA2E66C012EE8FA561B8A",
        "05F1A2DE978462876FAF6C16E62B977AC1CB9F60FEA7AFF575D90B2C77411B31",
        "05EC813FD2213CBDF2DF3F793D92D4EEACE5A15C5D95BE861053CB014A7CB9AD",
        "0337E447AEA0F6BD8B07E32FDB2980FFF8BAC4A8EC5EEE602764D806957E21F6",
        "02B06FF1258DA97CCE0EF9A5BEA5000229AC8D7B4A941B3C14E19C86D6F69B38",
        "06BB9AB4D4AA4A8E7FDC0D189BB89B0A92A25CE2DBA3F2FC43F58668E1EF4A99",
        "04E7A4A56B0332BDBCC7EAA0EE136C4DA7796D0CD2F02CFFD12B737CBD218051",
        "05F3DF7DA9CE5314ECC3100878F36915F5D4B9A110C7D0C26182612D5FDF2A3E",
    ];
    // LLA - LLB 256 bit
    static LLAB_SUB: [&str; 9] = [
        "044D1216E4078E9F099727752286EDE6C1B75580353B91979EAF4BF4DB9D59F4",
        "06F9EAF81ED8DFC1A6DD0B804A938CB4EC457ED5089995214918F09530B5D6AD",
        "0655142AADC3EC8C8A9C874D3FFB08A55FC32BF3833425EF9976662BDD85A00B",
        "05B4C7A22C8014F4E64E61E6BC3696085CD4972CB464B54B48F8AC5C0FD60CBD",
        "02E7BCDD4F1D83121B4EA515E6175589B15328EEAEE296FDBF16B9576A919BBF",
        "010F6560F42F7EF5683BF72CE095FD0C2D330CCF9665E65CFFA4BEEB006E42A5",
        "0755CE0A1D437BDF3B7F72651C95122E808C2871FFBAE1104AAE0CF8CC54DB25",
        "00BBD771F92C3F7476E5545A6754CED54BCCBE8A4AB5A109AAF053BFC5955ACC",
        "06A649E2CE02470D4D73D2C77B71C66D17FA33B23C1BF26B5BF59AD13FC52939",
    ];

    static LLAB_MUL: [&str; 9] = [
        "01bc0eb8170a32fbc6b6548f1cc62330713ce9e5094f187c77fe9fee725bc175",
        "042a77b70882bd68be108ad3078459c9cf5c1abd611ac8c06e5b0b4b64f0ed1e",
        "05c8bc6db51258d96c5e313bcb8670545459939304f6bf764ed6958d3422afa3",
        "04C65BAC2C6106202554E236DA9430870D8DF7C76B84AEBA7335B76EE5791837",
        "012D3FC45B8065CA18F68C22ADCF146807A27FB31C8858D9AA89F6352A3154E0",
        "028D5EE2DD005C7A69FBA5BDC25AA6F8614BD02FB88E8FE52C5DACA4C596026D",
        "02ca32f84fb6056e22634943cd916fb94058354d6203dfbb660672d0a73d8e2e",
        "02412158b0b6af4099ab10573ea71a4ef8a98c45a09596945e31146b470aa2bc",
        "04046308d4de3dbdbfcff78b8193b29f59856b54f98a6fb6dd2b15356d9ff236",
    ];

    //==============Testing field
    //addition on base field
    #[test]
    fn fadd() {
        for i in 0..9 {
            // add works for 256 bit
            // add works for 256 + 64 combination
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            let b = Fp::new(U256::from_u64(LLA64[i]));
            let c = Fp::new(U256::from_be_hex(LLA64_ADD[i]));
            // //////println!("{}, {}, {}", a, b, c);
            assert_eq!(c, add(a, b));
        }
    }

    //add assign
    #[test]
    fn faddassign() {
        for i in 0..9 {
            let mut a = Fp::new(U256::from_be_hex(LLA[i])); //a
            let b = Fp::new(U256::from_be_hex(LLB[i]));
            let c = Fp::new(U256::from_be_hex(LLAB_ADD[i])); // a + b
            a += b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }
    //sub assign
    #[test]
    fn fsubassign() {
        for i in 0..9 {
            let mut a = Fp::new(U256::from_be_hex(LLA[i])); //a
            let b = Fp::new(U256::from_be_hex(LLB[i]));
            let c = Fp::new(U256::from_be_hex(LLAB_SUB[i])); // a + b
            a -= b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }
    //mul assign
    #[test]
    fn fmulassign() {
        for i in 0..9 {
            let mut a = Fp::new(U256::from_be_hex(LLA[i])); //a
            let b = Fp::new(U256::from_be_hex(LLB[i]));
            let c = Fp::new(U256::from_be_hex(LLAB_MUL[i])); // a + b
            a *= b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }
    //double, triple for base field
    #[test]
    fn fdt() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            let b = add(a, a);
            assert_eq!(Fp::double(a), add(a, a));
            assert_eq!(Fp::triple(a), add(a, b))
        }
    }

    //square on base field
    #[test]
    fn fsq() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            // assert_eq!(Fp::square(&a), mul(a, a));
            assert_eq!(Fp::double(a), add(a, a))
        }
    }
    //subtraction
    #[test]
    fn fsub() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            // //////println!("{}", a);
            let b = Fp::new(U256::from_be_hex(LLB[i]));
            // //////println!("{}", b);
            let res = Fp::new(U256::from_be_hex(LLAB_SUB[i]));
            // //////println!("{}", res);
            assert_eq!(res, sub(a, b))
        }
    }
    //multiplication
    #[test]
    fn fmul() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            let b = Fp::new(U256::from_be_hex(LLB[i]));
            let c = Fp::new(U256::from_be_hex(LLAB_MUL[i]));
            // //////println!("{}\n{}\n{} \n", a, b, c);
            assert_eq!(c, mul(a, b))
        }
    }
    //square
    #[test]
    fn fsquare() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            assert_eq!(a.square(), mul(a, a))
        }
    }
    //invert
    #[test]
    fn inverse() {
        for i in 0..50 {
            let a = Fp::new(U256::from_be_hex(SLLA1[i]));
            assert_eq!(mul(a, a.invert().unwrap()), Fp::ONE)
        }
    }
    //division
    #[test]
    fn fdiv() {
        for i in 0..9 {
            let a = Fp::new(U256::from_be_hex(LLA[i]));
            let b = Fp::new(U256::from_u64(LLA64[i]));
            assert_eq!(div(b, b), Fp::ONE);
            assert_eq!(mul(b, div(a, b)), a);
            assert_eq!(mul(a, div(b, a)), b);
        }
    }

    #[test]
    //divassign
    fn divassigntest() {
        for i in 0..9 {
            let mut a = Fp::new(U256::from_be_hex(LLA[i])); //a
            let c = a;
            let b = Fp::new(U256::from_be_hex(LLB[i])); //b
            a /= b;
            assert_eq!(mul(a, b), c)
        }
    }
    //square root
    #[test]
    fn sqrttest() {
        //square root is not unique in a field
        for i in 0..50 {
            let a = Fp::new(U256::from_be_hex(SLLAB1_MUL[i]));
            let _aa = Fp::ZERO;
            let b = a.square(); //a*a
            let c = Fp::sqrt(b).unwrap();
            assert_eq!(b, c * c)
        }
    }

    //neg
    #[test]
    fn negtest() {
        for i in 0..9 {
            let a = Fp::new(U256::from_u64(LLA64[i]));
            assert_eq!(sub(Fp::ZERO, a), neg(a));
        }
    }

    //is_zero
    #[test]
    fn is_zero() {
        let a = Fp::is_zero(Fp::ZERO);
        assert_eq!(true, a);
        for i in 0..9 {
            let b = Fp::is_zero(Fp::new(U256::from_be_hex(LLB[i]))); //b
            assert_eq!(false, b);
        }
    }

    //exp
    #[test]
    fn exptest() {
        //exponent
        for i in 0..50 {
            // let a = Fp::new(U256::from_u8(128)); // a
            let inp = Fp::new(U256::from_be_hex(SLLA1[i]));
            // let a = Fp::new(MODULUS_1); //3
            let b = Fp::new(U256::from_u32(3));
            let bb = b.0.to_words();
            // let aa = a.0.to_words();
            let res1 = Fp::power_by(inp, &bb);
            assert_eq!(res1, Field::cube(inp));
        }
    }
    //is_odd
    #[test]
    fn f_is_odd() {
        let a = &Fp::ONE;
        let b = Fp::double(*a);
        let c = Fp::is_odd(b);
        assert_eq!(false, bool::from(c));
    }
    //root of unity
    #[test]
    fn getrootunity() {
        let _aa = Fp(TWO_ADIC_ROOT);
        for i in 0..194 {
            let res = PrimeField::get_root_of_unity(i); //some root /
            print!("{}\n", res); //2^ith root
            for j in 1..i {
                let pow = (Fp::new(U256::from_u32(j))).0.to_words();
                let ptp = (Fp::power_by(Fp::new(U256::from_u32(2)), &pow))
                    .0
                    .to_words(); // 2^ppow < i-1
                let check = Fp::power_by(res, &ptp); // res ^ pptp
                                                     // //////println!("check is {}", check);
                assert!(check != Fp::ONE, "wrong");
            }
        }
    }

    //Scalar testing
    #[test]
    fn saddtest() {
        for i in 0..50 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i])); // a
            let b = Scalar::new(U256::from_be_hex(SLLB[0])); // b
            let c = Scalar::new(U256::from_be_hex(SLLAB1_ADD[i])); //sum of a  + b
                                                                   // //////println!(" {}\n {}\n {} \n", a, b, c);
            assert_eq!(c, sadd(a, b))
        }
    }
    //double triple
    #[test]
    fn sdt() {
        for i in 0..9 {
            let a = Scalar::new(U256::from_be_hex(SLLAB1_ADD[i]));
            let b = sadd(a, a); //a+a
            assert_eq!(Scalar::double(a), b); // double
            assert_eq!(Scalar::triple(a), sadd(a, b)) //triple
        }
    }
    //subtraction
    #[test]
    fn ssubtest() {
        for i in 0..50 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            let res = Scalar::new(U256::from_be_hex(SLLAB1_SUB[i]));
            ////////println!(" {}\n {}\n {} \n", a, b, res);
            assert_eq!(res, ssub(a, b))
        }
    }
    //multiplication
    #[test]
    fn smultest() {
        for i in 0..50 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            let res = Scalar::new(U256::from_be_hex(SLLAB1_MUL[i]));
            //////println!("a:{}, b:{} \n res:{} \n", a, b, res);
            //////println!("{}", smul(a, b));
            assert_eq!(res, smul(a, b))
        }
    }
    //division
    #[test]
    fn ssdiv() {
        for i in 0..9 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            assert_eq!(sdiv(b, b), Scalar::ONE);
            assert_eq!(smul(b, sdiv(a, b)), a);
            assert_eq!(smul(a, sdiv(b, a)), b);
        }
    }
    //square
    #[test]
    fn ssquare() {
        for i in 0..9 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            let b = Scalar::new(U256::from_u8(1));
            assert_eq!(a.square(), smul(smul(a, b), a))
        }
    }
    //neg
    #[test]
    fn snegtest() {
        for i in 0..9 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            assert_eq!(ssub(Scalar::ZERO, a), sneg(a));
        }
    }
    //add assign
    #[test]
    fn saddassign() {
        for i in 0..9 {
            let mut a = Scalar::new(U256::from_be_hex(SLLA1[i])); //a
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            let c = Scalar::new(U256::from_be_hex(SLLAB1_ADD[i])); // a + b
            a += b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }
    //sub assign
    #[test]
    fn ssubassign() {
        for i in 0..9 {
            let mut a = Scalar::new(U256::from_be_hex(SLLA1[i])); //a
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            let c = Scalar::new(U256::from_be_hex(SLLAB1_SUB[i])); // a + b
            a -= b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }
    //mul assign
    #[test]
    fn smulassign() {
        for i in 0..9 {
            let mut a = Scalar::new(U256::from_be_hex(SLLA1[i])); //a
            let b = Scalar::new(U256::from_be_hex(SLLB[0]));
            let c = Scalar::new(U256::from_be_hex(SLLAB1_MUL[i])); // a + b
            a *= b;
            //a  = a+ b ?
            assert_eq!(a, c)
        }
    }

    #[test]
    fn scalarsqrt_check() {
        for _ in 0..1000 {
            let a = Scalar::random();
            let b = a.square();
            let c = b.sqrt().unwrap().square();
            assert_eq!(b, c);
        }
    }
    #[test]
    //divassign
    fn sdivassigntest() {
        for i in 0..9 {
            let mut a = Scalar::new(U256::from_be_hex(SLLA1[i])); //a
            let c = a;
            let b = Scalar::new(U256::from_be_hex(SLLB[0])); //b
            a /= b;
            assert_eq!(smul(a, b), c)
        }
    }
    //invert
    #[test]
    fn sinverse() {
        for i in 0..9 {
            let a = Scalar::new(U256::from_be_hex(SLLA1[i]));
            assert_eq!(smul(a, Scalar::invert(a).unwrap()), Scalar::ONE);
        }
    }
    //is_zero
    #[test]
    fn s_is_zero() {
        let a = Scalar::is_zero(Scalar::ZERO);
        assert_eq!(true, a);
        for i in 0..9 {
            let b = Scalar::is_zero(Scalar::new(U256::from_be_hex(SLLB[i]))); //b
            assert_eq!(false, b);
        }
    }
    //is odd
    #[test]
    fn s_is_odd() {
        let a = &Scalar::ONE;
        let b = Scalar::triple(*a);
        let c = Scalar::is_odd(b);
        assert_eq!(true, bool::from(c));
    }

    //exp
    #[test]
    fn sexptest() {
        for _ in 0..50 {
            let a = Scalar::new(U256::from_u8(0)); // a
                                                   //power
            let b = Scalar::new(U256::from_u8(3)); // 3
            let b2 = Scalar::new(U256::from_u8(4)); // 3
            // to words as [u64; 4]
            let bb = b.0.to_words();
            let bb2 = b2.0.to_words();
            assert_eq!(Scalar::power_by(a, &bb), smul(a, smul(a, a)));
            assert_eq!(Scalar::power_by(a, &bb2), smul(a, smul(a, smul(a, a))));
        }
    }


    #[test]
    fn random_fn_check() {
        for _ in 0..(1 << 10) {
            let a = Fp::random();
            let b = Scalar::random();
            if (a < Fp(MODULUS)) & (b < Scalar(SCALAR_MODULUS)) {
                println!("random is working fine")
            } else {
                panic!()
            }
        }
    }

    
    #[test]
    fn test_poseidon2_hash() {
        let mut v = Vec::new();
        for _ in 0..500 {
            v.push(Fp::random());
        }
        for i in 0..250 {
            let mut b = vec![v[i], v[i + 250]];
            <PoseidonHash<Fp>>::hash(&mut b);
        }
    }



    #[test]
    fn test_rescue_hash() {
        let mut v = Vec::new();
        for _ in 0..4 {
            v.push(Fp::random());
        }
        let b: Vec<Fp> = v.clone();
        let mut c: Vec<Fp> = v.clone();
        let _a = <Rescue<Fp>>::hash(&mut v);
        apply_sbox(&mut c);
        apply_inv_sbox(&mut c);
        apply_mds(&mut c);
        apply_inv_mds(& mut c);
        assert_eq!(b, c);
    }

}
