use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use assert_cmd::cargo::cargo_bin_cmd;
use bip0032::{
    ExtendedPrivateKey, Version,
    curve::secp256k1::{K256Backend, Secp256k1Curve},
};
use bip0039::{AnyMnemonic, BuiltInLanguage};
use serde_json::Value;

// Published BIP32 vector 1 and BIP39 zero-entropy mnemonic; never use for funds.
const SEED: &str = "000102030405060708090a0b0c0d0e0f";
const ROOT_PRIVATE: &str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";
const ROOT_PUBLIC: &str = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
const CHILD_PRIVATE: &str = "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7";
const CHILD_PUBLIC: &str = "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw";
const GRANDCHILD_PRIVATE: &str = "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs";
const GRANDCHILD_PUBLIC: &str = "xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ";
const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

fn bli() -> assert_cmd::Command {
    let mut command = cargo_bin_cmd!("bli");
    command.timeout(Duration::from_secs(30));
    command
}

fn run(args: &[&str], input: &[u8]) -> Output {
    // Use raw output for secret-bearing cases: assert_cmd's assertion failures include I/O.
    bli().args(args).write_stdin(input).output().expect("failed to execute bli")
}

fn success(args: &[&str], input: &str) -> Value {
    let output = run(args, input.as_bytes());
    // Do not dump generated secret material into test failure logs.
    assert!(output.status.success(), "command failed");
    assert!(output.stderr.is_empty(), "unexpected diagnostic output");
    assert!(output.stdout.ends_with(b"\n"));
    assert!(!output.stdout.ends_with(b"\n\n"));
    serde_json::from_slice(&output.stdout).unwrap_or_else(|_| panic!("output was not valid JSON"))
}

fn failure(args: &[&str], input: &str, code: i32, message: &str) {
    let output = run(args, input.as_bytes());
    assert_eq!(output.status.code(), Some(code));
    assert!(output.stdout.is_empty());
    let diagnostic = String::from_utf8(output.stderr).unwrap();
    assert!(diagnostic.contains(message), "missing expected diagnostic");
    if !input.trim().is_empty() {
        assert!(!diagnostic.contains(input.trim()), "input was echoed in diagnostic");
    }
}

struct InputFile(PathBuf);

impl InputFile {
    fn new(contents: &[u8]) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "bli-test-{}-{}.txt",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
        file.write_all(contents).unwrap();
        Self(path)
    }

    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl Drop for InputFile {
    fn drop(&mut self) {
        fs::remove_file(&self.0).unwrap();
    }
}

fn check_generated(value: &Value, language: BuiltInLanguage, words: usize, passphrase: &str) {
    assert_eq!(value.as_object().unwrap().len(), 6);
    assert_eq!(value["words"], words);
    assert!(value["passphrase"] == passphrase);
    let mnemonic = AnyMnemonic::from_phrase(language, value["mnemonic"].as_str().unwrap()).unwrap();
    assert_eq!(mnemonic.phrase().split_whitespace().count(), words);
    assert_eq!(mnemonic.entropy().len(), words / 3 * 4);
    assert!(value["entropy"] == const_hex::encode_prefixed(mnemonic.entropy()));
    assert!(value["seed"] == const_hex::encode_prefixed(mnemonic.to_seed(passphrase)));
}

#[test]
fn help_aliases_and_version() {
    let assertion = bli().arg("--help").assert().success().stderr("");
    let help = std::str::from_utf8(&assertion.get_output().stdout).unwrap();
    for word in ["generate", "gen", "inspect", "info", "derive"] {
        assert!(help.contains(word));
    }
    bli()
        .arg("--version")
        .assert()
        .success()
        .stdout(concat!("bli ", env!("CARGO_PKG_VERSION"), "\n"))
        .stderr("");
    assert_eq!(success(&["info", "-o", "json"], MNEMONIC)["type"], "mnemonic");
    check_generated(&success(&["gen", "-o", "json"], ""), BuiltInLanguage::English, 12, "");
}

#[test]
fn output_is_scoped_to_each_subcommand() {
    let root_help = bli().arg("--help").assert().success().stderr("");
    let root_help = std::str::from_utf8(&root_help.get_output().stdout).unwrap();
    assert!(!root_help.contains("--output"));

    for command in ["generate", "gen", "inspect", "info", "derive"] {
        let help = bli().args([command, "--help"]).assert().success().stderr("");
        let help = std::str::from_utf8(&help.get_output().stdout).unwrap();
        assert!(help.contains("-o, --output"));
        for option in ["--output", "-o"] {
            failure(&[option, "json", command], "", 2, option);
        }
    }
}

