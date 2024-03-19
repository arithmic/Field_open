use crypto_bigint::Uint;
use traits::traits::{Field, PrimeField};

// Stores the matrix values 
pub fn rescue_mds<F:Field + PrimeField>() -> [F; 16] {
    [
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD28",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000438",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE7B",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000028",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8E19",
        )),
        F::from(Uint::from_be_hex(
            "000000000000000000000000000000000000000000000000000000000000A5E7",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC749",
        )),
        F::from(Uint::from_be_hex(
            "00000000000000000000000000000000000000000000000000000000000004BA",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF28A57",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000137EC8",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF9728C",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000008458",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE872169",
        )),
        F::from(Uint::from_be_hex(
            "000000000000000000000000000000000000000000000000000000000220DD96",
        )),
        F::from(Uint::from_be_hex(
            "0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF49E0B9",
        )),
        F::from(Uint::from_be_hex(
            "00000000000000000000000000000000000000000000000000000000000E204B",
        )),
    ]
}

// Stores the inverse matrix values 
pub fn rescue_inv_mds<F:Field + PrimeField>() -> [F; 16] {
    [
        F::from(Uint::from_be_hex(
            "02a06782796baad98f3d16c2e09b45af3d51f4232e9ecba10a2a5ec3c47f52cf",
        )),
        F::from(Uint::from_be_hex(
            "009514dcbb7160dbb940a044ade7393b03dd91892d34c3342938a37fba9ec402",
        )),
        F::from(Uint::from_be_hex(
            "0324ee73436072b72560b149f8ebc561507367ab2c74c2fc54844607a629f421",
        )),
        F::from(Uint::from_be_hex(
            "01a5952d87c281a4922197ae7891bbb46e5d12a877b7ae2e7818b7b4dab7f510",
        )),
        F::from(Uint::from_be_hex(
            "077a3158671cdc62de57102aa9007b31b8f5de4315f0fdac019cddfd2a272607",
        )),
        F::from(Uint::from_be_hex(
            "052dbf9735fe8a640cf4fecf876b26e0d5f8aadc3d859fabb27161be7c912d71",
        )),
        F::from(Uint::from_be_hex(
            "0653d17fe91fe5861a138c78ffe3445ae21324e2cb6168693390c5fa865f6df3",
        )),
        F::from(Uint::from_be_hex(
            "05043d9079c4b3e5faa0648ccfb119928efe51fde127fa3f1860fa49d2e83e99",
        )),
        F::from(Uint::from_be_hex(
            "06ecaf953edbd4834d41ab0490ae2da6cdc88409d528625393d743c668a5c019",
        )),
        F::from(Uint::from_be_hex(
            "015deada1cf3a19632ff4236dc245d7cf9dfc550f698b7dadabcc56eedef38d9",
        )),
        F::from(Uint::from_be_hex(
            "00b7f57db4548a3c3c9bd04b1f9e319afe87c2173a9e638c8eb415472ec1d14f",
        )),
        F::from(Uint::from_be_hex(
            "06fd7012efdbffcc43234279738f434139cff48df9a0824502b7e1837aa935c2",
        )),
        F::from(Uint::from_be_hex(
            "004bda12f684bda1d097b425ed097b425ed097b425ed097b425ed097b425ed0b",
        )),
        F::from(Uint::from_be_hex(
            "041d7f7926fabb8e8a021b641511e8d2b3183afef24df5770b96a673e28086d9",
        )),
        F::from(Uint::from_be_hex(
            "03452e00b3cc070ceb47fd30cfe3e81ee7113506ac1242b8b69b3722102754a2",
        )),
        F::from(Uint::from_be_hex(
            "005178732eb47fd3ba1e33452e00b3cc0705f8463bb2be54fb6f51d25932377c",
        )),
    ]
}


