use crypto_bigint::{Limb, Uint};
use traits::traits::{Field, PrimeField};

// Stores the MDS matrix for state_width = 4
pub fn mds_4_2<F:Field + PrimeField>() -> [F; 16] {
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
pub fn mds_6_5<F:Field + PrimeField>() -> [F; 36] {
    [
        F::from(Uint::from_be_hex(
            "1dcdb642190c6d7ce3b118f400a9f177580d5a048cbab560769ad66776917efe",
        )),
        F::from(Uint::from_be_hex(
            "0dd447e573f95cad1d221844a4d07aa08a680d7b10b4c23d96432b29a95b2ccd",
        )),
        F::from(Uint::from_be_hex(
            "19bff069b6d1ca99b347c9ac5be1d56141a1ff102d501ad2b10e604a3aabf1a5",
        )),
        F::from(Uint::from_be_hex(
            "173137476fb55fc5dae833bf847dabc618576ae0dc629598f01c6c5ce7363857",
        )),
        F::from(Uint::from_be_hex(
            "16f281e30517308b440dbad57f47c276ec7c3ea8577ec09b26ef9e8e03839982",
        )),
        F::from(Uint::from_be_hex(
            "0c269b502d02fd0eea9d2dccfbe78361c970db0bb12522d880e05eb0d5557f9b",
        )),
        F::from(Uint::from_be_hex(
            "231c89cae118f10ff5cc36c9cb78bf8d36d846cb4f184f4c9629d82c3b5b9fd3",
        )),
        F::from(Uint::from_be_hex(
            "0f81a28e2e4c6f7833c3daf74c613823071df7d0adc7bc3bcf52aaf5317cdc45",
        )),
        F::from(Uint::from_be_hex(
            "070706e9be0bc5742e47ebcb2be0bc37cec9054fc54ffe6c87a2d78f9ccd89fd",
        )),
        F::from(Uint::from_be_hex(
            "203870e18c0b367fada16adacd8637acc98a753af019b3f31cc1bc37ced437ca",
        )),
        F::from(Uint::from_be_hex(
            "113d8c6e01db5d4f66980678845468f83fb5cbb6bfec19258ce0cad107e3c665",
        )),
        F::from(Uint::from_be_hex(
            "13dcfee65a47d64e65b48ff52f83e13eef6be9dcfbc62ea1334b445352b8353f",
        )),
        F::from(Uint::from_be_hex(
            "1490559d6a19a7a09251280a35f6b85039133048d2399f967edab5690927b99c",
        )),
        F::from(Uint::from_be_hex(
            "27df39224bc807d147e43441e03c7942e62af9ba797785e0f91b1ed72a5dfb41",
        )),
        F::from(Uint::from_be_hex(
            "2359bca34773bfb0eb8b68257b8c39cc34b59ca491fad18023cf37aa1e75dd0a",
        )),
        F::from(Uint::from_be_hex(
            "1a85d7c2ddefda0c43fc74c66dd5cb27a9e3dc22edd7fc9ddb503ba5d659fd6e",
        )),
        F::from(Uint::from_be_hex(
            "0e52f60836f85faf5383d7ea7b2894dc8d03641cb78cd1f30255668285fd5958",
        )),
        F::from(Uint::from_be_hex(
            "18e48a75d0dc31a5c4aec5ef86563882874690c8a3abe693f17b2e1475b5001e",
        )),
        F::from(Uint::from_be_hex(
            "20d4db8694014cbf284a033ab7bd6a7cc198d9fc451099de52f3c478cd76113e",
        )),
        F::from(Uint::from_be_hex(
            "033c0a53e822ed2596abd91bedf4a877de2b7f27b5db1e1c1f9aafa94bc8d600",
        )),
        F::from(Uint::from_be_hex(
            "0f1765613333e8cf1101c4682dd50c026c72a8790f1f6376b27d0b5d0545f452",
        )),
        F::from(Uint::from_be_hex(
            "01264bd289f02e5128d06b4a067a85b07a79727043c1a9519e2f15da3f034e34",
        )),
        F::from(Uint::from_be_hex(
            "1b840915a91ab455c91d6541136a274355850961fefa9215da45f59724e669c6",
        )),
        F::from(Uint::from_be_hex(
            "00b13261758c2e41d728692f35b7ea174d9c908af7d537ab72016fa6ec667c63",
        )),
        F::from(Uint::from_be_hex(
            "03898c0e12c25a8565d7ba63151863e74a32afbac6ae849921e534d75b161419",
        )),
        F::from(Uint::from_be_hex(
            "0b636f90598842d3193767ef59f5a0d089bd8b3f9600038b4ebcb0a90313eca3",
        )),
        F::from(Uint::from_be_hex(
            "0b406fc96fbccecba28876a282024b202060f21027a995e52028ae937160f74a",
        )),
        F::from(Uint::from_be_hex(
            "2beded856398b60738a0abe6e5b54724f496265db97ce6d40638ea11a231ce2d",
        )),
        F::from(Uint::from_be_hex(
            "193ce24897b2ce4c3660111c8b7502abb93ecb7610af6c6ba87d22c7fd72da79",
        )),
        F::from(Uint::from_be_hex(
            "17e8479d3f9a03df042d466457efbc7002629622775d9abd0e13629fba963be5",
        )),
        F::from(Uint::from_be_hex(
            "281a60692faffca86751e8c63e2d2cc333980e94a93d103ab13497b4689ca30a",
        )),
        F::from(Uint::from_be_hex(
            "15bc28942e28856efe0c84659483e52db8ee225edda3404598bb509661b12279",
        )),
        F::from(Uint::from_be_hex(
            "2a9e8b3370fce7720455c568fc78d29485fd022b0669627a88e66d8e70106b73",
        )),
        F::from(Uint::from_be_hex(
            "05e8aac71c1733d5aaff3576f56015a0bb46b1753309a01a40748b652f94c854",
        )),
        F::from(Uint::from_be_hex(
            "250d7e6361e86edc88d6a5c8721ac8a45c088884109c85703d83bf0118152b2f",
        )),
        F::from(Uint::from_be_hex(
            "2a0884a1f855d6eeb7497fd8223fe8b64e2214174dc331feab4650887cc0fd1c",
        )),
    ]
}

// Stores the round constants values for state_width = 4
pub fn round_constants_4_2<F:Field + PrimeField>() -> [F; 256] {
    [
        F::from(Uint {
            limbs: [
                Limb(11494181749597339992),
                Limb(8227544131179568399),
                Limb(3066609924631073546),
                Limb(2053500369819756060),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15289420477277556949),
                Limb(11205906428753264082),
                Limb(5665441093877292851),
                Limb(1046371750460922029),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17056219967167906590),
                Limb(16411911735925289755),
                Limb(10153933215628784027),
                Limb(1105501874670182129),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6525547162025819280),
                Limb(249491045482295640),
                Limb(2893675533636546803),
                Limb(2296869732408112843),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8542175105240468002),
                Limb(5477084558748833093),
                Limb(6047714111500160135),
                Limb(854182114051883107),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15169625177751094379),
                Limb(14234474952201518620),
                Limb(2150659848101236262),
                Limb(1722749445506663230),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17883129646560746322),
                Limb(11441142816474347344),
                Limb(1400419157080731430),
                Limb(527568624759849665),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6690440665419846477),
                Limb(11476075000366151095),
                Limb(18133411385382220781),
                Limb(1352360512791542612),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16136192599378221097),
                Limb(3111627176433615174),
                Limb(3542459224376126321),
                Limb(473074954077891528),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6499088130948692129),
                Limb(8450654598832168777),
                Limb(17729008647781294990),
                Limb(1660428567382876872),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16590441818789705350),
                Limb(11827970562804917318),
                Limb(14827670273426629013),
                Limb(1063859945062847230),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17097252518070759312),
                Limb(15968577556535678906),
                Limb(8913791269084867547),
                Limb(837209136552961002),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(9831985310923426374),
                Limb(2847009671866850712),
                Limb(15964095417491445888),
                Limb(1472743879360935493),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7423494208385349099),
                Limb(1022896490615205334),
                Limb(1189733669575234499),
                Limb(1714699081589608213),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17978009580634993216),
                Limb(3852033805750780441),
                Limb(7875528568706444567),
                Limb(1309105302700492884),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10009045429319766234),
                Limb(2035659664832985410),
                Limb(16976481256935599214),
                Limb(1775372038117160675),
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
                Limb(16156708499916712703),
                Limb(12116409524835271958),
                Limb(5609034694708309244),
                Limb(971842657917130863),
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
                Limb(2025113440042819967),
                Limb(3502547166072423622),
                Limb(844102288352707825),
                Limb(2134455801102375269),
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
                Limb(9079886242123218487),
                Limb(3560999507394258711),
                Limb(10997265703676617145),
                Limb(1248039408623509621),
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
                Limb(17742949552404804865),
                Limb(13236142755451751181),
                Limb(14520012491682260142),
                Limb(1335970286975935027),
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
                Limb(8789953895425414298),
                Limb(12232150937329817828),
                Limb(3538960643369923148),
                Limb(1639632567289162448),
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
                Limb(6249524562271154910),
                Limb(11154020219770049722),
                Limb(4706595000326507698),
                Limb(1951629720402702583),
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
                Limb(5981204866331823676),
                Limb(15828883159154982279),
                Limb(17155911160726242685),
                Limb(1989976360376810876),
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
                Limb(11622079236710524692),
                Limb(2562000732605634328),
                Limb(7427654656418981343),
                Limb(2211403280858641522),
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
                Limb(11611827009759377566),
                Limb(7650074715870800350),
                Limb(15855271697903636812),
                Limb(165365728363629870),
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
                Limb(7940816120204315898),
                Limb(5401391188926932239),
                Limb(15770866840334719328),
                Limb(1047228470862918974),
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
                Limb(16919551300509334781),
                Limb(5810444791200601276),
                Limb(736194258039534317),
                Limb(1637698086770614045),
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
                Limb(9292810768453336516),
                Limb(5921328410050383681),
                Limb(14723392246516319283),
                Limb(2201210412398076598),
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
                Limb(832122558391387117),
                Limb(7044499180654207817),
                Limb(6840178992595891070),
                Limb(965397896944834930),
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
                Limb(4183930892902501554),
                Limb(4167439033969460158),
                Limb(11153804677311854943),
                Limb(1449210033596475110),
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
                Limb(15328708714815824515),
                Limb(11848006846947479828),
                Limb(7152843413421047852),
                Limb(1121712273831010963),
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
                Limb(12218936346833539624),
                Limb(12298039202832113217),
                Limb(5350364679479814789),
                Limb(813262785015976683),
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
                Limb(10657240157663266213),
                Limb(9810994425555840784),
                Limb(7365124370746969087),
                Limb(952794242534711295),
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
                Limb(17356766243542469763),
                Limb(7853172481146846388),
                Limb(3638244892116805924),
                Limb(2264915855825023032),
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
                Limb(8120962196213549782),
                Limb(8926009176507112789),
                Limb(16430161995150011142),
                Limb(1835586876852902569),
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
                Limb(8449723565304175327),
                Limb(6804051549139367803),
                Limb(16058445116361830),
                Limb(1677052056808047056),
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
                Limb(13804582097816129289),
                Limb(8120735702851297049),
                Limb(2746777025701766916),
                Limb(975413915928753122),
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
                Limb(15650725934838504692),
                Limb(16813500002941588528),
                Limb(5829793052358710609),
                Limb(1084929890685867621),
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
                Limb(5062360198325079447),
                Limb(7767015967130134238),
                Limb(880763963731889842),
                Limb(1150145379132670216),
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
                Limb(9877634740003461968),
                Limb(15305526524726477554),
                Limb(7163179127373581885),
                Limb(1089360442308034895),
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
                Limb(6547261112610779886),
                Limb(11439240109222208368),
                Limb(4285306916792171174),
                Limb(1215825659515091230),
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
                Limb(2678577407978343311),
                Limb(10838346931562558279),
                Limb(6872895825937330688),
                Limb(1871849334175386345),
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
                Limb(15655029743740763078),
                Limb(16741143245850398466),
                Limb(14906156442746925629),
                Limb(455505439230106938),
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
                Limb(5387754379680692082),
                Limb(16917130632840641774),
                Limb(7560072902085905660),
                Limb(485917006966317193),
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
                Limb(14787686018151434830),
                Limb(2420164645854675140),
                Limb(14663405056826668640),
                Limb(1928412916435631224),
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
                Limb(16350977982559826200),
                Limb(17981424213238392901),
                Limb(11161463255254259206),
                Limb(572955048842429418),
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
                Limb(14368351266755666740),
                Limb(8168853478898348303),
                Limb(8323883545368080514),
                Limb(1426959740868358189),
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
                Limb(5111701483426037018),
                Limb(11853149512539512659),
                Limb(2718959925328520163),
                Limb(729588604452381040),
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
                Limb(1363229328352251497),
                Limb(4705394571885178175),
                Limb(10003785012035127265),
                Limb(1669886663438415214),
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
                Limb(3125946715955639618),
                Limb(13848314157860583664),
                Limb(13722621078953918482),
                Limb(979818744840110668),
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
                Limb(11933812348984203538),
                Limb(5501531874056751920),
                Limb(9740101924906479978),
                Limb(1409968515755338160),
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
                Limb(8562436048843628728),
                Limb(5698720978621539190),
                Limb(11580408187022564723),
                Limb(1395615787649258270),
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
                Limb(8227372983679851918),
                Limb(13340754926221886663),
                Limb(17523512761915037083),
                Limb(498642606459719787),
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
                Limb(11693975295131647906),
                Limb(12088984719555086766),
                Limb(4077489920213110002),
                Limb(2231269866486547376),
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
                Limb(4776338040077832425),
                Limb(11622349861514992955),
                Limb(13880051934551539786),
                Limb(597118281000413838),
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
                Limb(7581044969695965969),
                Limb(6315919076925207527),
                Limb(5107078898858443551),
                Limb(1422635911114629154),
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
                Limb(383947771017199665),
                Limb(5407905049496439242),
                Limb(3599660408990852880),
                Limb(3096027254698942242),
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
                Limb(16519790583391928506),
                Limb(18133514156793408398),
                Limb(5358957488932960541),
                Limb(572717594466056318),
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
                Limb(17760442202285773445),
                Limb(12438599616357824906),
                Limb(1615383193025477237),
                Limb(2019005067721071805),
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
                Limb(9370878172622886503),
                Limb(17093821906065247858),
                Limb(7498052621102594378),
                Limb(669546631294138073),
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
                Limb(16804640926559188848),
                Limb(13152832239015562121),
                Limb(12223319205770110208),
                Limb(512841774470013836),
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
                Limb(17566014820127302425),
                Limb(11559613071837095415),
                Limb(11224154214118947262),
                Limb(1858219347483329519),
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
                Limb(6697380752769643044),
                Limb(7817200188990371345),
                Limb(5485367975575434007),
                Limb(1648945001504495764),
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
                Limb(16353565717890677018),
                Limb(6534841590887529684),
                Limb(14758174668456967007),
                Limb(1627342449238046408),
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
                Limb(17501652164828376729),
                Limb(122375380407927131),
                Limb(16314341681750708613),
                Limb(694694590110744672),
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
                Limb(2746941364695896211),
                Limb(1692936538047623178),
                Limb(6129442019391814403),
                Limb(1501674103832774210),
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
                Limb(3209469567527114473),
                Limb(7416596281409572080),
                Limb(6509010887719057927),
                Limb(1839067281977529027),
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
                Limb(15946641342015358275),
                Limb(4079842102610630489),
                Limb(11114426005033477682),
                Limb(312894661381777785),
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
                Limb(11002389281166703887),
                Limb(3108684875380118394),
                Limb(16337765439704938198),
                Limb(761354666753803758),
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
                Limb(11377172954070188399),
                Limb(17738383278066811329),
                Limb(14936246858401308818),
                Limb(49019804242077980),
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
                Limb(4882389708270356961),
                Limb(13365236879282828549),
                Limb(1499548028508243395),
                Limb(1918907101231118762),
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
                Limb(5960216970650611231),
                Limb(2746491046203579250),
                Limb(13628963476733403538),
                Limb(1906448767338914183),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7508375676790698052),
                Limb(142385142124056566),
                Limb(2075374479085143371),
                Limb(991940957895256672),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14717837250266512104),
                Limb(7532168481414470548),
                Limb(2646986829014164601),
                Limb(1960860890677234462),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1856021251341573957),
                Limb(2339845727031841700),
                Limb(5905526121222670027),
                Limb(1575018492651679019),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12820343829654407404),
                Limb(12483836988742656960),
                Limb(2346575420710866922),
                Limb(1672386440363158813),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11146209445808336642),
                Limb(11437921451328563799),
                Limb(4372539818649716679),
                Limb(1922170954627194146),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6468702154137395517),
                Limb(15626607694239940104),
                Limb(7311402515256128812),
                Limb(2177001733315909488),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3608093528265066053),
                Limb(16096769603434078363),
                Limb(10038096340057347148),
                Limb(2195651240344111650),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11976148534707638292),
                Limb(5384484272495452475),
                Limb(15756142239150126167),
                Limb(1123166062745384190),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13378987070687154206),
                Limb(4542318944413391466),
                Limb(8107312691780164526),
                Limb(2190142799876588794),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(9469205349842416687),
                Limb(7699181291257246079),
                Limb(4103631254383245539),
                Limb(1606095197938615482),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10718605847556617649),
                Limb(1726702632802337061),
                Limb(9809338988879329738),
                Limb(1089267305055582769),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13617277712091678906),
                Limb(16997123869543077969),
                Limb(2087141417968916165),
                Limb(486668076355244071),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10424273022359845329),
                Limb(156609170091278321),
                Limb(11268887302026828120),
                Limb(3203780359852605456),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14031812058040018919),
                Limb(12875788017929349579),
                Limb(4463343113750443171),
                Limb(963525789377823928),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1159276550842699125),
                Limb(5426410619708328657),
                Limb(12803984163539533750),
                Limb(1164408354411633928),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1545415218526404731),
                Limb(6079027754313929892),
                Limb(16450494207598802453),
                Limb(3370651437787387029),
            ],
        }),
    ]
}

