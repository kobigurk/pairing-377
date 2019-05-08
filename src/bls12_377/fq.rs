use super::fq2::Fq2;
use ff::{Field, PrimeField, PrimeFieldDecodingError, PrimeFieldRepr};

// B coefficient of BLS12-377 curve, 4.
pub const B_COEFF: Fq = Fq(FqRepr([
    0x2cdffffffffff68,
    0x51409f837fffffb1,
    0x9f7db3a98a7d3ff2,
    0x7b4e97b76e7c6305,
    0x4cf495bf803c84e8,
    0x8d6661e2fdf49a,
]));

// The generators of G1/G2 are computed by finding the lexicographically smallest valid x coordinate,
// and its lexicographically smallest y coordinate and multiplying it by the cofactor such that the
// result is nonzero.

// Generator of G1
// x = 81937999373150964239938255573465948239988671502647976594219695644855304257327692006745978603320413799295628339695
// y = 241266749859715473739788878240585681733927191168601896383759122102112907357779751001206799952863815012735208165030
pub const G1_GENERATOR_X: Fq = Fq(FqRepr([
    0x260f33b9772451f4,
    0xc54dd773169d5658,
    0x5c1551c469a510dd,
    0x761662e4425e1698,
    0xc97d78cc6f065272,
    0xa41206b361fd4d,
]));
pub const G1_GENERATOR_Y: Fq = Fq(FqRepr([
    0x8193961fb8cb81f3,
    0x638d4c5f44adb8,
    0xfafaf3dad4daf54a,
    0xc27849e2d655cd18,
    0x2ec3ddb401d52814,
    0x7da93326303c71,
]));

// Generator of G2
// x =
// y =
pub const G2_GENERATOR_X_C0: Fq = Fq(FqRepr([
    0x68904082f268725b,
    0x668f2ea74f45328b,
    0xebca7a65802be84f,
    0x1e1850f4c1ada3e6,
    0x830dc22d588ef1e9,
    0x1862a81767c0982,
]));
pub const G2_GENERATOR_X_C1: Fq = Fq(FqRepr([
    0x5f02a915c91c7f39,
    0xf8c553ba388da2a7,
    0xd51a416dbd198850,
    0xe943c6f38ae3073a,
    0xffe24aa8259a4981,
    0x11853391e73dfdd,
]));
pub const G2_GENERATOR_Y_C0: Fq = Fq(FqRepr([
    0xd5b19b897881430f,
    0x5be9118a5b371ed,
    0x6063f91f86c131ee,
    0x3244a61be8f4ec19,
    0xa02e425b9f9a3a12,
    0x18af8c04f3360d2,
]));
pub const G2_GENERATOR_Y_C1: Fq = Fq(FqRepr([
    0x57601ac71a5b96f5,
    0xe99acc1714f2440e,
    0x2339612f10118ea9,
    0x8321e68a3b1cd722,
    0x2b543b050cc74917,
    0x590182b396c112,
]));

// Coefficients for the Frobenius automorphism.
pub const FROBENIUS_COEFF_FQ2_C1: [Fq; 2] = [
    // Fq(-1)**(((q^0) - 1) / 2)
    Fq(FqRepr([
        0x2cdffffffffff68,
        0x51409f837fffffb1,
        0x9f7db3a98a7d3ff2,
        0x7b4e97b76e7c6305,
        0x4cf495bf803c84e8,
        0x8d6661e2fdf49a,
    ])),
    // Fq(-1)**(((q^1) - 1) / 2)
    Fq(FqRepr([
        0x823ac00000000099,
        0xc5cabdc0b000004f,
        0x7f75ae862f8c080d,
        0x9ed4423b9278b089,
        0x79467000ec64c452,
        0x120d3e434c71c50,
    ])),
];

pub const FROBENIUS_COEFF_FQ6_C1: [Fq2; 6] = [
    // Fq2(u + 1)**(((q^0) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2cdffffffffff68,
            0x51409f837fffffb1,
            0x9f7db3a98a7d3ff2,
            0x7b4e97b76e7c6305,
            0x4cf495bf803c84e8,
            0x8d6661e2fdf49a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^1) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x5892506da58478da,
            0x133366940ac2a74b,
            0x9b64a150cdf726cf,
            0x5cc426090a9c587e,
            0x5cf848adfdcd640c,
            0x4702bf3ac02380,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^2) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0xdacd106da5847973,
            0xd8fe2454bac2a79a,
            0x1ada4fd6fd832edc,
            0xfb9868449d150908,
            0xd63eb8aeea32285e,
            0x167d6a36f873fd0,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^3) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
        c1: Fq(FqRepr([
            0x823ac00000000099,
            0xc5cabdc0b000004f,
            0x7f75ae862f8c080d,
            0x9ed4423b9278b089,
            0x79467000ec64c452,
            0x120d3e434c71c50,
        ])),
    },
    // Fq2(u + 1)**(((q^4) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2c766f925a7b8727,
            0x3d7f6b0253d58b5,
            0x838ec0deec122131,
            0xbd5eb3e9f658bb10,
            0x6942bd126ed3e52e,
            0x1673786dd04ed6a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^5) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
        c1: Fq(FqRepr([
            0xaa3baf925a7b868e,
            0x3e0d38ef753d5865,
            0x4191258bc861923,
            0x1e8a71ae63e00a87,
            0xeffc4d11826f20dc,
            0x4663a2a83dd119,
        ])),
    },
];