#[test]
fn invalid_arguments_are_usage_errors() {
    for args in [
        vec!["generate", "--words", "13"],
        vec!["generate", "--lang", "unknown"],
        vec!["generate", "--passphrase-prompt", "--passphrase-file", "-"],
        vec!["derive"],
        vec!["derive", "--path", "m//1"],
        vec!["derive", "--path", "m/2147483648"],
        vec!["derive", "--path", "m", "--private"],
        vec!["derive", "--path", "m", "--from", "seed"],
        vec!["inspect", "--output", "yaml"],
    ] {
        failure(&args, "", 2, "error:");
    }
}

#[test]
fn runtime_usage_errors_keep_their_exit_code_and_single_prefix() {
    for format in ["text", "json"] {
        for (args, input, message) in [
            (vec!["derive", "--path", "m//1"], "", "invalid derivation path"),
            (
                vec!["derive", "--path", "m", "--network", "mainnet"],
                ROOT_PRIVATE,
                "--network only applies to seed input",
            ),
            (
                vec!["inspect", "--lang", "english"],
                ROOT_PUBLIC,
                "--lang only applies to mnemonic input",
            ),
        ] {
            let mut args = args;
            args.extend(["--output", format]);
            let output = run(&args, input.as_bytes());
            assert_eq!(output.status.code(), Some(2));
            assert!(output.stdout.is_empty());
            let diagnostic = String::from_utf8(output.stderr).unwrap();
            assert!(diagnostic.contains(message));
            assert_eq!(diagnostic.matches("error:").count(), 1);
            if !input.is_empty() {
                assert!(!diagnostic.contains(input), "input was echoed in diagnostic");
            }
        }
    }
}

#[test]
fn generate_supports_all_word_counts() {
    for words in [12, 15, 18, 21, 24] {
        let value = success(&["generate", "--output", "json", "-w", &words.to_string()], "");
        assert_eq!(value["language"], "english");
        check_generated(&value, BuiltInLanguage::English, words, "");
    }
}

#[test]
fn generate_supports_all_languages() {
    for (name, language) in [
        ("english", BuiltInLanguage::English),
        ("chinese-simplified", BuiltInLanguage::ChineseSimplified),
        ("chinese-traditional", BuiltInLanguage::ChineseTraditional),
        ("czech", BuiltInLanguage::Czech),
        ("french", BuiltInLanguage::French),
        ("italian", BuiltInLanguage::Italian),
        ("japanese", BuiltInLanguage::Japanese),
        ("korean", BuiltInLanguage::Korean),
        ("portuguese", BuiltInLanguage::Portuguese),
        ("spanish", BuiltInLanguage::Spanish),
    ] {
        let value = success(&["gen", "-l", name, "--output", "json"], "");
        assert_eq!(value["language"], name);
        check_generated(&value, language, 12, "");
        let inspected = success(
            &["inspect", "--output", "json", "--lang", name],
            value["mnemonic"].as_str().unwrap(),
        );
        assert_eq!(inspected["languages"], serde_json::json!([name]));
    }
}

#[test]
fn passphrase_file_and_stdin_preserve_meaningful_whitespace() {
    for (transport, expected) in [
        ("TREZOR", "TREZOR"),
        ("  TREZOR  \r\n", "  TREZOR  "),
        ("TREZOR\n\n", "TREZOR\n"),
        ("TREZOR\r", "TREZOR\r"),
        ("㍍ガバヴァぱばぐゞちぢ十人十色\n", "㍍ガバヴァぱばぐゞちぢ十人十色"),
        ("\n", ""),
    ] {
        let value = success(&["gen", "--passphrase-file", "-", "--output", "json"], transport);
        check_generated(&value, BuiltInLanguage::English, 12, expected);
    }
    let file = InputFile::new(b"  TREZOR  \r\n");
    let value =
        success(&["gen", "--passphrase-file", file.path(), "--output", "json"], "ignored stdin");
    check_generated(&value, BuiltInLanguage::English, 12, "  TREZOR  ");
}

#[test]
fn text_generation_has_labeled_outputs_without_passphrase() {
    let output = run(&["gen", "--passphrase-file", "-"], b"TREZOR");
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(text.lines().count(), 5);
    assert!(text.ends_with("words: 12\n"));
    for label in ["mnemonic:", "entropy:", "seed:", "language:", "words:"] {
        assert!(text.contains(label));
    }
    assert!(!text.contains("passphrase:"));
    assert!(!text.contains("TREZOR"));
}