// Stores the round constants values for state_width = 6
pub fn round_constants_6_5<F:Field + PrimeField>() -> [F; 390] {
    [
        F::from(Uint {
            limbs: [
                Limb(13312447917890517311),
                Limb(13212933353547110383),
                Limb(16843557200819205222),
                Limb(525140905673953561),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7546760354183025985),
                Limb(3265098341444984784),
                Limb(8085847414424152682),
                Limb(1781464936502067958),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14047578828305996606),
                Limb(8850713581032887337),
                Limb(17514746674439703569),
                Limb(1631027415724733999),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3074917529858521903),
                Limb(3365858207702473615),
                Limb(9609559980196179752),
                Limb(1083918625703049680),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(34957678995120887),
                Limb(2964186770485315675),
                Limb(2663099823091176265),
                Limb(607063427012248632),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(4204784124835950948),
                Limb(11453623929451421517),
                Limb(10615974708933695705),
                Limb(2099884195225406113),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(9503865611148088460),
                Limb(9087040354990546668),
                Limb(4645485835844294131),
                Limb(596792218580824656),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8504280882579225507),
                Limb(11947515215671052367),
                Limb(4416817700317139717),
                Limb(2264669069817853596),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13358886517015313022),
                Limb(3420629210221322055),
                Limb(12565740635319915341),
                Limb(2255102861209199544),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6049114661448394712),
                Limb(13962719484388602699),
                Limb(999771793439014023),
                Limb(736321582195987303),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12184382449288955506),
                Limb(16802599940271750935),
                Limb(2169447839511300606),
                Limb(1522555994189512385),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3552448021668105012),
                Limb(2926823635214140349),
                Limb(12449811393892576327),
                Limb(170070446074055476),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17787314380476048271),
                Limb(12266727233449264302),
                Limb(18169024149560656894),
                Limb(799423476923995429),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(6971465093031657687),
                Limb(1512496562860518080),
                Limb(9700195260631708632),
                Limb(554798369678150821),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(623897287144564603),
                Limb(2955840442258507629),
                Limb(13368972195853648821),
                Limb(1665403869604049305),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(18422385657214933344),
                Limb(6407228936910642208),
                Limb(12971717467874300758),
                Limb(551170859439352319),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8421540810278144132),
                Limb(13259491794635521733),
                Limb(3604518079854704129),
                Limb(2049002856119839041),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14984099769415762359),
                Limb(10034150611566839851),
                Limb(13450580824476552989),
                Limb(1666753643788958669),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14764224546120624315),
                Limb(3393008338716125332),
                Limb(9803089663998686510),
                Limb(1219521628373636190),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11304570027246261435),
                Limb(11335991197859912728),
                Limb(7181013732525464000),
                Limb(1406699178150365242),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8137635280345812163),
                Limb(17518253216545230377),
                Limb(5179407520009862124),
                Limb(1364182340348365302),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(3852530464620986171),
                Limb(8674748617062274928),
                Limb(2701155724306429712),
                Limb(1651887606480561083),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2769382426910665777),
                Limb(12570053741962796239),
                Limb(7856467514717384083),
                Limb(735937441910731830),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11622674420901152911),
                Limb(4186970467218744240),
                Limb(1189858658194369627),
                Limb(934190068087666266),
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
                Limb(3009826517075407546),
                Limb(3462720305121254117),
                Limb(14004457249077364211),
                Limb(3450477236547336452),
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
                Limb(11427652728822059740),
                Limb(7452784606037679325),
                Limb(10750436130260457397),
                Limb(1611912176847360825),
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
                Limb(15046903351984114570),
                Limb(7163796871070585112),
                Limb(6745278503785901529),
                Limb(2141904501092747243),
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
                Limb(12958026147355849959),
                Limb(16069700907477784637),
                Limb(9486148007148856302),
                Limb(25071550287217198),
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
                Limb(6342264222099589223),
                Limb(11888234595461682271),
                Limb(15668857596541282664),
                Limb(1543675888095388933),
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
                Limb(1068918481134298626),
                Limb(4234379653499122231),
                Limb(7926179327255345251),
                Limb(2058549746502419679),
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
                Limb(18133087874795392238),
                Limb(3310200644049413894),
                Limb(16363100710182426853),
                Limb(581947224672456258),
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
                Limb(3911247624493543980),
                Limb(3509592537052499452),
                Limb(9615914258137391375),
                Limb(959617121288030679),
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
                Limb(3345390287676707689),
                Limb(17245488803168396723),
                Limb(8664911889094427431),
                Limb(509197275906076013),
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
                Limb(14787489563265187210),
                Limb(711996997303369781),
                Limb(15703779027711651004),
                Limb(1076530448260117870),
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
                Limb(4802945778227405719),
                Limb(3141160808526096210),
                Limb(4268088804061052917),
                Limb(832499091620393569),
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
                Limb(5118746099893586624),
                Limb(4033582100362218298),
                Limb(6429387164928339233),
                Limb(1932754524152271411),
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
                Limb(7476491466845511788),
                Limb(7842801681240312144),
                Limb(8600918252987854913),
                Limb(2080026979153353976),
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
                Limb(5037817526088426928),
                Limb(7837061451001213366),
                Limb(14965119274167176979),
                Limb(1524339869244505485),
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
                Limb(3718603037331274165),
                Limb(12288807914396199522),
                Limb(12136057669544973881),
                Limb(517780227340341716),
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
                Limb(17827529575281779282),
                Limb(11081411341245986265),
                Limb(26407032782560676),
                Limb(2172002908537075006),
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
                Limb(2204203304898225245),
                Limb(12088877762370631889),
                Limb(12041434808516197048),
                Limb(860639402959857446),
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
                Limb(7544122765287560183),
                Limb(10472045790833951422),
                Limb(8315639990679764395),
                Limb(2622383979822611704),
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
                Limb(16988691118201869684),
                Limb(14683319831473614125),
                Limb(15627097008377868075),
                Limb(690345331068228510),
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
                Limb(1704381151900580813),
                Limb(1957299255869018378),
                Limb(17116639795200737131),
                Limb(1622951174983079471),
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
                Limb(7624873580351126383),
                Limb(13692569245456436310),
                Limb(15153650471893443301),
                Limb(1765735355172044452),
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
                Limb(15104309286945777214),
                Limb(11335604856180787804),
                Limb(15844099099057772077),
                Limb(455947814968961780),
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
                Limb(13419444752521512720),
                Limb(5618222777816361984),
                Limb(9224837642827883405),
                Limb(669851156317274443),
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
                Limb(3671451001579758535),
                Limb(10805114409499760071),
                Limb(14491321906168978505),
                Limb(591678235972049243),
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
                Limb(9750910913275796583),
                Limb(14887829903255385513),
                Limb(4504993522386914560),
                Limb(2244073101064396485),
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
                Limb(13718117076801722911),
                Limb(17840108065227086839),
                Limb(14018052530754098887),
                Limb(1564475879195149559),
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
                Limb(6142765185786446484),
                Limb(854053587849993426),
                Limb(9757097930300083446),
                Limb(852393671094124592),
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
                Limb(3169187808222496709),
                Limb(3442863026275977273),
                Limb(17896943282687277622),
                Limb(1060094969333664294),
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
                Limb(4859335248369032159),
                Limb(4408143300833484561),
                Limb(13785000305757134247),
                Limb(2840419542236594605),
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
                Limb(8340273256371251619),
                Limb(3066935670979475434),
                Limb(13309445811887341860),
                Limb(709924749123694685),
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
                Limb(17345094495693858957),
                Limb(2132987222529529275),
                Limb(2003874936303329619),
                Limb(845607749525220710),
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
                Limb(7748656228405573179),
                Limb(706060664410085253),
                Limb(12913387555003799869),
                Limb(976300065062960093),
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
                Limb(7938270626466626026),
                Limb(17625113856729900053),
                Limb(13015799064776087942),
                Limb(1266810994307884318),
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
                Limb(7886146881688013556),
                Limb(6690529589473955172),
                Limb(9503927333735720530),
                Limb(1479983638806489291),
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
                Limb(8814896628927806061),
                Limb(18335654066215426531),
                Limb(18158176785486003146),
                Limb(1981705073807098991),
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
                Limb(13216132069349885064),
                Limb(11674984873844498888),
                Limb(8277304404783898635),
                Limb(1129454606122767359),
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
                Limb(2425235315839591478),
                Limb(14456434843927178174),
                Limb(5891523505784552296),
                Limb(1274655661221393882),
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
                Limb(2425807343626857669),
                Limb(4863471652978561094),
                Limb(14794243671208960007),
                Limb(1651816322179919500),
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
                Limb(5141646379712505766),
                Limb(1634247910983814802),
                Limb(18167345418328627660),
                Limb(1323364926025699361),
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
                Limb(18422890567816757919),
                Limb(15172130720131251753),
                Limb(3814328669936310691),
                Limb(622901499791351105),
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
                Limb(11951409272227083785),
                Limb(12684070582171779531),
                Limb(6453172731621094128),
                Limb(1435863650832161339),
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
                Limb(3999943293921830557),
                Limb(2017748872024758860),
                Limb(16274954011003781947),
                Limb(1352744074546731746),
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
                Limb(14189986262438083520),
                Limb(14526504533565326978),
                Limb(14081197889994424014),
                Limb(1277390185395930940),
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
                Limb(10402033125701993161),
                Limb(4111452992092936982),
                Limb(2990728895335574803),
                Limb(421025229565971618),
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
                Limb(2390466725360893551),
                Limb(8635783160449296115),
                Limb(5267840854628259496),
                Limb(1515088170685133246),
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
                Limb(1283343586818455106),
                Limb(9655347830887622356),
                Limb(16988994675035054432),
                Limb(1638803106864260229),
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
                Limb(17747102264255098325),
                Limb(2962312778987100305),
                Limb(713119718147995080),
                Limb(1792105149798275517),
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
                Limb(3178099635338832291),
                Limb(12120998467867192722),
                Limb(12379196272907584032),
                Limb(2237028314830483249),
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
                Limb(12497564253055159934),
                Limb(14396176416035969398),
                Limb(7911568975928853628),
                Limb(1121118907930233099),
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
                Limb(17997650516650456645),
                Limb(12868048965747925172),
                Limb(7441372884936312378),
                Limb(949345469738727675),
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
                Limb(3660042515502789258),
                Limb(7784678804898783858),
                Limb(2028862286296524180),
                Limb(950640061689597232),
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
                Limb(14081871491454537932),
                Limb(10214212437155479228),
                Limb(4151014251143061847),
                Limb(884260253899311205),
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
                Limb(4226527069296049553),
                Limb(13586266026082982344),
                Limb(1501497871137958239),
                Limb(702433117784632222),
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
                Limb(18082620954995681016),
                Limb(1490274718658602473),
                Limb(148180411872584008),
                Limb(789350337553188992),
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
                Limb(9163236722191979971),
                Limb(17057733311140838622),
                Limb(7667205072729534960),
                Limb(2247768082761262096),
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
                Limb(15643003163998398491),
                Limb(2085621564896617238),
                Limb(15292116393398904558),
                Limb(677020684488006966),
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
                Limb(6269466681061849425),
                Limb(4384572009835947757),
                Limb(7991723184582383460),
                Limb(1276383750582214026),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(1044817352616904634),
                Limb(14129703465510453803),
                Limb(17602233786292484702),
                Limb(1696231295712060269),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16805048440708019747),
                Limb(8304846891449972290),
                Limb(7077473397339121458),
                Limb(2033722044006118061),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(16492120791713709387),
                Limb(7354746100285815421),
                Limb(15287298278203106341),
                Limb(1266813246184782674),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(5494684072325566554),
                Limb(4625359697438232821),
                Limb(17241473180993715764),
                Limb(1248410805049641090),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2634701192228788813),
                Limb(7560030906077116558),
                Limb(5889935091063868982),
                Limb(746411195984226412),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(12930899431189438333),
                Limb(5770261987704289766),
                Limb(1156232416234534623),
                Limb(965951040126070071),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(246317205488887640),
                Limb(1375119981529433022),
                Limb(17741594938603082653),
                Limb(626744600279399716),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13132013142399861775),
                Limb(13183090680708861473),
                Limb(10608434020248273940),
                Limb(1594950533099190734),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(14237674004784255284),
                Limb(9523836622466508361),
                Limb(2219349627222730165),
                Limb(1089030712461877218),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15363241563070883623),
                Limb(1678234220114706351),
                Limb(16448559037226950319),
                Limb(2059044642709202662),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(2254132754153651862),
                Limb(11767536580515906434),
                Limb(9129073512199453013),
                Limb(1839500994446445431),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13139294276677134401),
                Limb(1541362812662227765),
                Limb(5703072075091757445),
                Limb(1882104864754802382),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(8361522533213771721),
                Limb(2483625978799555174),
                Limb(14498812285028661136),
                Limb(2675707901963219011),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(18337265723668797650),
                Limb(1689582321590356190),
                Limb(4168803653322569118),
                Limb(740366294785480181),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(13980554172487549417),
                Limb(3660353458615699599),
                Limb(2841153300907936387),
                Limb(2145713444672641959),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(17510075838874380996),
                Limb(5871890360642238948),
                Limb(17134022395994168973),
                Limb(2171138943423426178),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(18311246071636481040),
                Limb(3155506465033594612),
                Limb(5971487967851114085),
                Limb(1307807294853754300),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(5242299243806885445),
                Limb(15495524648309843698),
                Limb(18432184476172321572),
                Limb(1523890571591574127),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(11371701409464256605),
                Limb(17020633029222085411),
                Limb(1345833970486475250),
                Limb(1139525791243877747),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(10839438671708835343),
                Limb(17951776200002143334),
                Limb(7299321370802831773),
                Limb(1712793622587087304),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(15965646635737744546),
                Limb(11715527126386303199),
                Limb(7063153552327042320),
                Limb(689697742449746862),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(848873078536564018),
                Limb(7947004222040635666),
                Limb(3034430883095794628),
                Limb(1503631903694381191),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(639283070753688500),
                Limb(16606506510129473332),
                Limb(1219070754936581710),
                Limb(2040044489703594425),
            ],
        }),
        F::from(Uint {
            limbs: [
                Limb(7152740687256118545),
                Limb(17645439806097379729),
                Limb(4756902389046447858),
                Limb(1447050112022991664),
            ],
        }),
    ]
}