pub const FROBENIUS_COEFF_FQ6_C2: [Fq2; 6] = [
    // Fq2(u + 1)**(((2q^0) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2cdffffffffff68,
            0x51409f837fffffb1,
            0x9f7db3a98a7d3ff2,
            0x7b4e97b76e7c6305,
            0x4cf495bf803c84e8,
            0x8d6661e2fdf49a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((2q^1) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0xdacd106da5847973,
            0xd8fe2454bac2a79a,
            0x1ada4fd6fd832edc,
            0xfb9868449d150908,
            0xd63eb8aeea32285e,
            0x167d6a36f873fd0,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((2q^2) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2c766f925a7b8727,
            0x3d7f6b0253d58b5,
            0x838ec0deec122131,
            0xbd5eb3e9f658bb10,
            0x6942bd126ed3e52e,
            0x1673786dd04ed6a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((2q^3) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2cdffffffffff68,
            0x51409f837fffffb1,
            0x9f7db3a98a7d3ff2,
            0x7b4e97b76e7c6305,
            0x4cf495bf803c84e8,
            0x8d6661e2fdf49a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((2q^4) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0xdacd106da5847973,
            0xd8fe2454bac2a79a,
            0x1ada4fd6fd832edc,
            0xfb9868449d150908,
            0xd63eb8aeea32285e,
            0x167d6a36f873fd0,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((2q^5) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([
            0x2c766f925a7b8727,
            0x3d7f6b0253d58b5,
            0x838ec0deec122131,
            0xbd5eb3e9f658bb10,
            0x6942bd126ed3e52e,
            0x1673786dd04ed6a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
];

// non_residue^((modulus^i-1)/6) for i=0,...,11
pub const FROBENIUS_COEFF_FQ12_C1: [Fq2; 12] = [
    // Fq2(u + 1)**(((q^0) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x2cdffffffffff68,
            0x51409f837fffffb1,
            0x9f7db3a98a7d3ff2,
            0x7b4e97b76e7c6305,
            0x4cf495bf803c84e8,
            0x8d6661e2fdf49a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^1) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x6ec47a04a3f7ca9e,
            0xa42e0cb968c1fa44,
            0x578d5187fbd2bd23,
            0x930eeb0ac79dd4bd,
            0xa24883de1e09a9ee,
            0xdaa7058067d46f,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^2) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x5892506da58478da,
            0x133366940ac2a74b,
            0x9b64a150cdf726cf,
            0x5cc426090a9c587e,
            0x5cf848adfdcd640c,
            0x4702bf3ac02380,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^3) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x982c13d9d084771f,
            0xfd49de0c6da34a32,
            0x61a530d183ab0e53,
            0xdf8fe44106dd9879,
            0x40f29b58d88472bc,
            0x158723199046d5d,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^4) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0xdacd106da5847973,
            0xd8fe2454bac2a79a,
            0x1ada4fd6fd832edc,
            0xfb9868449d150908,
            0xd63eb8aeea32285e,
            0x167d6a36f873fd0,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^5) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x296799d52c8cac81,
            0x591bd15304e14fee,
            0xa17df4987d85130,
            0x4c80f9363f3fc3bc,
            0x9eaa177aba7ac8ce,
            0x7dcb2c189c98ed,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^6) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x823ac00000000099,
            0xc5cabdc0b000004f,
            0x7f75ae862f8c080d,
            0x9ed4423b9278b089,
            0x79467000ec64c452,
            0x120d3e434c71c50,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^7) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x164445fb5c083563,
            0x72dd508ac73e05bc,
            0xc76610a7be368adc,
            0x8713eee839573ed1,
            0x23f281e24e979f4c,
            0xd39340975d3c7b,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^8) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x2c766f925a7b8727,
            0x3d7f6b0253d58b5,
            0x838ec0deec122131,
            0xbd5eb3e9f658bb10,
            0x6942bd126ed3e52e,
            0x1673786dd04ed6a,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^9) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0xecdcac262f7b88e2,
            0x19c17f37c25cb5cd,
            0xbd4e315e365e39ac,
            0x3a92f5b1fa177b15,
            0x85486a67941cd67e,
            0x55c8147ec0a38d,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^10) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0xaa3baf925a7b868e,
            0x3e0d38ef753d5865,
            0x4191258bc861923,
            0x1e8a71ae63e00a87,
            0xeffc4d11826f20dc,
            0x4663a2a83dd119,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
    // Fq2(u + 1)**(((q^11) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([
            0x5ba1262ad3735380,
            0xbdef8bf12b1eb012,
            0x14db82e63230f6cf,
            0xcda1e0bcc1b54fd3,
            0x2790ee45b226806c,
            0x1306f19ff2877fd,
        ])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
    },
];

// -((2**377) mod q) mod q
pub const NEGATIVE_ONE: Fq = Fq(FqRepr([
    0x43f5fffffffcaaae,
    0x32b7fff2ed47fffd,
    0x7e83a49a2e99d69,
    0xeca8f3318332bb7a,
    0xef148d1ea0f4c069,
    0x40ab3263eff0206,
]));

#[derive(PrimeField)]
#[PrimeFieldModulus = "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177"]
#[PrimeFieldGenerator = "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458172"]
pub struct Fq(FqRepr);

#[test]
fn test_b_coeff() {
    assert_eq!(Fq::from_repr(FqRepr::from(1)).unwrap(), B_COEFF);
}