// Stores the constant values
pub fn rescue_ark<F:Field + PrimeField>() -> [Vec<F>; 16] {
    [
        [
            F::from(Uint::from_be_hex(
                "054fb2a3b689b206980a7b228ebfdc888a207f4b58155e0e899a84c940fb2634",
            )),
            F::from(Uint::from_be_hex(
                "031e11ba6d43c5f019c1eb413deda4e4a182cef569be6f6fe8c498da0b5be469",
            )),
            F::from(Uint::from_be_hex(
                "021de486a2c651bc46b06e76b62e480518e4631bb7343fa9d49ab04ec3990c13",
            )),
            F::from(Uint::from_be_hex(
                "078cb86577702996da98e264a50f23a8b76a004d9cc6470a98805207e0ef91d8",
            )),
            F::from(Uint::from_be_hex(
                "01680b9531d823562df0dc361478fdd5749152f162586a4d870498b27805ad30",
            )),
            F::from(Uint::from_be_hex(
                "0705a4cbf53614137e5f79202bacd5fab61bf70755880411cb77de2bdfa8db7b",
            )),
            F::from(Uint::from_be_hex(
                "027da53b6ca41f47c3cfaab72c639690dbff60e24c55bda327b6f9064a1cd41c",
            )),
            F::from(Uint::from_be_hex(
                "060c3fffe1aa026aeb1f88b8841e1fb8abc5b9d783ac18945fc45682ea2a8457",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "03a240371af4eeca7a33c1cd04067f6c2c4159a9fc628107ab1ef1a4f54e27c6",
            )),
            F::from(Uint::from_be_hex(
                "079fbb8bbdd8e52d112985dbf4049109cff212f1940990293c2b4d5a04da2f01",
            )),
            F::from(Uint::from_be_hex(
                "037dcffaa5063199df8ef3123be5a56c83b73fcbdeadde9f8c9143b6dd9b945f",
            )),
            F::from(Uint::from_be_hex(
                "059c3bfdf41db508a4210c150b45b9601372c506ac820d243f98cfc7162c4c0b",
            )),
            F::from(Uint::from_be_hex(
                "075308ac17c0b44d8dcaa02cf5a991fa11576c82d8cb6023438ad701a024dc70",
            )),
            F::from(Uint::from_be_hex(
                "01ca245201c4ecbd0348c546200ae51d636f2705e08917077a885d32a8ae2046",
            )),
            F::from(Uint::from_be_hex(
                "048a90d6b3bcbc41293e8b7f8296027d761e80edc55bd45c7800a630441e15e1",
            )),
            F::from(Uint::from_be_hex(
                "02962944511907c23223c6c0b94235723ccd511dc178cc2e0388d85c0924c02a",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "02cbaead7f61454ec5b7d117e6e1c4b3786af1c740c5dec1349dd81fde390cd4",
            )),
            F::from(Uint::from_be_hex(
                "0748ea35c159451949d31a4cfa6579bc84cdeded94b9a482aa02b67ca6a5d6fd",
            )),
            F::from(Uint::from_be_hex(
                "06e0b6accc488ad2a76113ee7c5ebc97c57167b0fe7cbd176db10aa33da59da9",
            )),
            F::from(Uint::from_be_hex(
                "05a4e9afe6d35e96c35a0d41a804788f3bda0c4186a3b48ce3e9e15e8be847e9",
            )),
            F::from(Uint::from_be_hex(
                "0211c16eb229c9e7b37d20d08ad5d92474f394eb3c16d02b30f9de916fcc0fdb",
            )),
            F::from(Uint::from_be_hex(
                "06ac9889ce1987e7ddb7b1b621a56ec4d792f93146c21d3c1cd27ba8f271b878",
            )),
            F::from(Uint::from_be_hex(
                "0324465fed969eb777788a041f74213830d892fe50fc0ad66ffff165fd57e3eb",
            )),
            F::from(Uint::from_be_hex(
                "0260912268e9c70b8c6a335635d558115d9e623bb6f7ed4cca3e1b6214106458",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "007610a48c7f095938504690bc701d6e0b2a7a866dbcd7e164d567116ccf4a23",
            )),
            F::from(Uint::from_be_hex(
                "074cde35fc541f8b3fc0a130026f01b565fe00af124c9c269cd7300d03ada5f0",
            )),
            F::from(Uint::from_be_hex(
                "05d5d685c94f2e6ea30e3f1e09a6813ff57b9881ea4fc7f642bac40ab2865a63",
            )),
            F::from(Uint::from_be_hex(
                "06d1097f8c31ab7213616811e99f66fb0a318e8aa9c7dd52c5410b63e029e250",
            )),
            F::from(Uint::from_be_hex(
                "034e8095bdf5b4218c0eca5ac36fce558aaca730d1f1b1ddaa53c7e7408d6e3f",
            )),
            F::from(Uint::from_be_hex(
                "0634d9edbacb06e85c951a02e3fcc050373486096ad8a664f412d747ed7f65de",
            )),
            F::from(Uint::from_be_hex(
                "03eb573b894ecc2b941960c3bd84530d34d90860b93c88c1ca3e4f7c436add15",
            )),
            F::from(Uint::from_be_hex(
                "07f604d76c50cc2ca2294caa1a1cf71f585f75e5075865e1858a3f6764bc81c6",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "00da13a384b5c93be90a343fe496b01a20f8c11536238fbc3b9c087207ea246f",
            )),
            F::from(Uint::from_be_hex(
                "054de597a9cf1eb0bdfaca7f024e2d95939525f6367ecd8f79c6c3171aa08953",
            )),
            F::from(Uint::from_be_hex(
                "021a5933170d53921684cec481b1d674d39a112b07e138cbd7fb26adfa5ed2f4",
            )),
            F::from(Uint::from_be_hex(
                "01df4146b699aea2e0952f2f4bb500b89d9ee59f52cf57e476d7eaa4c180a028",
            )),
            F::from(Uint::from_be_hex(
                "004ddcd9c014e68311ef404862388bf19d38404f0e3a0e1bf584fed7dff0e8b3",
            )),
            F::from(Uint::from_be_hex(
                "06d5c3adcad3140d5821473068a573ff72109a8a2854716aa5754a1e00c2f1f4",
            )),
            F::from(Uint::from_be_hex(
                "06401756a5c738687d370e3835dd8c8035bb415673f9e2e9f058c2463090c50b",
            )),
            F::from(Uint::from_be_hex(
                "0300f52b0e399fc1a2c642d02aac55a60f8a9d0d5baeb44802bd3321581792a3",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "0207161f85e0dcbeecb453f89bb2b2066e753408f7c73d223426c243082a2519",
            )),
            F::from(Uint::from_be_hex(
                "05629015ccdf2f13b4b55c12710d32ae340a4d25093cc956208a144a24934bb7",
            )),
            F::from(Uint::from_be_hex(
                "045f5a9cc524500b295d64b626a40fb1b4154833b8ff3281720ec47d24396213",
            )),
            F::from(Uint::from_be_hex(
                "007f9f7249d4a415117433e39249b4cacdec9156339adae5f5bf1d683ed4d0e7",
            )),
            F::from(Uint::from_be_hex(
                "078735fc1c0c83c09f6e2278b1e5b48422d09b2985eb33244836b60299a1aa2d",
            )),
            F::from(Uint::from_be_hex(
                "0530f7517b8255cb221e066546ae6116fecce4822f64a70964115a4c1d8c351f",
            )),
            F::from(Uint::from_be_hex(
                "0253f91d09f870721fe487ee153845e2e2190f63fd5365cd27d349caac9c63ba",
            )),
            F::from(Uint::from_be_hex(
                "04ed4491041c85344a33a8e1cdd5204b49d428e0339411189827f319cf176cfe",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "027bac9e6f3629bca87e8a786ec9c463617ac36425b73bbadaa02c0bdadbf040",
            )),
            F::from(Uint::from_be_hex(
                "01dda192d6d03c3bdb0ef25a9e5587300fb41f4eaf30a48624db2cd5276820af",
            )),
            F::from(Uint::from_be_hex(
                "04f5a937032513944465d1dfbfb48bca3a0db593fc4581f0fe4b6139a1c58960",
            )),
            F::from(Uint::from_be_hex(
                "0264e0cb1439ab2180f41d12c9c7ed22c52256609523843be2dad583b7704d3f",
            )),
            F::from(Uint::from_be_hex(
                "04141f70feabaa4aaf15825b08abe0e3f261086a1b6591caedabb6693ead6559",
            )),
            F::from(Uint::from_be_hex(
                "0302bf7e20486837d52568bd136d89819cc8e6e6ea70ccb6ca5eb3864f3cd637",
            )),
            F::from(Uint::from_be_hex(
                "03481e1f65ebee391f1d46a2da234d26622ff96a971a00a967cb3953202e4e28",
            )),
            F::from(Uint::from_be_hex(
                "0774977fbbd56106481b845d2d245a6b37abe9cc3902038efa5bf2284bea8967",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "01df2f3d0ff181c637c1fb921fbd1a6d8d091e71a588201810881659cfbf721f",
            )),
            F::from(Uint::from_be_hex(
                "058559f59852aba7e81fd587c3d3b95fcdf78ed81c8744c013d88929b847bfa2",
            )),
            F::from(Uint::from_be_hex(
                "043f73d886df95b49fb459f577effc064328bb5f103167c72d34874020d02d8d",
            )),
            F::from(Uint::from_be_hex(
                "00f1e2b1e45bb133489a843ddc71958542f9d9d0c43ca2d65f181b8cc8f1b103",
            )),
            F::from(Uint::from_be_hex(
                "06758a3773ad5e1fa83797caaea9888b4814a102ed74da5d97cca6867ca65cf3",
            )),
            F::from(Uint::from_be_hex(
                "06e44b302d93a5ca6d1271cf887a329ee2ef710f4e8ca7c5c49d2db4ac8a6236",
            )),
            F::from(Uint::from_be_hex(
                "02c786203901c72f38465c6b78f73167ca3f8fafd3baeaaab0cce04dff48ece1",
            )),
            F::from(Uint::from_be_hex(
                "038e8d4fa1c807c20ab81a5474ac3062e60abda5efffa370cb7e2aae0fa7257a",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "07d9643279f35a27ec3b51eb6d1f1f2f007c1dab731e9f03ad02d8cd1b6938ab",
            )),
            F::from(Uint::from_be_hex(
                "07ec971d164be16045b3f46eb7deae1fc8541accd9d0eda0c9d1e317e17b674c",
            )),
            F::from(Uint::from_be_hex(
                "05ea0526b03aaa78903eeb58f8e196ec0e9d15ea98edf837cf6eac617f50d7a9",
            )),
            F::from(Uint::from_be_hex(
                "01485dc7f740d17d8f9767395fd06ed60e455a621159fe21fe9012166ddf6c48",
            )),
            F::from(Uint::from_be_hex(
                "0621d54a57807ad59e8c83b13c1eccbfc9c6598743bbbdb041c1a861f39ca107",
            )),
            F::from(Uint::from_be_hex(
                "01b2432de6400cd0ad04dbf55e4d2ea23d45d8b953dff364e4f8fa1a9ea956b7",
            )),
            F::from(Uint::from_be_hex(
                "00c2b135d93cfe3841076eb83d67ccf67b3774242f4a3f11a5930fa2d233ff29",
            )),
            F::from(Uint::from_be_hex(
                "00765f7b2df82f2ad25093189e5ab0d699deee1d3920e07b56f14face6f1835b",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "060d9376b407ea0dca51c02ce0084bff335cca83d86d87efd389b4a49006dd9e",
            )),
            F::from(Uint::from_be_hex(
                "0294831ada7a18570664d45a15101e139c4f6df02aca1031c33e916efe69887a",
            )),
            F::from(Uint::from_be_hex(
                "0180d0ed2ec99ae25976063a4e3353b8f1571e4692a8b2232738e84a13650bbd",
            )),
            F::from(Uint::from_be_hex(
                "07ed24411979f0fd119b491848d921104dcb1e324ece42736b122020dc7a1f21",
            )),
            F::from(Uint::from_be_hex(
                "0537025c27fa4148c08913775bc8d0097c4017c2b5d8b9b897bb3db3e09cfba5",
            )),
            F::from(Uint::from_be_hex(
                "014ebb76e5cd33de359eadb0f1f9882735986c24e103daabb2e432f4e4caf75e",
            )),
            F::from(Uint::from_be_hex(
                "05d18eb1d29f91a648a47be81b0e8f5a27d14eb8a535e6853a66f22b7fc58530",
            )),
            F::from(Uint::from_be_hex(
                "01c53fda0ca5e543164db3545d46ab723ab29ef9626844d29a23704221d6379d",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "05c6d8c0700c69f4517ddb9a458ee07243903c865aa53129294e05ec34f29fd9",
            )),
            F::from(Uint::from_be_hex(
                "04d27afc7aa71e16848d3168d5593f99c0f93cc94b90fdf4b0f788a20f52f09f",
            )),
            F::from(Uint::from_be_hex(
                "077261f8d1d0657e683a56526e5cdc0ca3a93c72839253160ce8bd86a398ddfa",
            )),
            F::from(Uint::from_be_hex(
                "06cb28bce4d77bbd39ff7a5c5c02d27a7e35746b84a06c2355a2c5525b2b6258",
            )),
            F::from(Uint::from_be_hex(
                "055299e8228ffe0e3d64ea4f881ea0c18deea2fd9373739b703c549602f79e9e",
            )),
            F::from(Uint::from_be_hex(
                "06dfe4b85cce67e870beb2d6f84127a72a2a609ea8fb33f72434722d122448cb",
            )),
            F::from(Uint::from_be_hex(
                "03df23161d6864c9ae4a7dbafced80fdf86611b4e3b3adf559b863d0b1318fd7",
            )),
            F::from(Uint::from_be_hex(
                "057773c229bd2ca5fb5970ce79b4d6bf884b59049e19f3241cddba14e41f6ea4",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "01fc64137517bf1c5dc132adbdf3ddd0341e6f14428f0fc1cfabbec1030ee3d9",
            )),
            F::from(Uint::from_be_hex(
                "0093c903c5dd7f78fb8b3ff50d75c2f0a415cac1ddf58635eadf24d18261fc32",
            )),
            F::from(Uint::from_be_hex(
                "004fd946f7bc598acf6dde75507fb801f09780b969425c44547d0fc9d6d90d95",
            )),
            F::from(Uint::from_be_hex(
                "04170d8219a88da7eaffa9253e3eb1479c6a67e2479c36cdf9a548971cfd6d46",
            )),
            F::from(Uint::from_be_hex(
                "01bcf4ea30d54ce714dc5dba100e278c34b5711131207c6b0680a471483254d6",
            )),
            F::from(Uint::from_be_hex(
                "00777f98789b7f015a32ad94a2674c2e9e86883d304db6ed09b3079d3d70e4d5",
            )),
            F::from(Uint::from_be_hex(
                "0211a79f9c9c3697e3dd031f4e6366d60830259f9d824ce9c6729aad786d6e02",
            )),
            F::from(Uint::from_be_hex(
                "0346022dd291d533373893b459e48fb563f5b1803fb6595908ae40af99e96219",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "07793ceba1a7993c7608b958ec400302f579f8a071b6956751c6d9a525bbbb0c",
            )),
            F::from(Uint::from_be_hex(
                "06e5ae57f844cc3cb1ab70106fb9b3681d51e90b570da3585cb270d72b47e935",
            )),
            F::from(Uint::from_be_hex(
                "043485a515ec950775e855ef81cd87e352c28cf8f94c427cf01d6c0da8a669e7",
            )),
            F::from(Uint::from_be_hex(
                "02cf0b050ad6540b078f65abb70fa67b66a2d04cad32557d7b8ba0ba4afd3dc9",
            )),
            F::from(Uint::from_be_hex(
                "05264a33fb024b2220b02bc9fbe477b64de14b74ab3e5c926f349a0dbbb0f475",
            )),
            F::from(Uint::from_be_hex(
                "04c8e10a3baa746566ae8c68e54351f73e25bfe8375f14af4f1a5ef0be2e848e",
            )),
            F::from(Uint::from_be_hex(
                "07d45d5cd27621237cc5c384e447607de7d1e2aab32c27b7bba92c99f94ea9e4",
            )),
            F::from(Uint::from_be_hex(
                "01be6c5c992f43ec372a8280a8ff7701f8138c4e3e11ffb42de5555f410aaa58",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "0053331c7b9f408b8af5f81d2a752335391b13ce800bb2be0bc76a4af18a2dba",
            )),
            F::from(Uint::from_be_hex(
                "05707586f121301d635a771905c850568309e44e6bc5544cdf97850c2c6b8e3c",
            )),
            F::from(Uint::from_be_hex(
                "00e0057a3f6faa5552cf537e3a3df85bba7ebadc4192ed6a8a1a8a20e2d37124",
            )),
            F::from(Uint::from_be_hex(
                "0574b99565f4e432166b2fa1d578eda8c89b85b089d11d39d8fb6c1772be5c44",
            )),
            F::from(Uint::from_be_hex(
                "0472d15bcdb99eb9f81ba91c74ad13c3aa0534ac4f202ec1177da74c4a2c9c7e",
            )),
            F::from(Uint::from_be_hex(
                "0603c8b46894e780c8d5c2c9fbc9aeae5a8faaaf1f8f40d8bc1f851f33e06962",
            )),
            F::from(Uint::from_be_hex(
                "02db5679f3452f8b37f2cd24ee12a2cbdb90b3f46f2669fd943b3bedacc84f38",
            )),
            F::from(Uint::from_be_hex(
                "063d0a69b98376d31509955d5f21b33a1473a6f8944769d694edf5d6e722ef66",
            )),
        ]
        .to_vec(),
        [
            F::from(Uint::from_be_hex(
                "0225835d6be6bcf5ff417fe98020d0b774d7bbd4563fde76e42fd8322bfb8601",
            )),
            F::from(Uint::from_be_hex(
                "0579f01966a6d1079ddd2a42c1739c55907d3e6999e5ce9ee81c41cdd7b85517",
            )),
            F::from(Uint::from_be_hex(
                "0364484a8b2ff1218c1cde0f3b2a1918935a0ab2782fea66c7891290f0e724a0",
            )),
            F::from(Uint::from_be_hex(
                "063494e8fbfded84fbc3b8ac3780f29cc859e109e466ea052bab9043c88ba536",
            )),
            F::from(Uint::from_be_hex(
                "0771a5b2e10e6e712f447c3b75d2d7c330778719cb423812855005a1b6b00c72",
            )),
            F::from(Uint::from_be_hex(
                "0267e10662023614a40bd47c05a3b37602288812d4490db53837528a6cbdcf21",
            )),
            F::from(Uint::from_be_hex(
                "032649aa3c6647157ebe06ee24cdfab38451e0c1184c2172686b596f2909500b",
            )),
            F::from(Uint::from_be_hex(
                "0667cbf3ca5885c540720cb1729799f7128a658d77fdea58569a428c6464f81c",
            )),
        ]
        .to_vec(),
        [F::ZERO; 8].to_vec(),
    ]
}