// Stores the internal matrix values for state_width = 4
pub fn internal_mds_4_2<F:Field + PrimeField>() -> [F; 16] {
    [
        F::from(Uint::from_be_hex(
            "13da07dc64d428369873e97160234641f8beb56fdd05e5f3563fa39d9c22df4e",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0f50bde18ae25611d41f6249cdf1cc2c7cb4034f6b4b6c42913e65e94748f0a2",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0c009b84e650e6d23dc00c7dccef7483a553939689d350cd46e7b89055fd4738",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "011f16b1c63a854f01992e3956f42d8b04eb650c6d535eb0203dec74befdca06",
        )),
    ]
}

// Stores the internal matrix values for state_width = 6
pub fn internal_mds_6_5<F:Field + PrimeField>() -> [F; 36] {
    [
        F::from(Uint::from_be_hex(
            "02ad1ca11e2e9517bf2cf70c54998048cc67e544bad7f2b6f42f170a7b524353",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "07dd69f131a4b5b4e43f0584b76e10d21b0188ab2651b7aae6f7ff3ebcb1a644",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0f03177f0fcbc74799d67deed3bdea77990eb90f5c1c93ef30ac705142146b70",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0fe0c3ca5ccf11d6c45b50d065065fc2cc1abd7f02aa8dfbc9a443ef2aa872e9",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "19020bcbf16087675c1802aa511ec2bfe75c21daa879d5752fce1b1f4c888c5a",
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
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )),
        F::from(Uint::from_be_hex(
            "0e65af69ba570eb71eec74712592bebd565ab9dd956cab7c3a3981ede961a8ae",
        )),
    ]
}