// needs correct constants
#[ignore]
#[test]
fn test_frob_coeffs() {
    let mut nqr = Fq::one();
    nqr.negate();

    assert_eq!(FROBENIUS_COEFF_FQ2_C1[0], Fq::one());
    assert_eq!(
        FROBENIUS_COEFF_FQ2_C1[1],
        nqr.pow([
            0xdcff7fffffffd555,
            0xf55ffff58a9ffff,
            0xb39869507b587b12,
            0xb23ba5c279c2895f,
            0x258dd3db21a5d66b,
            0xd0088f51cbff34d
        ])
    );

    let nqr = Fq2 {
        c0: Fq::one(),
        c1: Fq::one(),
    };
    // let nqr = Fq2 {
    //     c0: Fq(FqRepr([0, 0, 0, 0, 0, 0])),
    //     c1: Fq(FqRepr([
    //         202099033278250856u64,
    //         5854854902718660529u64,
    //         11492539364873682930u64,
    //         8885205928937022213u64,
    //         5545221690922665192u64,
    //         39800542322357402u64,
    //     ])),
    // };

    assert_eq!(FROBENIUS_COEFF_FQ6_C1[0], Fq2::one());
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C1[1],
        nqr.pow([
            0x9354ffffffffe38e,
            0xa395554e5c6aaaa,
            0xcd104635a790520c,
            0xcc27c3d6fbd7063f,
            0x190937e76bc3e447,
            0x8ab05f8bdd54cde
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C1[2],
        nqr.pow([
            0xb78e0000097b2f68,
            0xd44f23b47cbd64e3,
            0x5cb9668120b069a9,
            0xccea85f9bf7b3d16,
            0xdba2c8d7adb356d,
            0x9cd75ded75d7429,
            0xfc65c31103284fab,
            0xc58cb9a9b249ee24,
            0xccf734c3118a2e9a,
            0xa0f4304c5a256ce6,
            0xc3f0d2f8e0ba61f8,
            0xe167e192ebca97
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C1[3],
        nqr.pow([
            0xdbc6fcd6f35b9e06,
            0x997dead10becd6aa,
            0x9dbbd24c17206460,
            0x72b97acc6057c45e,
            0xf8e9a230bf0c628e,
            0x647ccb1885c63a7,
            0xce80264fc55bf6ee,
            0x94d8d716c3939fc4,
            0xad78f0eb77ee6ee1,
            0xd6fe49bfe57dc5f9,
            0x2656d6c15c63647,
            0xdf6282f111fa903,
            0x1bdba63e0632b4bb,
            0x6883597bcaa505eb,
            0xa56d4ec90c34a982,
            0x7e4c42823bbe90b2,
            0xf64728aa6dcb0f20,
            0x16e57e16ef152f
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C1[4],
        nqr.pow([
            0x4649add3c71c6d90,
            0x43caa6528972a865,
            0xcda8445bbaaa0fbb,
            0xc93dea665662aa66,
            0x2863bc891834481d,
            0x51a0c3f5d4ccbed8,
            0x9210e660f90ccae9,
            0xe2bd6836c546d65e,
            0xf223abbaa7cf778b,
            0xd4f10b222cf11680,
            0xd540f5eff4a1962e,
            0xa123a1f140b56526,
            0x31ace500636a59f6,
            0x3a82bc8c8dfa57a9,
            0x648c511e217fc1f8,
            0x36c17ffd53a4558f,
            0x881bef5fd684eefd,
            0x5d648dbdc5dbb522,
            0x8fd07bf06e5e59b8,
            0x8ddec8a9acaa4b51,
            0x4cc1f8688e2def26,
            0xa74e63cb492c03de,
            0x57c968173d1349bb,
            0x253674e02a866
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C1[5],
        nqr.pow([
            0xf896f792732eb2be,
            0x49c86a6d1dc593a1,
            0xe5b31e94581f91c3,
            0xe3da5cc0a6b20d7f,
            0x822caef950e0bfed,
            0x317ed950b9ee67cd,
            0xffd664016ee3f6cd,
            0x77d991c88810b122,
            0x62e72e635e698264,
            0x905e1a1a2d22814a,
            0xf5b7ab3a3f33d981,
            0x175871b0bc0e25dd,
            0x1e2e9a63df5c3772,
            0xe888b1f7445b149d,
            0x9551c19e5e7e2c24,
            0xecf21939a3d2d6be,
            0xd830dbfdab72dbd4,
            0x7b34af8d622d40c0,
            0x3df6d20a45671242,
            0xaf86bee30e21d98,
            0x41064c1534e5df5d,
            0xf5f6cabd3164c609,
            0xa5d14bdf2b7ee65,
            0xa718c069defc9138,
            0xdb1447e770e3110e,
            0xc1b164a9e90af491,
            0x7180441f9d251602,
            0x1fd3a5e6a9a893e,
            0x1e17b779d54d5db,
            0x3c7afafe3174
        ])
    );

    assert_eq!(FROBENIUS_COEFF_FQ6_C2[0], Fq2::one());
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C2[1],
        nqr.pow([
            0x26a9ffffffffc71c,
            0x1472aaa9cb8d5555,
            0x9a208c6b4f20a418,
            0x984f87adf7ae0c7f,
            0x32126fced787c88f,
            0x11560bf17baa99bc
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C2[2],
        nqr.pow([
            0x6f1c000012f65ed0,
            0xa89e4768f97ac9c7,
            0xb972cd024160d353,
            0x99d50bf37ef67a2c,
            0x1b74591af5b66adb,
            0x139aebbdaebae852,
            0xf8cb862206509f56,
            0x8b1973536493dc49,
            0x99ee698623145d35,
            0x41e86098b44ad9cd,
            0x87e1a5f1c174c3f1,
            0x1c2cfc325d7952f
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C2[3],
        nqr.pow([
            0xb78df9ade6b73c0c,
            0x32fbd5a217d9ad55,
            0x3b77a4982e40c8c1,
            0xe572f598c0af88bd,
            0xf1d344617e18c51c,
            0xc8f996310b8c74f,
            0x9d004c9f8ab7eddc,
            0x29b1ae2d87273f89,
            0x5af1e1d6efdcddc3,
            0xadfc937fcafb8bf3,
            0x4cadad82b8c6c8f,
            0x1bec505e223f5206,
            0x37b74c7c0c656976,
            0xd106b2f7954a0bd6,
            0x4ada9d9218695304,
            0xfc988504777d2165,
            0xec8e5154db961e40,
            0x2dcafc2dde2a5f
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C2[4],
        nqr.pow([
            0x8c935ba78e38db20,
            0x87954ca512e550ca,
            0x9b5088b775541f76,
            0x927bd4ccacc554cd,
            0x50c779123068903b,
            0xa34187eba9997db0,
            0x2421ccc1f21995d2,
            0xc57ad06d8a8dacbd,
            0xe44757754f9eef17,
            0xa9e2164459e22d01,
            0xaa81ebdfe9432c5d,
            0x424743e2816aca4d,
            0x6359ca00c6d4b3ed,
            0x750579191bf4af52,
            0xc918a23c42ff83f0,
            0x6d82fffaa748ab1e,
            0x1037debfad09ddfa,
            0xbac91b7b8bb76a45,
            0x1fa0f7e0dcbcb370,
            0x1bbd9153595496a3,
            0x9983f0d11c5bde4d,
            0x4e9cc796925807bc,
            0xaf92d02e7a269377,
            0x4a6ce9c0550cc
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ6_C2[5],
        nqr.pow([
            0xf12def24e65d657c,
            0x9390d4da3b8b2743,
            0xcb663d28b03f2386,
            0xc7b4b9814d641aff,
            0x4595df2a1c17fdb,
            0x62fdb2a173dccf9b,
            0xffacc802ddc7ed9a,
            0xefb3239110216245,
            0xc5ce5cc6bcd304c8,
            0x20bc34345a450294,
            0xeb6f56747e67b303,
            0x2eb0e361781c4bbb,
            0x3c5d34c7beb86ee4,
            0xd11163ee88b6293a,
            0x2aa3833cbcfc5849,
            0xd9e4327347a5ad7d,
            0xb061b7fb56e5b7a9,
            0xf6695f1ac45a8181,
            0x7beda4148ace2484,
            0x15f0d7dc61c43b30,
            0x820c982a69cbbeba,
            0xebed957a62c98c12,
            0x14ba297be56fdccb,
            0x4e3180d3bdf92270,
            0xb6288fcee1c6221d,
            0x8362c953d215e923,
            0xe300883f3a4a2c05,
            0x3fa74bcd535127c,
            0x3c2f6ef3aa9abb6,
            0x78f5f5fc62e8
        ])
    );

    assert_eq!(FROBENIUS_COEFF_FQ12_C1[0], Fq2::one());
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[1],
        nqr.pow([
            0x49aa7ffffffff1c7,
            0x51caaaa72e35555,
            0xe688231ad3c82906,
            0xe613e1eb7deb831f,
            0xc849bf3b5e1f223,
            0x45582fc5eeaa66f
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[2],
        nqr.pow([
            0xdbc7000004bd97b4,
            0xea2791da3e5eb271,
            0x2e5cb340905834d4,
            0xe67542fcdfbd9e8b,
            0x86dd1646bd6d9ab6,
            0x84e6baef6baeba14,
            0x7e32e188819427d5,
            0x62c65cd4d924f712,
            0x667b9a6188c5174d,
            0x507a18262d12b673,
            0xe1f8697c705d30fc,
            0x70b3f0c975e54b
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[3],
        nqr.pow(vec![
            0x6de37e6b79adcf03,
            0x4cbef56885f66b55,
            0x4edde9260b903230,
            0x395cbd66302be22f,
            0xfc74d1185f863147,
            0x323e658c42e31d3,
            0x67401327e2adfb77,
            0xca6c6b8b61c9cfe2,
            0xd6bc7875bbf73770,
            0xeb7f24dff2bee2fc,
            0x8132b6b60ae31b23,
            0x86fb1417888fd481,
            0x8dedd31f03195a5d,
            0x3441acbde55282f5,
            0x52b6a764861a54c1,
            0x3f2621411ddf4859,
            0xfb23945536e58790,
            0xb72bf0b778a97,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[4],
        nqr.pow(vec![
            0xa324d6e9e38e36c8,
            0xa1e5532944b95432,
            0x66d4222ddd5507dd,
            0xe49ef5332b315533,
            0x1431de448c1a240e,
            0xa8d061faea665f6c,
            0x490873307c866574,
            0xf15eb41b62a36b2f,
            0x7911d5dd53e7bbc5,
            0x6a78859116788b40,
            0x6aa07af7fa50cb17,
            0x5091d0f8a05ab293,
            0x98d6728031b52cfb,
            0x1d415e4646fd2bd4,
            0xb246288f10bfe0fc,
            0x9b60bffea9d22ac7,
            0x440df7afeb42777e,
            0x2eb246dee2edda91,
            0xc7e83df8372f2cdc,
            0x46ef6454d65525a8,
            0x2660fc344716f793,
            0xd3a731e5a49601ef,
            0x2be4b40b9e89a4dd,
            0x129b3a7015433,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[5],
        nqr.pow(vec![
            0xfc4b7bc93997595f,
            0xa4e435368ee2c9d0,
            0xf2d98f4a2c0fc8e1,
            0xf1ed2e60535906bf,
            0xc116577ca8705ff6,
            0x98bf6ca85cf733e6,
            0x7feb3200b771fb66,
            0x3becc8e444085891,
            0x31739731af34c132,
            0xc82f0d0d169140a5,
            0xfadbd59d1f99ecc0,
            0xbac38d85e0712ee,
            0x8f174d31efae1bb9,
            0x744458fba22d8a4e,
            0x4aa8e0cf2f3f1612,
            0x76790c9cd1e96b5f,
            0x6c186dfed5b96dea,
            0x3d9a57c6b116a060,
            0x1efb690522b38921,
            0x857c35f718710ecc,
            0xa083260a9a72efae,
            0xfafb655e98b26304,
            0x52e8a5ef95bf732,
            0x538c6034ef7e489c,
            0xed8a23f3b8718887,
            0x60d8b254f4857a48,
            0x38c0220fce928b01,
            0x80fe9d2f354d449f,
            0xf0bdbbceaa6aed,
            0x1e3d7d7f18ba,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[6],
        nqr.pow(vec![
            0x21219610a012ba3c,
            0xa5c19ad35375325,
            0x4e9df1e497674396,
            0xfb05b717c991c6ef,
            0x4a1265bca93a32f2,
            0xd875ff2a7bdc1f66,
            0xc6d8754736c771b2,
            0x2d80c759ba5a2ae7,
            0x138a20df4b03cc1a,
            0xc22d07fe68e93024,
            0xd1dc474d3b433133,
            0xc22aa5e75044e5c,
            0xf657c6fbf9c17ebf,
            0xc591a794a58660d,
            0x2261850ee1453281,
            0xd17d3bd3b7f5efb4,
            0xf00cec8ec507d01,
            0x2a6a775657a00ae6,
            0x5f098a12ff470719,
            0x409d194e7b5c5afa,
            0x1d66478e982af5b,
            0xda425a5b5e01ca3f,
            0xf77e4f78747e903c,
            0x177d49f73732c6fc,
            0xa9618fecabe0e1f4,
            0xba5337eac90bd080,
            0x66fececdbc35d4e7,
            0xa4cd583203d9206f,
            0x98391632ceeca596,
            0x4946b76e1236ad3f,
            0xa0dec64e60e711a1,
            0xfcb41ed3605013,
            0x8ca8f9692ae1e3a9,
            0xd3078bfc28cc1baf,
            0xf0536f764e982f82,
            0x3125f1a2656,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[7],
        nqr.pow(vec![
            0x742754a1f22fdb,
            0x2a1955c2dec3a702,
            0x9747b28c796d134e,
            0xc113a0411f59db79,
            0x3bb0fa929853bfc1,
            0x28c3c25f8f6fb487,
            0xbc2b6c99d3045b34,
            0x98fb67d6badde1fd,
            0x48841d76a24d2073,
            0xd49891145fe93ae6,
            0xc772b9c8e74d4099,
            0xccf4e7b9907755bb,
            0x9cf47b25d42fd908,
            0x5616a0c347fc445d,
            0xff93b7a7ad1b8a6d,
            0xac2099256b78a77a,
            0x7804a95b02892e1c,
            0x5cf59ca7bfd69776,
            0xa7023502acd3c866,
            0xc76f4982fcf8f37,
            0x51862a5a57ac986e,
            0x38b80ed72b1b1023,
            0x4a291812066a61e1,
            0xcd8a685eff45631,
            0x3f40f708764e4fa5,
            0x8aa0441891285092,
            0x9eff60d71cdf0a9,
            0x4fdd9d56517e2bfa,
            0x1f3c80d74a28bc85,
            0x24617417c064b648,
            0x7ddda1e4385d5088,
            0xf9e132b11dd32a16,
            0xcc957cb8ef66ab99,
            0xd4f206d37cb752c5,
            0x40de343f28ad616b,
            0x8d1f24379068f0e3,
            0x6f31d7947ea21137,
            0x27311f9c32184061,
            0x9eea0664cc78ce5f,
            0x7d4151f6fea9a0da,
            0x454096fa75bd571a,
            0x4fe0f20ecb,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[8],
        nqr.pow(vec![
            0x802f5720d0b25710,
            0x6714f0a258b85c7c,
            0x31394c90afdf16e,
            0xe9d2b0c64f957b19,
            0xe67c0d9c5e7903ee,
            0x3156fdc5443ea8ef,
            0x7c4c50524d88c892,
            0xc99dc8990c0ad244,
            0xd37ababf3649a896,
            0x76fe4b838ff7a20c,
            0xcf69ee2cec728db3,
            0xb83535548e5f41,
            0x371147684ccb0c23,
            0x194f6f4fa500db52,
            0xc4571dc78a4c5374,
            0xe4d46d479999ca97,
            0x76b6785a615a151c,
            0xcceb8bcea7eaf8c1,
            0x80d87a6fbe5ae687,
            0x6a97ddddb85ce85,
            0xd783958f26034204,
            0x7144506f2e2e8590,
            0x948693d377aef166,
            0x8364621ed6f96056,
            0xf021777c4c09ee2d,
            0xc6cf5e746ecd50b,
            0xa2337b7aa22743df,
            0xae753f8bbacab39c,
            0xfc782a9e34d3c1cc,
            0x21b827324fe494d9,
            0x5692ce350ed03b38,
            0xf323a2b3cd0481b0,
            0xe859c97a4ccad2e3,
            0x48434b70381e4503,
            0x46042d62e4132ed8,
            0x48c4d6f56122e2f2,
            0xf87711ab9f5c1af7,
            0xb14b7a054759b469,
            0x8eb0a96993ffa9aa,
            0x9b21fb6fc58b760c,
            0xf3abdd115d2e7d25,
            0xf7beac3d4d12409c,
            0x40a5585cce69bf03,
            0x697881e1ba22d5a8,
            0x3d6c04e6ad373fd9,
            0x849871bf627be886,
            0x550f4b9b71b28ef9,
            0x81d2e0d78,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[9],
        nqr.pow(vec![
            0x4af4accf7de0b977,
            0x742485e21805b4ee,
            0xee388fbc4ac36dec,
            0x1e199da57ad178a,
            0xc27c12b292c6726a,
            0x162e6ed84505b5e8,
            0xe191683f336e09df,
            0x17deb7e8d1e0fce6,
            0xd944f19ad06f5836,
            0x4c5f5e59f6276026,
            0xf1ba9c7c148a38a8,
            0xd205fe2dba72b326,
            0x9a2cf2a4c289824e,
            0x4f47ad512c39e24d,
            0xc5894d984000ea09,
            0x2974c03ff7cf01fa,
            0xfcd243b48cb99a22,
            0x2b5150c9313ac1e8,
            0x9089f37c7fc80eda,
            0x989540cc9a7aea56,
            0x1ab1d4e337e63018,
            0x42b546c30d357e43,
            0x1c6abc04f76233d9,
            0x78b3b8d88bf73e47,
            0x151c4e4c45dc68e6,
            0x519a79c4f54397ed,
            0x93f5b51535a127c5,
            0x5fc51b6f52fa153e,
            0x2e0504f2d4a965c3,
            0xc85bd3a3da52bffe,
            0x98c60957a46a89ef,
            0x48c03b5976b91cae,
            0xc6598040a0a61438,
            0xbf0b49dc255953af,
            0xb78dff905b628ab4,
            0x68140b797ba74ab8,
            0x116cf037991d1143,
            0x2f7fe82e58acb0b8,
            0xc20bf7a8f7be5d45,
            0x86c2905c338d5709,
            0xff13a3ae6c8ace3d,
            0xb6f95e2282d08337,
            0xd49f7b313e9cbf29,
            0xf794517193a1ce8c,
            0x39641fecb596a874,
            0x411c4c4edf462fb3,
            0x3f8cd55c10cf25b4,
            0x2bdd7ea165e860b6,
            0xacd7d2cef4caa193,
            0x6558a1d09a05f96,
            0x1f52b5f5b546fc20,
            0x4ee22a5a8c250c12,
            0xd3a63a54a205b6b3,
            0xd2ff5be8,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[10],
        nqr.pow(vec![
            0xe5953a4f96cdda44,
            0x336b2d734cbc32bb,
            0x3f79bfe3cd7410e,
            0x267ae19aaa0f0332,
            0x85a9c4db78d5c749,
            0x90996b046b5dc7d8,
            0x8945eae9820afc6a,
            0x2644ddea2b036bd,
            0x39898e35ac2e3819,
            0x2574eab095659ab9,
            0x65953d51ac5ea798,
            0xc6b8c7afe6752466,
            0x40e9e993e9286544,
            0x7e0ad34ad9700ea0,
            0xac1015eba2c69222,
            0x24f057a19239b5d8,
            0x2043b48c8a3767eb,
            0x1117c124a75d7ff4,
            0x433cfd1a09fb3ce7,
            0x25b087ce4bcf7fb,
            0xbcee0dc53a3e5bdb,
            0xbffda040cf028735,
            0xf7cf103a25512acc,
            0x31d4ecda673130b9,
            0xea0906dab18461e6,
            0x5a40585a5ac3050d,
            0x803358fc14fd0eda,
            0x3678ca654eada770,
            0x7b91a1293a45e33e,
            0xcd5e5b8ea8530e43,
            0x21ae563ab34da266,
            0xecb00dad60df8894,
            0x77fe53e652facfef,
            0x9b7d1ad0b00244ec,
            0xe695df5ca73f801,
            0x23cdb21feeab0149,
            0x14de113e7ea810d9,
            0x52600cd958dac7e7,
            0xc83392c14667e488,
            0x9f808444bc1717fc,
            0x56facb4bcf7c788f,
            0x8bcad53245fc3ca0,
            0xdef661e83f27d81c,
            0x37d4ebcac9ad87e5,
            0x6fe8b24f5cdb9324,
            0xee08a26c1197654c,
            0xc98b22f65f237e9a,
            0xf54873a908ed3401,
            0x6e1cb951d41f3f3,
            0x290b2250a54e8df6,
            0x7f36d51eb1db669e,
            0xb08c7ed81a6ee43e,
            0x95e1c90fb092f680,
            0x429e4afd0e8b820,
            0x2c14a83ee87d715c,
            0xf37267575cfc8af5,
            0xb99e9afeda3c2c30,
            0x8f0f69da75792d5a,
            0x35074a85a533c73,
            0x156ed119,
        ])
    );
    assert_eq!(
        FROBENIUS_COEFF_FQ12_C1[11],
        nqr.pow(vec![
            0x107db680942de533,
            0x6262b24d2052393b,
            0x6136df824159ebc,
            0xedb052c9970c5deb,
            0xca813aea916c3777,
            0xf49dacb9d76c1788,
            0x624941bd372933bb,
            0xa5e60c2520638331,
            0xb38b661683411074,
            0x1d2c9af4c43d962b,
            0x17d807a0f14aa830,
            0x6e6581a51012c108,
            0x668a537e5b35e6f5,
            0x6c396cf3782dca5d,
            0x33b679d1bff536ed,
            0x736cce41805d90aa,
            0x8a562f369eb680bf,
            0x9f61aa208a11ded8,
            0x43dd89dd94d20f35,
            0xcf84c6610575c10a,
            0x9f318d49cf2fe8e6,
            0xbbc6e5f25a6e434e,
            0x6528c433d11d987b,
            0xffced71cc48c0e8a,
            0x4cbb1474f4cb2a26,
            0x66a035c0b28b7231,
            0xa6f2875faa1a82ae,
            0xdd1ea3deff818b02,
            0xe0cfdf0dcdecf701,
            0x9aefa231f2f6d23,
            0xfb251297efa06746,
            0x5a40d367df985538,
            0x1ea31d69ab506fed,
            0xc64ea8280e89a73f,
            0x969acf9f2d4496f4,
            0xe84c9181ee60c52c,
            0xc60f27fc19fc6ca4,
            0x760b33d850154048,
            0x84f69080f66c8457,
            0xc0192ba0fabf640e,
            0xd2c338765c23a3a8,
            0xa7838c20f02cec6c,
            0xb7cf01d020572877,
            0xd63ffaeba0be200a,
            0xf7492baeb5f041ac,
            0x8602c5212170d117,
            0xad9b2e83a5a42068,
            0x2461829b3ba1083e,
            0x7c34650da5295273,
            0xdc824ba800a8265a,
            0xd18d9b47836af7b2,
            0x3af78945c58cbf4d,
            0x7ed9575b8596906c,
            0x6d0c133895009a66,
            0x53bc1247ea349fe1,
            0x6b3063078d41aa7a,
            0x6184acd8cd880b33,
            0x76f4d15503fd1b96,
            0x7a9afd61eef25746,
            0xce974aadece60609,
            0x88ca59546a8ceafd,
            0x6d29391c41a0ac07,
            0x443843a60e0f46a6,
            0xa1590f62fd2602c7,
            0x536d5b15b514373f,
            0x22d582b,
        ])
    );
}

// needs correct constants
#[ignore]
#[test]
fn test_neg_one() {
    let mut o = Fq::one();
    o.negate();

    assert_eq!(NEGATIVE_ONE, o);
}

#[cfg(test)]
use rand::{Rand, SeedableRng, XorShiftRng};

#[test]
fn test_fq_repr_ordering() {
    use std::cmp::Ordering;

    fn assert_equality(a: FqRepr, b: FqRepr) {
        assert_eq!(a, b);
        assert!(a.cmp(&b) == Ordering::Equal);
    }

    fn assert_lt(a: FqRepr, b: FqRepr) {
        assert!(a < b);
        assert!(b > a);
    }

    assert_equality(
        FqRepr([9999, 9999, 9999, 9999, 9999, 9999]),
        FqRepr([9999, 9999, 9999, 9999, 9999, 9999]),
    );
    assert_equality(
        FqRepr([9999, 9998, 9999, 9999, 9999, 9999]),
        FqRepr([9999, 9998, 9999, 9999, 9999, 9999]),
    );
    assert_equality(
        FqRepr([9999, 9999, 9999, 9997, 9999, 9999]),
        FqRepr([9999, 9999, 9999, 9997, 9999, 9999]),
    );
    assert_lt(
        FqRepr([9999, 9999, 9999, 9997, 9999, 9998]),
        FqRepr([9999, 9999, 9999, 9997, 9999, 9999]),
    );
    assert_lt(
        FqRepr([9999, 9999, 9999, 9997, 9998, 9999]),
        FqRepr([9999, 9999, 9999, 9997, 9999, 9999]),
    );
    assert_lt(
        FqRepr([9, 9999, 9999, 9997, 9998, 9999]),
        FqRepr([9999, 9999, 9999, 9997, 9999, 9999]),
    );
}

#[test]
fn test_fq_repr_from() {
    assert_eq!(FqRepr::from(100), FqRepr([100, 0, 0, 0, 0, 0]));
}

#[test]
fn test_fq_repr_is_odd() {
    assert!(!FqRepr::from(0).is_odd());
    assert!(FqRepr::from(0).is_even());
    assert!(FqRepr::from(1).is_odd());
    assert!(!FqRepr::from(1).is_even());
    assert!(!FqRepr::from(324834872).is_odd());
    assert!(FqRepr::from(324834872).is_even());
    assert!(FqRepr::from(324834873).is_odd());
    assert!(!FqRepr::from(324834873).is_even());
}

#[test]
fn test_fq_repr_is_zero() {
    assert!(FqRepr::from(0).is_zero());
    assert!(!FqRepr::from(1).is_zero());
    assert!(!FqRepr([0, 0, 0, 0, 1, 0]).is_zero());
}

#[test]
fn test_fq_repr_div2() {
    let mut a = FqRepr([
        0x8b0ad39f8dd7482a,
        0x147221c9a7178b69,
        0x54764cb08d8a6aa0,
        0x8519d708e1d83041,
        0x41f82777bd13fdb,
        0xf43944578f9b771b,
    ]);
    a.div2();
    assert_eq!(
        a,
        FqRepr([
            0xc58569cfc6eba415,
            0xa3910e4d38bc5b4,
            0xaa3b265846c53550,
            0xc28ceb8470ec1820,
            0x820fc13bbde89fed,
            0x7a1ca22bc7cdbb8d
        ])
    );
    for _ in 0..10 {
        a.div2();
    }
    assert_eq!(
        a,
        FqRepr([
            0x6d31615a73f1bae9,
            0x54028e443934e2f1,
            0x82a8ec99611b14d,
            0xfb70a33ae11c3b06,
            0xe36083f04eef7a27,
            0x1e87288af1f36e
        ])
    );
    for _ in 0..300 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x7288af1f36ee3608, 0x1e8, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..50 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x7a1ca2, 0x0, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..22 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x1, 0x0, 0x0, 0x0, 0x0, 0x0]));
    a.div2();
    assert!(a.is_zero());
}

#[test]
fn test_fq_repr_shr() {
    let mut a = FqRepr([
        0xaa5cdd6172847ffd,
        0x43242c06aed55287,
        0x9ddd5b312f3dd104,
        0xc5541fd48046b7e7,
        0x16080cf4071e0b05,
        0x1225f2901aea514e,
    ]);
    a.shr(0);
    assert_eq!(
        a,
        FqRepr([
            0xaa5cdd6172847ffd,
            0x43242c06aed55287,
            0x9ddd5b312f3dd104,
            0xc5541fd48046b7e7,
            0x16080cf4071e0b05,
            0x1225f2901aea514e
        ])
    );
    a.shr(1);
    assert_eq!(
        a,
        FqRepr([
            0xd52e6eb0b9423ffe,
            0x21921603576aa943,
            0xceeead98979ee882,
            0xe2aa0fea40235bf3,
            0xb04067a038f0582,
            0x912f9480d7528a7
        ])
    );
    a.shr(50);
    assert_eq!(
        a,
        FqRepr([
            0x8580d5daaa50f54b,
            0xab6625e7ba208864,
            0x83fa9008d6fcf3bb,
            0x19e80e3c160b8aa,
            0xbe52035d4a29c2c1,
            0x244
        ])
    );
    a.shr(130);
    assert_eq!(
        a,
        FqRepr([
            0xa0fea40235bf3cee,
            0x4067a038f0582e2a,
            0x2f9480d7528a70b0,
            0x91,
            0x0,
            0x0
        ])
    );
    a.shr(64);
    assert_eq!(
        a,
        FqRepr([0x4067a038f0582e2a, 0x2f9480d7528a70b0, 0x91, 0x0, 0x0, 0x0])
    );
}

#[test]
fn test_fq_repr_mul2() {
    let mut a = FqRepr::from(23712937547);
    a.mul2();
    assert_eq!(a, FqRepr([0xb0acd6c96, 0x0, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..60 {
        a.mul2();
    }
    assert_eq!(
        a,
        FqRepr([0x6000000000000000, 0xb0acd6c9, 0x0, 0x0, 0x0, 0x0])
    );
    for _ in 0..300 {
        a.mul2();
    }
    assert_eq!(a, FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0xcd6c960000000000]));
    for _ in 0..17 {
        a.mul2();
    }
    assert_eq!(a, FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x2c00000000000000]));
    for _ in 0..6 {
        a.mul2();
    }
    assert!(a.is_zero());
}

#[test]
fn test_fq_repr_num_bits() {
    let mut a = FqRepr::from(0);
    assert_eq!(0, a.num_bits());
    a = FqRepr::from(1);
    for i in 1..385 {
        assert_eq!(i, a.num_bits());
        a.mul2();
    }
    assert_eq!(0, a.num_bits());
}

#[test]
fn test_fq_repr_sub_noborrow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FqRepr([
        0x827a4a08041ebd9,
        0x3c239f3dcc8f0d6b,
        0x9ab46a912d555364,
        0x196936b17b43910b,
        0xad0eb3948a5c34fd,
        0xd56f7b5ab8b5ce8,
    ]);
    t.sub_noborrow(&FqRepr([
        0xc7867917187ca02b,
        0x5d75679d4911ffef,
        0x8c5b3e48b1a71c15,
        0x6a427ae846fd66aa,
        0x7a37e7265ee1eaf9,
        0x7c0577a26f59d5,
    ]));
    assert!(
        t == FqRepr([
            0x40a12b8967c54bae,
            0xdeae37a0837d0d7b,
            0xe592c487bae374e,
            0xaf26bbc934462a61,
            0x32d6cc6e2b7a4a03,
            0xcdaf23e091c0313
        ])
    );

    for _ in 0..1000 {
        let mut a = FqRepr::rand(&mut rng);
        a.0[5] >>= 30;
        let mut b = a;
        for _ in 0..10 {
            b.mul2();
        }
        let mut c = b;
        for _ in 0..10 {
            c.mul2();
        }

        assert!(a < b);
        assert!(b < c);

        let mut csub_ba = c;
        csub_ba.sub_noborrow(&b);
        csub_ba.sub_noborrow(&a);

        let mut csub_ab = c;
        csub_ab.sub_noborrow(&a);
        csub_ab.sub_noborrow(&b);

        assert_eq!(csub_ab, csub_ba);
    }

    // Subtracting q+1 from q should produce -1 (mod 2**384)
    let mut qplusone = FqRepr([
        0xb9feffffffffaaab,
        0x1eabfffeb153ffff,
        0x6730d2a0f6b0f624,
        0x64774b84f38512bf,
        0x4b1ba7b6434bacd7,
        0x1a0111ea397fe69a,
    ]);
    qplusone.sub_noborrow(&FqRepr([
        0xb9feffffffffaaac,
        0x1eabfffeb153ffff,
        0x6730d2a0f6b0f624,
        0x64774b84f38512bf,
        0x4b1ba7b6434bacd7,
        0x1a0111ea397fe69a,
    ]));
    assert_eq!(
        qplusone,
        FqRepr([
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff
        ])
    );
}

#[test]
fn test_fq_repr_add_nocarry() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FqRepr([
        0x827a4a08041ebd9,
        0x3c239f3dcc8f0d6b,
        0x9ab46a912d555364,
        0x196936b17b43910b,
        0xad0eb3948a5c34fd,
        0xd56f7b5ab8b5ce8,
    ]);
    t.add_nocarry(&FqRepr([
        0xc7867917187ca02b,
        0x5d75679d4911ffef,
        0x8c5b3e48b1a71c15,
        0x6a427ae846fd66aa,
        0x7a37e7265ee1eaf9,
        0x7c0577a26f59d5,
    ]));
    assert!(
        t == FqRepr([
            0xcfae1db798be8c04,
            0x999906db15a10d5a,
            0x270fa8d9defc6f79,
            0x83abb199c240f7b6,
            0x27469abae93e1ff6,
            0xdd2fd2d4dfab6be
        ])
    );

    // Test for the associativity of addition.
    for _ in 0..1000 {
        let mut a = FqRepr::rand(&mut rng);
        let mut b = FqRepr::rand(&mut rng);
        let mut c = FqRepr::rand(&mut rng);

        // Unset the first few bits, so that overflow won't occur.
        a.0[5] >>= 3;
        b.0[5] >>= 3;
        c.0[5] >>= 3;

        let mut abc = a;
        abc.add_nocarry(&b);
        abc.add_nocarry(&c);

        let mut acb = a;
        acb.add_nocarry(&c);
        acb.add_nocarry(&b);

        let mut bac = b;
        bac.add_nocarry(&a);
        bac.add_nocarry(&c);

        let mut bca = b;
        bca.add_nocarry(&c);
        bca.add_nocarry(&a);

        let mut cab = c;
        cab.add_nocarry(&a);
        cab.add_nocarry(&b);

        let mut cba = c;
        cba.add_nocarry(&b);
        cba.add_nocarry(&a);

        assert_eq!(abc, acb);
        assert_eq!(abc, bac);
        assert_eq!(abc, bca);
        assert_eq!(abc, cab);
        assert_eq!(abc, cba);
    }

    // Adding 1 to (2^384 - 1) should produce zero
    let mut x = FqRepr([
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
    ]);
    x.add_nocarry(&FqRepr::from(1));
    assert!(x.is_zero());
}

// needs correct constants
#[ignore]
#[test]
fn test_fq_is_valid() {
    let mut a = Fq(MODULUS);
    assert!(!a.is_valid());
    a.0.sub_noborrow(&FqRepr::from(1));
    assert!(a.is_valid());
    assert!(Fq(FqRepr::from(0)).is_valid());
    assert!(Fq(FqRepr([
        0xdf4671abd14dab3e,
        0xe2dc0c9f534fbd33,
        0x31ca6c880cc444a6,
        0x257a67e70ef33359,
        0xf9b29e493f899b36,
        0x17c8be1800b9f059
    ]))
    .is_valid());
    assert!(!Fq(FqRepr([
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff
    ]))
    .is_valid());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        let a = Fq::rand(&mut rng);
        assert!(a.is_valid());
    }
}

#[test]
fn test_fq_add_assign() {
    // Test associativity

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Generate a, b, c and ensure (a + b) + c == a + (b + c).
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);
        let c = Fq::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.add_assign(&b);
        tmp1.add_assign(&c);

        let mut tmp2 = b;
        tmp2.add_assign(&c);
        tmp2.add_assign(&a);

        assert!(tmp1.is_valid());
        assert!(tmp2.is_valid());
        assert_eq!(tmp1, tmp2);
    }
}

#[test]
fn test_fq_sub_assign() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure that (a - b) + (b - a) = 0.
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.sub_assign(&b);

        let mut tmp2 = b;
        tmp2.sub_assign(&a);

        tmp1.add_assign(&tmp2);
        assert!(tmp1.is_zero());
    }
}

#[test]
fn test_fq_mul_assign() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * b) * c = a * (b * c)
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);
        let c = Fq::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.mul_assign(&b);
        tmp1.mul_assign(&c);

        let mut tmp2 = b;
        tmp2.mul_assign(&c);
        tmp2.mul_assign(&a);

        assert_eq!(tmp1, tmp2);
    }

    for _ in 0..1000000 {
        // Ensure that r * (a + b + c) = r*a + r*b + r*c

        let r = Fq::rand(&mut rng);
        let mut a = Fq::rand(&mut rng);
        let mut b = Fq::rand(&mut rng);
        let mut c = Fq::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.add_assign(&b);
        tmp1.add_assign(&c);
        tmp1.mul_assign(&r);

        a.mul_assign(&r);
        b.mul_assign(&r);
        c.mul_assign(&r);

        a.add_assign(&b);
        a.add_assign(&c);

        assert_eq!(tmp1, a);
    }
}

#[test]
fn test_fq_squaring() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * a) = a^2
        let a = Fq::rand(&mut rng);

        let mut tmp = a;
        tmp.square();

        let mut tmp2 = a;
        tmp2.mul_assign(&a);

        assert_eq!(tmp, tmp2);
    }
}

#[test]
fn test_fq_inverse() {
    assert!(Fq::zero().inverse().is_none());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let one = Fq::one();

    for _ in 0..1000 {
        // Ensure that a * a^-1 = 1
        let mut a = Fq::rand(&mut rng);
        let ainv = a.inverse().unwrap();
        a.mul_assign(&ainv);
        assert_eq!(a, one);
    }
}

#[test]
fn test_fq_double() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure doubling a is equivalent to adding a to itself.
        let mut a = Fq::rand(&mut rng);
        let mut b = a;
        b.add_assign(&a);
        a.double();
        assert_eq!(a, b);
    }
}

#[test]
fn test_fq_negate() {
    {
        let mut a = Fq::zero();
        a.negate();

        assert!(a.is_zero());
    }

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure (a - (-a)) = 0.
        let mut a = Fq::rand(&mut rng);
        let mut b = a;
        b.negate();
        a.add_assign(&b);

        assert!(a.is_zero());
    }
}

#[test]
fn test_fq_pow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for i in 0..1000 {
        // Exponentiate by various small numbers and ensure it consists with repeated
        // multiplication.
        let a = Fq::rand(&mut rng);
        let target = a.pow(&[i]);
        let mut c = Fq::one();
        for _ in 0..i {
            c.mul_assign(&a);
        }
        assert_eq!(c, target);
    }

    for _ in 0..1000 {
        // Exponentiating by the modulus should have no effect in a prime field.
        let a = Fq::rand(&mut rng);

        assert_eq!(a, a.pow(Fq::char()));
    }
}

#[test]
fn test_fq_sqrt() {
    use ff::SqrtField;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    assert_eq!(Fq::zero().sqrt().unwrap(), Fq::zero());

    for _ in 0..1000 {
        // Ensure sqrt(a^2) = a or -a
        let a = Fq::rand(&mut rng);
        let mut nega = a;
        nega.negate();
        let mut b = a;
        b.square();

        let b = b.sqrt().unwrap();

        assert!(a == b || nega == b);
    }

    for _ in 0..1000 {
        // Ensure sqrt(a)^2 = a for random a
        let a = Fq::rand(&mut rng);

        if let Some(mut tmp) = a.sqrt() {
            tmp.square();

            assert_eq!(a, tmp);
        }
    }
}

// needs correct constants
#[ignore]
#[test]
fn test_fq_from_into_repr() {
    // q + 1 should not be in the field
    assert!(Fq::from_repr(FqRepr([
        0xb9feffffffffaaac,
        0x1eabfffeb153ffff,
        0x6730d2a0f6b0f624,
        0x64774b84f38512bf,
        0x4b1ba7b6434bacd7,
        0x1a0111ea397fe69a
    ]))
    .is_err());

    // q should not be in the field
    assert!(Fq::from_repr(Fq::char()).is_err());

    // Multiply some arbitrary representations to see if the result is as expected.
    let a = FqRepr([
        0x4a49dad4ff6cde2d,
        0xac62a82a8f51cd50,
        0x2b1f41ab9f36d640,
        0x908a387f480735f1,
        0xae30740c08a875d7,
        0x6c80918a365ef78,
    ]);
    let mut a_fq = Fq::from_repr(a).unwrap();
    let b = FqRepr([
        0xbba57917c32f0cf0,
        0xe7f878cf87f05e5d,
        0x9498b4292fd27459,
        0xd59fd94ee4572cfa,
        0x1f607186d5bb0059,
        0xb13955f5ac7f6a3,
    ]);
    let b_fq = Fq::from_repr(b).unwrap();
    let c = FqRepr([
        0xf5f70713b717914c,
        0x355ea5ac64cbbab1,
        0xce60dd43417ec960,
        0xf16b9d77b0ad7d10,
        0xa44c204c1de7cdb7,
        0x1684487772bc9a5a,
    ]);
    a_fq.mul_assign(&b_fq);
    assert_eq!(a_fq.into_repr(), c);

    // Zero should be in the field.
    assert!(Fq::from_repr(FqRepr::from(0)).unwrap().is_zero());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Try to turn Fq elements into representations and back again, and compare.
        let a = Fq::rand(&mut rng);
        let a_repr = a.into_repr();
        let b_repr = FqRepr::from(a);
        assert_eq!(a_repr, b_repr);
        let a_again = Fq::from_repr(a_repr).unwrap();

        assert_eq!(a, a_again);
    }
}

#[test]
fn test_fq_repr_display() {
    assert_eq!(
        format!("{}", FqRepr([0xa956babf9301ea24, 0x39a8f184f3535c7b, 0xb38d35b3f6779585, 0x676cc4eef4c46f2c, 0xb1d4aad87651e694, 0x1947f0d5f4fe325a])),
        "0x1947f0d5f4fe325ab1d4aad87651e694676cc4eef4c46f2cb38d35b3f677958539a8f184f3535c7ba956babf9301ea24".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0xb4171485fd8622dd, 0x864229a6edec7ec5, 0xc57f7bdcf8dfb707, 0x6db7ff0ecea4584a, 0xf8d8578c4a57132d, 0x6eb66d42d9fcaaa])),
        "0x06eb66d42d9fcaaaf8d8578c4a57132d6db7ff0ecea4584ac57f7bdcf8dfb707864229a6edec7ec5b4171485fd8622dd".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff])),
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0, 0, 0, 0, 0, 0])),
        "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
}

