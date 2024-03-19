use crypto_bigint::{Limb, Uint, U256};
use traits::traits::{Field, PrimeField};

// Stores the MDS matrix for state_width = 4
pub fn mds_4_2<F: Field + PrimeField>() -> [F; 16] {
    [
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000005",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000007",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000003",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000004",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000006",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000003",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000005",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000007",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000004",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000006",
        )),
    ]
}

// Stores the MDS matrix for state_width = 6
pub fn mds_6_5<F: Field + PrimeField>() -> [F; 36] {
    [
        F::from(U256::from_be_hex(
            "117d9115c7741d2d021fccfc15f692cc5bb4ba317f6399c949d484ef2a339e7a",
        )),
        F::from(U256::from_be_hex(
            "046ba516add26bb63ab2fcedcaaa1265ec8b784b4f7d56e148116ef398c9405d",
        )),
        F::from(U256::from_be_hex(
            "129b9d6b2225611bbc1ab78b4935135f63d713d495a9d3f950a21f5a164386a7",
        )),
        F::from(U256::from_be_hex(
            "0e3f5e309be5755cca7257a0010ca162e70795df2fe6fed61302ea56e350a540",
        )),
        F::from(U256::from_be_hex(
            "05a9f8a99018aff8207491cf395f8d1729121e64e6d7670673e6d19ded789a65",
        )),
        F::from(U256::from_be_hex(
            "204d6161372a70453afb590b3d207ef773eace3a7867e323f8006821d3569258",
        )),
        F::from(U256::from_be_hex(
            "23841b8143d6ccfcebb0ee8dd93ebd486cfa3b7792115ba85838624fa31553c1",
        )),
        F::from(U256::from_be_hex(
            "14d6ce0e76d457e9e99443c35ed7ba71947fba67e54da8bc20d678d1cc677a4e",
        )),
        F::from(U256::from_be_hex(
            "299083c123f3166d3b55494064695afe2181d42cbbbcb26d4ca9b35912a3ec36",
        )),
        F::from(U256::from_be_hex(
            "13eb33a1e62858fc3a48ca168337d279d4b4a1497e536c5468925f0f0ad2dde7",
        )),
        F::from(U256::from_be_hex(
            "0b3e07c0ab744e59bfd895f8800a9c5e86919550d8a9da8a37b1afd433def792",
        )),
        F::from(U256::from_be_hex(
            "002c316dfa9dc1062a893b2f098ff2ca9baab5078fee7d3d97d61e4940b8dacf",
        )),
        F::from(U256::from_be_hex(
            "22c65c62e9598a810bd0cd74ef48ec10e1a792d561c0353954575dc0491b2ebd",
        )),
        F::from(U256::from_be_hex(
            "033204ca26e66dd16fc47e394c721292505a34417e2ee6d2c6ca35127fb6810a",
        )),
        F::from(U256::from_be_hex(
            "067bdf5382f49144d184c9b6e46f19903895f710c4962d1f02af6ca9b5acf1ef",
        )),
        F::from(U256::from_be_hex(
            "1eaeb224930226340b346a3417a131e2e19e7ff348a38444f18c98748f30fb62",
        )),
        F::from(U256::from_be_hex(
            "0311fca4eda9bfc70642ad13c667330bd1bec6c9af43f0bae72223f66c77fb58",
        )),
        F::from(U256::from_be_hex(
            "1820dbeecdbe52fb1fcdfc3effe49aa53dfcd7ec416703d57f76437d6f7d4d66",
        )),
        F::from(U256::from_be_hex(
            "07484e1ef55358525033af02dd743d43419dafbcf228d7cb41516d1fbcaf2bca",
        )),
        F::from(U256::from_be_hex(
            "2a957064b5ab2d475c89bc09fffeaa75c5cdad7f24485137352ede29ba9bd933",
        )),
        F::from(U256::from_be_hex(
            "2be7949c73efe69812c039c99539039a46355ba5ea85b0a6f03fd515c3d330e8",
        )),
        F::from(U256::from_be_hex(
            "21598336c19ca42d300dbbb4ad3b352acc13de2645d32c809b6711c5c8dc8207",
        )),
        F::from(U256::from_be_hex(
            "2afce53f75498f1868f91f5e8c8d70a2113ba4486ad6e1ec728863b733c467cb",
        )),
        F::from(U256::from_be_hex(
            "176281bebdd85236c5b31c38cacd59dca5fd3a74f0b01260639e35d08d24d265",
        )),
        F::from(U256::from_be_hex(
            "18b99f8a4b597983914ab013b973d137bd97e7f3356fbec1e1a60189fa2e6990",
        )),
        F::from(U256::from_be_hex(
            "0b0bc9fd2d0eda937c1b0bbc424d7beb17bd7fe9fceb22ca5d473a601017f501",
        )),
        F::from(U256::from_be_hex(
            "11eba42f302379419bc22fb860d60353666fc50fdef48547b54c11e74b7a2dab",
        )),
        F::from(U256::from_be_hex(
            "0d8d13397dbe6ea1cd884c1fdc2e11fdc0f25d755ed348a30c1129df4674bb12",
        )),
        F::from(U256::from_be_hex(
            "26e5d6aab15dc5e9d8a9b075ecddf6a8d8e6af63055a79527dbfda672d77dadd",
        )),
        F::from(U256::from_be_hex(
            "2a87ae8b8a9816512befb2be7c893111af7d7e5186e9fa3590a824bb39b851fc",
        )),
        F::from(U256::from_be_hex(
            "13888de784f859f806b3003722b3188e3b24ae9bc19adc6667741d20abdd876b",
        )),
        F::from(U256::from_be_hex(
            "2da3d93612ccbd695dbdad4bc2f755ffa38970959e692a7f540b3bde48379fa2",
        )),
        F::from(U256::from_be_hex(
            "059f5fcb1c5b321352ed60bf2400654f6fd5ba1aa7c14a0a82cf55d3d8f7f6ee",
        )),
        F::from(U256::from_be_hex(
            "2d3d36ac377ea13d4f08607a1d8d11ecb58ae113531f3c23e2b0d5141741dcc2",
        )),
        F::from(U256::from_be_hex(
            "078e7ddeba76eba503bc8e637ffaa7c8d10b03c15f76c5867d121ab99c381b7e",
        )),
        F::from(U256::from_be_hex(
            "2ee1ae099f60856085390bde27b6a943ceb42aa9f8a877e2124d296f43117a8a",
        )),
    ]
}