#[test]
fn text_output_layout_is_stable() {
    let cases = [
        (vec!["inspect"], MNEMONIC, "Valid mnemonic\nwords: 12\nlanguages: english\n".to_owned()),
        (
            vec!["inspect"],
            ROOT_PUBLIC,
            concat!(
                "Valid extended public key\n",
                "network: mainnet\n",
                "version: xpub (0x0488B21E)\n",
                "depth: 0\n",
                "child: 0\n",
                "parent fingerprint: 00000000\n",
            )
            .to_owned(),
        ),
        (
            vec!["derive", "--path", "m"],
            SEED,
            format!("private: {ROOT_PRIVATE}\npublic: {ROOT_PUBLIC}\n"),
        ),
        (vec!["derive", "--path", "1"], CHILD_PUBLIC, format!("public: {GRANDCHILD_PUBLIC}\n")),
    ];
    for (args, input, expected) in cases {
        for option in [None, Some("--output"), Some("-o")] {
            let mut args = args.clone();
            if let Some(option) = option {
                args.extend([option, "text"]);
            }
            let output = run(&args, input.as_bytes());
            assert!(output.status.success());
            assert!(output.stderr.is_empty());
            assert!(output.stdout == expected.as_bytes(), "text output layout changed");
        }
    }
}

#[test]
fn inspect_mnemonic_metadata_and_ambiguous_language() {
    let value = success(&["inspect", "--output", "json"], MNEMONIC);
    assert_eq!(
        value,
        serde_json::json!({"type": "mnemonic", "words": 12, "languages": ["english"]})
    );
    let chinese = "的 的 的 的 的 的 的 的 的 的 的 在";
    let value = success(&["info", "--output", "json"], chinese);
    assert_eq!(
        value["languages"],
        serde_json::json!(["chinese-simplified", "chinese-traditional"])
    );
    let value = success(&["info", "-l", "chinese-traditional", "--output", "json"], chinese);
    assert_eq!(value["languages"], serde_json::json!(["chinese-traditional"]));
    let output = run(&["inspect"], MNEMONIC.as_bytes());
    assert!(output.status.success());
    assert!(!String::from_utf8(output.stdout).unwrap().contains(MNEMONIC));
}

#[test]
fn inspect_rejects_bad_mnemonics_without_echoing_words() {
    failure(&["inspect"], "abandon about", 1, "12, 15");
    let invalid_checksum = ["abandon"; 12].join(" ");
    let unknown_word = MNEMONIC.replace("about", "secret-invalid-word");
    for format in ["text", "json"] {
        for language in [None, Some("english")] {
            let mut args = vec!["inspect", "--output", format];
            if let Some(language) = language {
                args.extend(["--lang", language]);
            }
            failure(&args, &invalid_checksum, 1, "checksum");
            let output = run(&args, unknown_word.as_bytes());
            assert_eq!(output.status.code(), Some(1));
            assert!(output.stdout.is_empty());
            let diagnostic = String::from_utf8(output.stderr).unwrap();
            assert!(diagnostic.contains("invalid mnemonic"));
            assert!(!diagnostic.contains("secret-invalid-word"));
            assert!(!diagnostic.contains("abandon"));
        }
    }
    failure(&["inspect"], SEED, 1, "invalid extended key");
}

#[test]
fn inspect_extended_key_metadata() {
    for (input, key_type, version) in
        [(CHILD_PRIVATE, "private", "xprv"), (CHILD_PUBLIC, "public", "xpub")]
    {
        let value = success(&["inspect", "--output", "json"], input);
        assert_eq!(value["type"], "extended-key");
        assert_eq!(value["key_type"], key_type);
        assert_eq!(value["version"], version);
        assert_eq!(value["network"], "mainnet");
        assert_eq!(value["depth"], 1);
        assert_eq!(value["child_number"], 1_u32 << 31);
        assert_eq!(value["parent_fingerprint"], "3442193e");
        assert!(!value.to_string().contains(input));
    }
    failure(&["inspect", "--lang", "english"], ROOT_PUBLIC, 2, "--lang only");
}