#[test]
fn test_fq_display() {
    assert_eq!(
        format!("{}", Fq::from_repr(FqRepr([0xa956babf9301ea24, 0x39a8f184f3535c7b, 0xb38d35b3f6779585, 0x676cc4eef4c46f2c, 0xb1d4aad87651e694, 0x1947f0d5f4fe32])).unwrap()),
        "Fq(0x001947f0d5f4fe32b1d4aad87651e694676cc4eef4c46f2cb38d35b3f677958539a8f184f3535c7ba956babf9301ea24)".to_string()
    );
    assert_eq!(
        format!("{}", Fq::from_repr(FqRepr([0xe28e79396ac2bbf8, 0x413f6f7f06ea87eb, 0xa4b62af4a792a689, 0xb7f89f88f59c1dc5, 0x9a551859b1e43a9a, 0x6c9f5a1060de9])).unwrap()),
        "Fq(0x0006c9f5a1060de99a551859b1e43a9ab7f89f88f59c1dc5a4b62af4a792a689413f6f7f06ea87ebe28e79396ac2bbf8)".to_string()
    );
}

#[test]
fn test_fq_num_bits() {
    assert_eq!(Fq::NUM_BITS, 377);
    assert_eq!(Fq::CAPACITY, 376);
}

#[test]
fn test_fq_root_of_unity() {
    use ff::SqrtField;

    assert_eq!(Fq::S, 46);
    assert_eq!(
        Fq::multiplicative_generator().pow([
            0x7510c00000021423,
            0x88bee82520005c2d,
            0x67cc03d44e3c7bcd,
            0x1701b28524ec688b,
            0xe9185f1443ab18ec,
            0x6b8
        ]),
        Fq::root_of_unity()
    );
    assert_eq!(Fq::root_of_unity().pow([1 << Fq::S]), Fq::one());
    assert!(Fq::multiplicative_generator().sqrt().is_none());
}