// Stores the round constants values for state_width = 4
pub fn round_constants_4_2<F: Field + PrimeField>() -> [F; 256] {
    [
        F::from(Uint {
            limbs: [
                Limb(4700749314399699658),
                Limb(15034221993265643419),
                Limb(14619367847521324155),
                Limb(1387104776476235830),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1845582398095254546),
                Limb(15390392379446261187),
                Limb(1819674602729007382),
                Limb(2292575931121391835),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(18353750466448752018),
                Limb(4814703311722665995),
                Limb(16152908511596570332),
                Limb(1320243768398897604),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1950631696565260189),
                Limb(13117875725226720953),
                Limb(4006057144302780489),
                Limb(845355991074736886),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(5174306835288044203),
                Limb(14041525537726201217),
                Limb(10120363909408740765),
                Limb(498886867524949500),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16056336923087003947),
                Limb(10523225488199756368),
                Limb(14503433497276216329),
                Limb(955072062660547154),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1373248757756692095),
                Limb(16132391599418665791),
                Limb(11316507201501179003),
                Limb(1541156750231834936),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17406592954518659233),
                Limb(3171361296224125049),
                Limb(10558242680072744777),
                Limb(1843438642421557078),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16650554439172953299),
                Limb(11697636397036515783),
                Limb(9337139082955813831),
                Limb(1487410867624499227),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1043546542737490966),
                Limb(32598506571974569),
                Limb(9317634860756757018),
                Limb(1484045210455080338),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15644642076389809308),
                Limb(8431596039500329969),
                Limb(10304332112960483186),
                Limb(494652035138686806),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(5745493142084407891),
                Limb(16186056426879459788),
                Limb(3091714509295116121),
                Limb(2564358396445636431),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6635759749186892153),
                Limb(132490766926371757),
                Limb(825180000838891949),
                Limb(1586694822950440634),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(875211881788771784),
                Limb(6710613325228642811),
                Limb(194752461942371235),
                Limb(1368543431647974002),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4298344487848862221),
                Limb(6873350871680237861),
                Limb(8322156443530592241),
                Limb(575962969111960308),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8243511553479086711),
                Limb(16102652522487473206),
                Limb(11892311205078425868),
                Limb(1715313294250414707),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17742962098261600996),
                Limb(3423889873320365041),
                Limb(9839244039653528876),
                Limb(1375058768621482419),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(15786296834273596025),
                Limb(12807582735515150975),
                Limb(15917140657671459806),
                Limb(1899524323762304130),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5216552811923614699),
                Limb(7927004399026928538),
                Limb(8878198385435347151),
                Limb(2185482335844647516),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(11336389861212614722),
                Limb(5073461298733389586),
                Limb(10066225174091757108),
                Limb(1085134630867034002),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5886486671446127176),
                Limb(5903870957285104858),
                Limb(15748792128305894399),
                Limb(779997208684266036),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(2266558252004097143),
                Limb(8931888022558347578),
                Limb(5504068064223662972),
                Limb(2279862424064560383),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17357996494148959409),
                Limb(6477715802228853916),
                Limb(17723985928740291543),
                Limb(474374816546099734),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17509504020453313894),
                Limb(7090639917768292226),
                Limb(13497885033703844351),
                Limb(660424399790188372),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3093695441211075289),
                Limb(2609790591720169954),
                Limb(18074316510315794569),
                Limb(2173399072792262616),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(13134374861649604544),
                Limb(95685005938898166),
                Limb(7661151697488447882),
                Limb(1698201393068054399),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(13271925814408252142),
                Limb(4587187039548152093),
                Limb(15497329450218761697),
                Limb(1747035545965906325),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16854030146339090918),
                Limb(7968894525675238648),
                Limb(1996664460336796660),
                Limb(765809621606456986),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1150294778674962640),
                Limb(409334337681133356),
                Limb(4258246755069306018),
                Limb(3169084532656725617),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(12386391465122826441),
                Limb(12323038770856581849),
                Limb(4555143962678714782),
                Limb(2989347098316135171),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3699059934632082571),
                Limb(17603995674273324374),
                Limb(12164282198838749627),
                Limb(1267554130195944132),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(18087012482776846117),
                Limb(1364522854995533192),
                Limb(1730235288988343389),
                Limb(561390858750696332),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9196568810193431728),
                Limb(4038673722625976185),
                Limb(6228421679345238503),
                Limb(1842573689160456904),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(12733735340708858903),
                Limb(17738465654230849854),
                Limb(4516975884531535361),
                Limb(1530277246167707573),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16227836969602385695),
                Limb(8242471728597896009),
                Limb(11784631829925454882),
                Limb(1022412563702955761),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(11884815535329975399),
                Limb(10772239202764115646),
                Limb(11483242791613591496),
                Limb(1493761602728534617),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14731763364572860599),
                Limb(8584910170496344943),
                Limb(2511762275965343094),
                Limb(2274031720838161179),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9204350487418490322),
                Limb(4995165035550711575),
                Limb(10120096734602930863),
                Limb(576585340923303728),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(4247186541489121412),
                Limb(15158622277878239882),
                Limb(3418845727822138582),
                Limb(2291136502298392240),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3689215974592860243),
                Limb(6121486601476052397),
                Limb(2837858978175125110),
                Limb(737062864106689995),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(13518398234321251376),
                Limb(18421247278889190671),
                Limb(11798757515735752373),
                Limb(1644379106539898894),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14621502034464785607),
                Limb(14524275399526287585),
                Limb(5185151593069075860),
                Limb(1108974070697078927),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(12747531450621853817),
                Limb(1756515342881229663),
                Limb(7192794427903223112),
                Limb(1802395655688281918),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(806536301605333793),
                Limb(1738823022016509600),
                Limb(16053706877035771318),
                Limb(1443603974199449460),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5094756638587804819),
                Limb(2542744689063765169),
                Limb(7280870788883557222),
                Limb(2253619559858864495),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8048880854053592628),
                Limb(17776710182784933406),
                Limb(17959584808072908183),
                Limb(971942374775967359),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8824773806980906467),
                Limb(7174928769602049777),
                Limb(4910785342066645200),
                Limb(1607034475338688231),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(15711618782684820697),
                Limb(2640248794103415755),
                Limb(7507076040967426467),
                Limb(2064019838835191187),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9368096313476631762),
                Limb(5777170329780098158),
                Limb(9754283664692844795),
                Limb(503504667756127716),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(314230853110590833),
                Limb(14647842701922147440),
                Limb(2183731412797985519),
                Limb(2016561240500842085),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(15712395899137957932),
                Limb(8457276949432124856),
                Limb(17955860841517362520),
                Limb(2255647448137272138),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8418841257399622541),
                Limb(14201932275402513636),
                Limb(147367843568435043),
                Limb(931588705417804522),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17041303122856597446),
                Limb(6318912965638134661),
                Limb(9238711259527084547),
                Limb(3240574731169020185),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3713194293891290579),
                Limb(17386891938847690844),
                Limb(15897597517993542218),
                Limb(503534049591952532),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(7023277794876528144),
                Limb(14642542462152488107),
                Limb(17352821919879846085),
                Limb(1620760622539724782),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6315281289045400607),
                Limb(11821850563660271846),
                Limb(16882180097844215257),
                Limb(866614037577210168),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5062006803471855901),
                Limb(14779853736579299263),
                Limb(9665729758164747960),
                Limb(736470994636233662),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14512433889065848744),
                Limb(15003276895048976149),
                Limb(5127611384333493400),
                Limb(2182967404797452166),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(15883642561901123277),
                Limb(8709596505172338400),
                Limb(17492676357267648348),
                Limb(2099523530710616831),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(11115039082762761915),
                Limb(3697196511849680716),
                Limb(3052527003034076833),
                Limb(1538967966309687108),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3478176366981862875),
                Limb(9607732494927751910),
                Limb(4765798667099924926),
                Limb(2090078794504980263),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(7680588431982087928),
                Limb(14717978919775067495),
                Limb(11145021688575185956),
                Limb(741566105655464035),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(2619439004817008640),
                Limb(8881312518317897479),
                Limb(4812951275799588194),
                Limb(1849310540209640158),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3959137756647069463),
                Limb(11606111925294008850),
                Limb(859388283438806073),
                Limb(1336999363763918812),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3019962408515908085),
                Limb(9375274057890165135),
                Limb(10424000568194398112),
                Limb(1446096592472843575),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6076793991040094047),
                Limb(7383130272634120260),
                Limb(10199535704410988870),
                Limb(1193915764203381598),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14304622460304950714),
                Limb(14750864180274860745),
                Limb(15350853509827427808),
                Limb(1670039456896844138),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6005421538642062444),
                Limb(10098671739814104434),
                Limb(5080162477473623556),
                Limb(1122499513291202204),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6801098651083752583),
                Limb(14288196792973429604),
                Limb(15620145120180267952),
                Limb(753299089417992340),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(13946177414665921781),
                Limb(6151970601912908066),
                Limb(14313270898681350737),
                Limb(2924135009644862965),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(4801873560487417068),
                Limb(11579843090931958574),
                Limb(8142965511381588828),
                Limb(664041138522498590),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9720424143276205533),
                Limb(13672741349234585031),
                Limb(161140166804275441),
                Limb(1417967980227469401),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13432376423120810242),
                Limb(15185778580732008330),
                Limb(13637877109405844942),
                Limb(336203992497445847),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17230534584780421836),
                Limb(15125202124853182295),
                Limb(7279785388527028940),
                Limb(1734152006302211155),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(737798507938371176),
                Limb(3609801521675345000),
                Limb(8557523351622418795),
                Limb(1633078105248724012),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12591227003487470513),
                Limb(10451667258028886258),
                Limb(14368578687486249536),
                Limb(1639938048459656729),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17240897257593905435),
                Limb(10259526045219891893),
                Limb(5779198524521072878),
                Limb(1064992183326424868),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7346907265455616252),
                Limb(11334555099112427847),
                Limb(17405683521351225395),
                Limb(1966445311811117467),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6667894282872958889),
                Limb(15287072969607770955),
                Limb(16053117307566678601),
                Limb(1771940234576630088),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16457200513615858948),
                Limb(1363012032045872471),
                Limb(13793063892945107244),
                Limb(1082135046887459925),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14008318309672330299),
                Limb(14750150913288550934),
                Limb(2296045504620129875),
                Limb(2270954271412929409),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16187931672791590302),
                Limb(16887328658791000672),
                Limb(11234287282778757371),
                Limb(1442638592240458983),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8897156407387510049),
                Limb(15984982495198517863),
                Limb(17421687785554822592),
                Limb(1071881162499941546),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1638545499395291659),
                Limb(5822182819416681636),
                Limb(9242661150555977094),
                Limb(556081657173648030),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16848984163182307089),
                Limb(9072648677822610615),
                Limb(4865113997150468362),
                Limb(1111546736166243559),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13147767127021886623),
                Limb(6333555672039705613),
                Limb(9066376404997997140),
                Limb(462334934565175355),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10641768696285329809),
                Limb(14013039105488449359),
                Limb(11058082834545335838),
                Limb(651655042962670800),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8573601090637624602),
                Limb(6345295967575959198),
                Limb(10680753565020385918),
                Limb(501061562747655111),
            ],
        }),
    ]
}
// Stores the round constants values for state_width = 6
pub fn round_constants_6_5<F: Field + PrimeField>() -> [F; 390] {
    [
        F::from(Uint {
            limbs: [
                Limb(8843532994139492416),
                Limb(5638228540268051800),
                Limb(15791193591680409915),
                Limb(1740795397809159342),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15083595005181808138),
                Limb(11749992310599342055),
                Limb(10476077721814713529),
                Limb(1887298042088905573),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1736660663383483851),
                Limb(8999439701249853052),
                Limb(5351854848226370528),
                Limb(451830954609163121),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15969075690852226107),
                Limb(15076628920417447451),
                Limb(17085551717781718207),
                Limb(1454960979645765091),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4322010278445937883),
                Limb(5045902365924747798),
                Limb(17280493613552905713),
                Limb(572413530707432214),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12666372975765629904),
                Limb(11247861269891419053),
                Limb(743346511892152141),
                Limb(1862892050399740479),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6557572462577603753),
                Limb(14726797191290328909),
                Limb(1492384221566546823),
                Limb(1633810657758674854),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4528983010674812962),
                Limb(12853645865180323197),
                Limb(1265959260718384854),
                Limb(464375422194113226),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1945930972052355034),
                Limb(18097974072024081032),
                Limb(4320494756019678874),
                Limb(2397395101588647759),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6512151564618982963),
                Limb(7160003362424910040),
                Limb(116432094704254231),
                Limb(2005896059163489103),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1376145384525511110),
                Limb(2990027345910719859),
                Limb(13744041080978530376),
                Limb(663408763228636136),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1341024794905820441),
                Limb(9004647880279100508),
                Limb(15674143410802042292),
                Limb(844711954300600034),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17173203836531588970),
                Limb(1342163189989090524),
                Limb(451530717504566938),
                Limb(3031974443920300349),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3012889784342987142),
                Limb(11035402468472900490),
                Limb(2682883835406634213),
                Limb(984352209670944184),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13951478086494030831),
                Limb(1206388921622437187),
                Limb(88698323518432977),
                Limb(1046023887366764695),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14127230291421095001),
                Limb(9946983062441191917),
                Limb(8238374474654402715),
                Limb(1298919769439792150),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10089267326502542231),
                Limb(2940160428095401908),
                Limb(1750644581773040506),
                Limb(772491262864089200),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10105954986145043946),
                Limb(1870030953694858995),
                Limb(6985041357385502783),
                Limb(1758390971577321208),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12454373703847459893),
                Limb(13919687468806020210),
                Limb(3640686805137856789),
                Limb(33588732180350414),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8676796491569374555),
                Limb(4725417604415351169),
                Limb(16207872038281619349),
                Limb(1916405077032387773),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2628376919840134905),
                Limb(9958080294789152780),
                Limb(8875161732715663347),
                Limb(2203988159166997804),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2420222908227840854),
                Limb(10356765051401762296),
                Limb(839777703059048276),
                Limb(2443621966418541886),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7753582957192641402),
                Limb(4462355265039968035),
                Limb(3905087357024792758),
                Limb(1340792119644797626),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16285888317210367987),
                Limb(6834658986538410170),
                Limb(1008620716251825984),
                Limb(748874606562349269),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6658751198375722633),
                Limb(897255970831269934),
                Limb(4907841428778869274),
                Limb(727022533770345382),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1583455333090325205),
                Limb(2068830828922631516),
                Limb(12610134796576597311),
                Limb(1022892359103967264),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9478475022883064930),
                Limb(6300871962260432937),
                Limb(12840762695060331786),
                Limb(1682348801837991626),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(7708338002591824405),
                Limb(14206317100896020452),
                Limb(1168754150321357579),
                Limb(1455001748511680990),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1324517903682845640),
                Limb(14314483762932818093),
                Limb(16361841800148094053),
                Limb(809068192885377617),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1500974849020841074),
                Limb(798140907543601189),
                Limb(12258427730488966723),
                Limb(590903808089904443),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(4622769860757651141),
                Limb(10681648810867434359),
                Limb(780385592605526082),
                Limb(1914052595958349699),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3502949320433736973),
                Limb(8353778206055903680),
                Limb(6129738656214524594),
                Limb(1811567703503521775),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16842288185732685511),
                Limb(15288969602935843666),
                Limb(14180563657592433085),
                Limb(880169937383751727),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10595602374368030031),
                Limb(13423543272832729504),
                Limb(9194533981403291558),
                Limb(770912049465365454),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5487539324394327464),
                Limb(17371939708734402909),
                Limb(926784216419538595),
                Limb(1704071641595349083),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(4416734224822953207),
                Limb(10326757264097292759),
                Limb(6680312583114461902),
                Limb(1846412637979039394),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(6742899862782737615),
                Limb(2391829165826736055),
                Limb(9172557669911695123),
                Limb(1670160561966651826),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(4084949743054965798),
                Limb(2787194968778071471),
                Limb(12272718713780052851),
                Limb(502060255276966077),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16833084239968977578),
                Limb(9520312556516922836),
                Limb(1152465146053701698),
                Limb(2186438765962083092),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9096805221076313263),
                Limb(3679506550509657183),
                Limb(7931222862045052874),
                Limb(707070552709112053),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8303835393103058854),
                Limb(11968468996393100157),
                Limb(3929221090935784362),
                Limb(2270410879099488123),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(2152191339528294171),
                Limb(12767737272546487487),
                Limb(4216421493559787497),
                Limb(1513434421700824189),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(2119962041741225305),
                Limb(15690579794682711878),
                Limb(10756019222125665325),
                Limb(597908658524331373),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(11785331207916297276),
                Limb(1318806453124063267),
                Limb(8768371829578781749),
                Limb(1787244768964459893),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(7461117396661110868),
                Limb(11202509606035188484),
                Limb(17472612452611901855),
                Limb(1041242043891521561),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(13765997467644495960),
                Limb(13540988855587506641),
                Limb(10801638474949125210),
                Limb(593566048111487796),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3895099330460158693),
                Limb(12974571045630070929),
                Limb(3239192398726245711),
                Limb(2249226303070652114),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14960016396784128136),
                Limb(12791013364048246020),
                Limb(8610951347175311425),
                Limb(1289431645576931236),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9854136783564481134),
                Limb(2156024499207147076),
                Limb(13334596358011903109),
                Limb(1913569071946772438),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14581717682394397060),
                Limb(16373170956620606975),
                Limb(12757121852926961318),
                Limb(1558378214010492288),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17011702275525190887),
                Limb(4691201203188068348),
                Limb(11923355996008408414),
                Limb(1206049227701737657),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9831731121731624604),
                Limb(14626664112097124854),
                Limb(14695345498136704639),
                Limb(2081259389188516007),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(3347912912342176365),
                Limb(4597588314382680369),
                Limb(10959105587493882408),
                Limb(1532080026144990734),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1174470268730466036),
                Limb(7322636536085632253),
                Limb(15111092294280069879),
                Limb(1455170221272519096),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14383062960831637703),
                Limb(5206326935972867703),
                Limb(1317773276742024039),
                Limb(1091449709997130679),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8842849456943524756),
                Limb(6539278454385831051),
                Limb(9275973834220215807),
                Limb(2155398280637433933),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(18417518456005215880),
                Limb(4764358043570780954),
                Limb(2262690253346334147),
                Limb(1669155534756774672),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10792390235582882761),
                Limb(919099419235622572),
                Limb(16233216167438589403),
                Limb(1363977535837488513),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(14594663876887972648),
                Limb(11213663916989915732),
                Limb(12517524933457548383),
                Limb(712356557730018057),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16918179261468920420),
                Limb(12360778991368226532),
                Limb(13705902844896022599),
                Limb(2035156703374891941),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10767513422698429409),
                Limb(11724176890699620787),
                Limb(11670984220265009977),
                Limb(1347620278125906063),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1600716572688758958),
                Limb(6098842635751977908),
                Limb(13162026849285577594),
                Limb(1560627374242884654),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5951883154004377917),
                Limb(13110435966761555147),
                Limb(14753224910247703102),
                Limb(631841938144806512),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8633441755559039191),
                Limb(13546163801736465768),
                Limb(15220791802614654074),
                Limb(2343018357079804098),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10046671209514613710),
                Limb(547821657503828985),
                Limb(17844447293601701679),
                Limb(31422786606179822),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8776662092256554288),
                Limb(5727470315660347339),
                Limb(10806369868622602159),
                Limb(707348586605422616),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(2640754666130084258),
                Limb(2832430566729059324),
                Limb(9888205143786022131),
                Limb(807496389165647022),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(15177116566857820707),
                Limb(542065127025486752),
                Limb(4537133901202462782),
                Limb(2019855760734968661),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17072214284928901771),
                Limb(8397929114845202483),
                Limb(395045710513429053),
                Limb(2037175217040140643),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9245409287013650299),
                Limb(17247110074231524469),
                Limb(5055359759698509985),
                Limb(1630514302848010172),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1244244197266343356),
                Limb(17637265513296901619),
                Limb(12498899828163578875),
                Limb(637599505395315431),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(387488744208813870),
                Limb(1156646750996403758),
                Limb(17183054305905767328),
                Limb(1877856262544993891),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(17111410463376043102),
                Limb(12561824676738084657),
                Limb(9278966995215157928),
                Limb(1327757700268348121),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(8854502081436419018),
                Limb(14693510875203505428),
                Limb(4957074413376792053),
                Limb(2142917310244176115),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1507099821690628171),
                Limb(8340741925927436620),
                Limb(6355426843956445500),
                Limb(1601187502583230566),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(5672586747515387592),
                Limb(13473820744580321155),
                Limb(17809901109913730452),
                Limb(1834531082879219591),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(16265478635952426195),
                Limb(10752195230036435645),
                Limb(12465790310226091656),
                Limb(540914742014659877),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(1638961961844112209),
                Limb(7652820220458239900),
                Limb(14869798937785036530),
                Limb(1919779476651763385),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10670361566320202941),
                Limb(16107951807978680443),
                Limb(11751711702531044281),
                Limb(461574874951782791),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(9970359179902515397),
                Limb(2906011794621211145),
                Limb(15000728004783933513),
                Limb(907085306457387985),
            ],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [Limb(0), Limb(0), Limb(0), Limb(0)],
        }),
        F::from(Uint {
            limbs: [
                Limb(10754938549595837109),
                Limb(15581293459450592801),
                Limb(9908705164329458450),
                Limb(383521169420088953),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8767612470912213904),
                Limb(17142357418012803722),
                Limb(6093389983214669655),
                Limb(1175621120275921411),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1888016501392737230),
                Limb(9212179428179684838),
                Limb(9008734610045321366),
                Limb(1869988289227948459),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3082797225415828324),
                Limb(5067577197858815838),
                Limb(5883781769527847372),
                Limb(928995131384779531),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3795140932706646319),
                Limb(12945233547491233940),
                Limb(12758999914310596288),
                Limb(467410725559065728),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1343836255766189678),
                Limb(4270901676610337617),
                Limb(17235805500496899611),
                Limb(1073504760669909064),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4066338070416952640),
                Limb(3993041127301023125),
                Limb(3876163070173515721),
                Limb(581988952159280093),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16963892930224997304),
                Limb(11225390462471591253),
                Limb(8770083208207314175),
                Limb(1439980879860659026),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4031683833403627717),
                Limb(9145022081540463850),
                Limb(13256709380460855233),
                Limb(209674278304095993),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2874945013039159970),
                Limb(14249312861431887737),
                Limb(16082735287109485611),
                Limb(709306578241045650),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14963186210855991592),
                Limb(16996933489984883582),
                Limb(16489146750858250895),
                Limb(3461655661359788551),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(5717554666639766387),
                Limb(8916180812186049631),
                Limb(11815417039088078815),
                Limb(2071948395136181725),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16312906045474228984),
                Limb(281592587839017286),
                Limb(15343127636732739668),
                Limb(736498252036258248),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1476083715476800547),
                Limb(16454468757118559987),
                Limb(16360054612656300805),
                Limb(1434856580557174186),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12875047958666518490),
                Limb(13420045551548851125),
                Limb(8992741092238977004),
                Limb(897649416159039593),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2792226934053416406),
                Limb(517826728983793953),
                Limb(10614168989876641396),
                Limb(1691592454835563783),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1680639648607505627),
                Limb(11699797686948907436),
                Limb(14915947955112978316),
                Limb(2079520798204467127),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(46026626470083061),
                Limb(14727006183969684235),
                Limb(10008974654106774824),
                Limb(1909164450334806451),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10211968043002548157),
                Limb(5877466300218889678),
                Limb(7185139572894492651),
                Limb(1158986004742723185),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7778266236507649920),
                Limb(7437376912566191389),
                Limb(6910566786631628380),
                Limb(1262173512639671645),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11465421906326704946),
                Limb(11671816574663435127),
                Limb(12242796711388742626),
                Limb(1221958801342307752),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16931133248674603758),
                Limb(634414255555619304),
                Limb(2133647140289151992),
                Limb(2039075817666086240),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17968946355311671467),
                Limb(3122903497276585897),
                Limb(3240776022685434634),
                Limb(611315517673000716),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2733948509820859015),
                Limb(11007607986863681454),
                Limb(918668522366601881),
                Limb(1025278358997850057),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16209765506933380959),
                Limb(3268974346795004900),
                Limb(16565855787702843585),
                Limb(1220712726209656157),
            ],
        }),
    ]
}
// Stores the internal matrix values for state_width = 4
pub fn internal_mds_4_2<F: Field + PrimeField>() -> [F; 16] {
    [
        F::from(U256::from_be_hex(
            "0ed69e5e383a688f209d9a561daa79612f3f78d0467ad45485df07093f367549",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "04dba94a7b0ce9e221acad41472b6bbe3aec507f5eb3d33f463672264c9f789b",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0f8c713453888a1782f07875fdc36ad032a64182a9487e0d68550c6db1cb58e8",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0a3f2637d840f3a16eb094271c9d237b6036757d4bb50bf7ce732ff1d4fa28e8",
        )),
    ]
}

// Stores the internal matrix values for state_width = 6
pub fn internal_mds_6_5<F: Field + PrimeField>() -> [F; 36] {
    [
        F::from(U256::from_be_hex(
            "042530bc804bdc60602858dcc9b9b93c6a20e0d4869efa96e0f395e119c2a2b4",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "1d512a0ae00ab28cb6c331cf7619cd689b32d4dfb866fb4b93c104d7f5a169ed",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "07b07405e603fb19eab1f2ec87a5a162bb881a92451b70e3cadd2aac9c54d7de",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "1b66c17caa14fb63e0b194165d884a11dada0cdbe6aae3647c7a67151c5c6212",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0f2cad32a3d6d7ab934e127b3699c3ef08ebc6378b9b9e909a493865750befe4",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(U256::from_be_hex(
            "02fa05dbed3ec79ec1deb688c4f1b7ee7f450bd4477feed3b9ae41fc91355f76",
        )),
    ]
}
