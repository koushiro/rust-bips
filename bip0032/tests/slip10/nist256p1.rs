#![cfg(all(feature = "slip10", feature = "p256"))]

use bip0032::curve::nist256p1::{Nist256p1Curve, P256Backend};

mod common;
use common::{Case, assert_nonhardened_private_case, assert_nonhardened_public_case};

type Curve = Nist256p1Curve<P256Backend>;

#[test]
fn slip10_nist256p1_vector_1() {
    let seed = "000102030405060708090a0b0c0d0e0f";

    let cases = [
        Case {
            path: "m",
            fingerprint: "00000000",
            chain_code: "beeb672fe4621673f722f38529c07392fecaa61015c80c34f29ce8b41b3cb6ea",
            private: "612091aaa12e22dd2abef664f8a01a82cae99ad7441b7ef8110424915c268bc2",
            public: "0266874dc6ade47b3ecd096745ca09bcd29638dd52c2c12117b11ed3e458cfa9e8",
        },
        Case {
            path: "m/0H",
            fingerprint: "be6105b5",
            chain_code: "3460cea53e6a6bb5fb391eeef3237ffd8724bf0a40e94943c98b83825342ee11",
            private: "6939694369114c67917a182c59ddb8cafc3004e63ca5d3b84403ba8613debc0c",
            public: "0384610f5ecffe8fda089363a41f56a5c7ffc1d81b59a612d0d649b2d22355590c",
        },
        Case {
            path: "m/0H/1",
            fingerprint: "9b02312f",
            chain_code: "4187afff1aafa8445010097fb99d23aee9f599450c7bd140b6826ac22ba21d0c",
            private: "284e9d38d07d21e4e281b645089a94f4cf5a5a81369acf151a1c3a57f18b2129",
            public: "03526c63f8d0b4bbbf9c80df553fe66742df4676b241dabefdef67733e070f6844",
        },
        Case {
            path: "m/0H/1/2H",
            fingerprint: "b98005c1",
            chain_code: "98c7514f562e64e74170cc3cf304ee1ce54d6b6da4f880f313e8204c2a185318",
            private: "694596e8a54f252c960eb771a3c41e7e32496d03b954aeb90f61635b8e092aa7",
            public: "0359cf160040778a4b14c5f4d7b76e327ccc8c4a6086dd9451b7482b5a4972dda0",
        },
        Case {
            path: "m/0H/1/2H/2",
            fingerprint: "0e9f3274",
            chain_code: "ba96f776a5c3907d7fd48bde5620ee374d4acfd540378476019eab70790c63a0",
            private: "5996c37fd3dd2679039b23ed6f70b506c6b56b3cb5e424681fb0fa64caf82aaa",
            public: "029f871f4cb9e1c97f9f4de9ccd0d4a2f2a171110c61178f84430062230833ff20",
        },
        Case {
            path: "m/0H/1/2H/2/1000000000",
            fingerprint: "8b2b5c4b",
            chain_code: "b9b7b82d326bb9cb5b5b121066feea4eb93d5241103c9e7a18aad40f1dde8059",
            private: "21c4f269ef0a5fd1badf47eeacebeeaa3de22eb8e5b0adcd0f27dd99d34d0119",
            public: "02216cd26d31147f72427a453c443ed2cde8a1e53c9cc44e5ddf739725413fe3f4",
        },
    ];

    for case in &cases {
        assert_nonhardened_private_case::<Curve>(seed, case);
    }
}