#[test]
fn fq_field_tests() {
    ::tests::field::random_field_tests::<Fq>();
    ::tests::field::random_sqrt_tests::<Fq>();
    ::tests::field::random_frobenius_tests::<Fq, _>(Fq::char(), 13);
    ::tests::field::from_str_tests::<Fq>();
}

#[test]
fn test_fq_ordering() {
    // FqRepr's ordering is well-tested, but we still need to make sure the Fq
    // elements aren't being compared in Montgomery form.
    for i in 0..100 {
        assert!(
            Fq::from_repr(FqRepr::from(i + 1)).unwrap() > Fq::from_repr(FqRepr::from(i)).unwrap()
        );
    }
}

#[test]
fn fq_repr_tests() {
    ::tests::repr::random_repr_tests::<FqRepr>();
}

#[test]
fn test_fq_legendre() {
    use ff::LegendreSymbol::*;
    use ff::SqrtField;

    assert_eq!(QuadraticResidue, Fq::one().legendre());
    assert_eq!(Zero, Fq::zero().legendre());

    assert_eq!(
        QuadraticResidue,
        Fq::from_repr(FqRepr::from(4)).unwrap().legendre()
    );

    assert_eq!(
        QuadraticNonResidue,
        Fq::from_repr(FqRepr::from(5)).unwrap().legendre()
    );
}