#[test]
fn inspect_and_derive_validate_key_payloads() {
    // BIP32 vector 5: bad checksum, invalid scalar, invalid curve point, bad root metadata.
    for input in [
        "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHL",
        "xprv9s21ZrQH143K24Mfq5zL5MhWK9hUhhGbd45hLXo2Pq2oqzMMo63oStZzF93Y5wvzdUayhgkkFoicQZcP3y52uPPxFnfoLZB21Teqt1VvEHx",
        "xpub661MyMwAqRbcEYS8w7XLSVeEsBXy79zSzH1J8vCdxAZningWLdN3zgtU6Q5JXayek4PRsn35jii4veMimro1xefsM58PgBMrvdYre8QyULY",
        "xpub661no6RGEX3uJkY4bNnPcw4URcQTrSibUZ4NqJEw5eBkv7ovTwgiT91XX27VbEXGENhYRCf7hyEbWrR3FewATdCEebj6znwMfQkhRYHRLpJ",
    ] {
        failure(&["inspect"], input, 1, "invalid extended");
        failure(&["derive", "-p", "0"], input, 1, "invalid extended");
    }
}

#[test]
fn derive_seed_and_private_inputs_match_bip32_vector() {
    let prefixed_seed = format!("0x{SEED}");
    for (path, private, public) in [
        ("m", ROOT_PRIVATE, ROOT_PUBLIC),
        ("m/0'", CHILD_PRIVATE, CHILD_PUBLIC),
        ("m/0h", CHILD_PRIVATE, CHILD_PUBLIC),
        ("m/0H/1", GRANDCHILD_PRIVATE, GRANDCHILD_PUBLIC),
    ] {
        for input in [SEED, &prefixed_seed, ROOT_PRIVATE] {
            let value = success(&["derive", "--path", path, "--output", "json"], input);
            assert_eq!(value.as_object().unwrap().len(), 2);
            assert!(value["private_key"] == private);
            assert!(value["public_key"] == public);
        }
    }
    let value = success(&["derive", "-p", "1", "--output", "json"], CHILD_PRIVATE);
    assert!(value["private_key"] == GRANDCHILD_PRIVATE);
    assert!(value["public_key"] == GRANDCHILD_PUBLIC);
}

#[test]
fn generated_seed_can_be_derived() {
    type Key = ExtendedPrivateKey<Secp256k1Curve<K256Backend>>;
    let passphrase = "TREZOR";
    let generated = success(&["gen", "--passphrase-file", "-", "--output", "json"], passphrase);
    let mnemonic =
        AnyMnemonic::from_phrase(BuiltInLanguage::English, generated["mnemonic"].as_str().unwrap())
            .unwrap();
    let expected = Key::new(&mnemonic.to_seed(passphrase)).unwrap();
    let derived =
        success(&["derive", "-p", "m", "-o", "json"], generated["seed"].as_str().unwrap());
    assert!(derived["private_key"] == expected.encode_with(Version::XPRV).unwrap().to_string());
    assert!(
        derived["public_key"]
            == expected.public_key().encode_with(Version::XPUB).unwrap().to_string()
    );
}

#[test]
fn derive_public_input_only_outputs_public_children() {
    let value = success(&["derive", "-p", "1", "--output", "json"], CHILD_PUBLIC);
    assert!(value["private_key"].is_null());
    assert!(value["public_key"] == GRANDCHILD_PUBLIC);
    let output = run(&["derive", "-p", "1"], CHILD_PUBLIC.as_bytes());
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.starts_with("public: "));
    assert!(!text.contains("private:"));
    failure(&["derive", "-p", "0'"], ROOT_PUBLIC, 1, "hardened");
}

#[test]
fn derive_checks_path_origin_and_depth() {
    for key in [CHILD_PRIVATE, CHILD_PUBLIC] {
        failure(&["derive", "-p", "m/1"], key, 1, "absolute paths require a master key");
    }
    let too_deep = vec!["0"; 256].join("/");
    failure(&["derive", "-p", &too_deep], SEED, 1, "depth of 255");
    let at_limit = vec!["0"; 255].join("/");
    // A non-root input must also include its existing depth in the limit check.
    failure(&["derive", "-p", &at_limit], CHILD_PRIVATE, 1, "depth of 255");

    let value = success(&["derive", "-p", &at_limit, "--output", "json"], SEED);
    for field in ["private_key", "public_key"] {
        let key = value[field].as_str().unwrap();
        let metadata = success(&["inspect", "--output", "json"], key);
        assert_eq!(metadata["depth"], 255);
        failure(&["derive", "-p", "0"], key, 1, "depth of 255");
    }
}