#[test]
fn slip10_nist256p1_vector_2_private_and_public() {
    let seed = "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542";

    let cases = [
        Case {
            path: "m",
            fingerprint: "00000000",
            chain_code: "96cd4465a9644e31528eda3592aa35eb39a9527769ce1855beafc1b81055e75d",
            private: "eaa31c2e46ca2962227cf21d73a7ef0ce8b31c756897521eb6c7b39796633357",
            public: "02c9e16154474b3ed5b38218bb0463e008f89ee03e62d22fdcc8014beab25b48fa",
        },
        Case {
            path: "m/0",
            fingerprint: "607f628f",
            chain_code: "84e9c258bb8557a40e0d041115b376dd55eda99c0042ce29e81ebe4efed9b86a",
            private: "d7d065f63a62624888500cdb4f88b6d59c2927fee9e6d0cdff9cad555884df6e",
            public: "039b6df4bece7b6c81e2adfeea4bcf5c8c8a6e40ea7ffa3cf6e8494c61a1fc82cc",
        },
        Case {
            path: "m/0/2147483647H",
            fingerprint: "946d2a54",
            chain_code: "f235b2bc5c04606ca9c30027a84f353acf4e4683edbd11f635d0dcc1cd106ea6",
            private: "96d2ec9316746a75e7793684ed01e3d51194d81a42a3276858a5b7376d4b94b9",
            public: "02f89c5deb1cae4fedc9905f98ae6cbf6cbab120d8cb85d5bd9a91a72f4c068c76",
        },
        Case {
            path: "m/0/2147483647H/1",
            fingerprint: "218182d8",
            chain_code: "7c0b833106235e452eba79d2bdd58d4086e663bc8cc55e9773d2b5eeda313f3b",
            private: "974f9096ea6873a915910e82b29d7c338542ccde39d2064d1cc228f371542bbc",
            public: "03abe0ad54c97c1d654c1852dfdc32d6d3e487e75fa16f0fd6304b9ceae4220c64",
        },
        Case {
            path: "m/0/2147483647H/1/2147483646H",
            fingerprint: "931223e4",
            chain_code: "5794e616eadaf33413aa309318a26ee0fd5163b70466de7a4512fd4b1a5c9e6a",
            private: "da29649bbfaff095cd43819eda9a7be74236539a29094cd8336b07ed8d4eff63",
            public: "03cb8cb067d248691808cd6b5a5a06b48e34ebac4d965cba33e6dc46fe13d9b933",
        },
        Case {
            path: "m/0/2147483647H/1/2147483646H/2",
            fingerprint: "956c4629",
            chain_code: "3bfb29ee8ac4484f09db09c2079b520ea5616df7820f071a20320366fbe226a7",
            private: "bb0a77ba01cc31d77205d51d08bd313b979a71ef4de9b062f8958297e746bd67",
            public: "020ee02e18967237cf62672983b253ee62fa4dd431f8243bfeccdf39dbe181387f",
        },
    ];

    for case in &cases {
        assert_nonhardened_private_case::<Curve>(seed, case);
    }

    assert_nonhardened_public_case::<Curve>(seed, &cases[1]);
}

#[test]
fn slip10_nist256p1_derivation_retry() {
    let seed = "000102030405060708090a0b0c0d0e0f";

    let cases = [
        Case {
            path: "m",
            fingerprint: "00000000",
            chain_code: "beeb672fe4621673f722f38529c07392fecaa61015c80c34f29ce8b41b3cb6ea",
            private: "612091aaa12e22dd2abef664f8a01a82cae99ad7441b7ef8110424915c268bc2",
            public: "0266874dc6ade47b3ecd096745ca09bcd29638dd52c2c12117b11ed3e458cfa9e8",
        },
        Case {
            path: "m/28578H",
            fingerprint: "be6105b5",
            chain_code: "e94c8ebe30c2250a14713212f6449b20f3329105ea15b652ca5bdfc68f6c65c2",
            private: "06f0db126f023755d0b8d86d4591718a5210dd8d024e3e14b6159d63f53aa669",
            public: "02519b5554a4872e8c9c1c847115363051ec43e93400e030ba3c36b52a3e70a5b7",
        },
        Case {
            path: "m/28578H/33941",
            fingerprint: "3e2b7bc6",
            chain_code: "9e87fe95031f14736774cd82f25fd885065cb7c358c1edf813c72af535e83071",
            private: "092154eed4af83e078ff9b84322015aefe5769e31270f62c3f66c33888335f3a",
            public: "0235bfee614c0d5b2cae260000bb1d0d84b270099ad790022c1ae0b2e782efe120",
        },
    ];

    for case in &cases {
        assert_nonhardened_private_case::<Curve>(seed, case);
    }
}

#[test]
fn slip10_nist256p1_seed_retry() {
    let seed = "a7305bc8df8d0951f0cb224c0e95d7707cbdf2c6ce7e8d481fec69c7ff5e9446";

    let case = Case {
        path: "m",
        fingerprint: "00000000",
        chain_code: "7762f9729fed06121fd13f326884c82f59aa95c57ac492ce8c9654e60efd130c",
        private: "3b8c18469a4634517d6d0b65448f8e6c62091b45540a1743c5846be55d47d88f",
        public: "0383619fadcde31063d8c5cb00dbfe1713f3e6fa169d8541a798752a1c1ca0cb20",
    };

    assert_nonhardened_private_case::<Curve>(seed, &case);
}