#[test]
fn unknown_versions_are_not_assigned_an_arbitrary_network() {
    type Key = ExtendedPrivateKey<Secp256k1Curve<K256Backend>>;
    let key = ROOT_PRIVATE.parse::<Key>().unwrap();
    let private = key.encode_with(Version::private(0x12345678)).unwrap().to_string();
    let public = key.public_key().encode_with(Version::public(0x12345679)).unwrap().to_string();
    for input in [&private, &public] {
        let value = success(&["inspect", "--output", "json"], input);
        assert!(value["network"].is_null());
        assert!(value["version"].is_null());
    }
    failure(&["derive", "-p", "0"], &private, 1, "cannot determine the public version");
    let value = success(&["derive", "-p", "0", "--output", "json"], &public);
    assert!(value["private_key"].is_null());
    let metadata = success(&["inspect", "--output", "json"], value["public_key"].as_str().unwrap());
    assert_eq!(metadata["version_bytes"], "0x12345679");
}

#[test]
fn closed_output_pipe_is_success_for_both_formats() {
    // This case needs direct access to the pipe handle, which assert_cmd captures internally.
    for format in ["text", "json"] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_bli"))
            .args(["gen", "--output", format])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        drop(child.stdout.take());
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn derive_network_and_version_preservation() {
    let value = success(&["derive", "-p", "m", "--network", "testnet", "--output", "json"], SEED);
    assert!(value["private_key"].as_str().unwrap().starts_with("tprv"));
    assert!(value["public_key"].as_str().unwrap().starts_with("tpub"));
    let metadata = success(&["inspect", "--output", "json"], value["public_key"].as_str().unwrap());
    assert_eq!(metadata["network"], "testnet");
    let child =
        success(&["derive", "-p", "0", "--output", "json"], value["private_key"].as_str().unwrap());
    assert!(child["private_key"].as_str().unwrap().starts_with("tprv"));
    assert!(child["public_key"].as_str().unwrap().starts_with("tpub"));
    failure(&["derive", "-p", "0", "--network", "mainnet"], ROOT_PRIVATE, 2, "--network only");

    type Key = ExtendedPrivateKey<Secp256k1Curve<K256Backend>>;
    let key = ROOT_PRIVATE.parse::<Key>().unwrap();
    for (private_version, private_prefix, public_prefix) in [
        (Version::YPRV, "yprv", "ypub"),
        (Version::ZPRV, "zprv", "zpub"),
        (Version::UPRV, "uprv", "upub"),
        (Version::VPRV, "vprv", "vpub"),
    ] {
        let input = key.encode_with(private_version).unwrap().to_string();
        let child = success(&["derive", "-p", "0", "--output", "json"], &input);
        assert!(child["private_key"].as_str().unwrap().starts_with(private_prefix));
        assert!(child["public_key"].as_str().unwrap().starts_with(public_prefix));
    }
}

#[test]
fn derive_validates_seed_lengths_and_hex() {
    for bytes in [16, 64] {
        let input = format!("0x{}", "AB".repeat(bytes));
        let value = success(&["derive", "-p", "m", "--output", "json"], &input);
        assert!(value["private_key"].is_string());
    }
    for bytes in [0, 15, 65] {
        failure(&["derive", "-p", "m"], &format!("0x{}", "ab".repeat(bytes)), 1, "16 to 64 bytes");
    }
    let double_prefixed = format!("0x0x{SEED}");
    for invalid_hex in ["abcdef0", "0xabcdef0", "0xnot-hex", &double_prefixed] {
        failure(&["derive", "-p", "m"], invalid_hex, 1, "hex");
    }
    failure(&["derive", "-p", "m"], MNEMONIC, 1, "invalid extended key");
}

#[test]
fn input_files_stdin_and_invalid_io() {
    let file = InputFile::new(format!("{MNEMONIC}\r\n").as_bytes());
    let from_file = success(&["inspect", "-i", file.path(), "--output", "json"], "ignored stdin");
    let from_stdin = success(&["inspect", "-i", "-", "--output", "json"], MNEMONIC);
    assert_eq!(from_file, from_stdin);
    let seed = InputFile::new(format!("{SEED}\n").as_bytes());
    let derived = success(&["derive", "-i", seed.path(), "-p", "m", "--output", "json"], "");
    assert!(derived["private_key"] == ROOT_PRIVATE);
    for command in [vec!["inspect"], vec!["derive", "-p", "m"]] {
        failure(&command, "\n", 1, "input is empty");
        failure(&command, &"x".repeat(65_537), 1, "64 KiB");
        let output = run(&command, &[0xff, 0xfe]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
    }
    let absent = file.0.with_extension("missing");
    failure(&["inspect", "-i", absent.to_str().unwrap()], "", 1, "error:");
    let output = run(&["gen", "--passphrase-file", "-"], &[0xff]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
}
