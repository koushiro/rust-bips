# bli

[![](https://github.com/koushiro/rust-bips/actions/workflows/bli.yml/badge.svg)][actions]
[![](https://img.shields.io/docsrs/bli)][docs.rs]
[![](https://img.shields.io/crates/v/bli)][crates.io]
[![](https://img.shields.io/crates/l/bli)][crates.io]
[![](https://img.shields.io/crates/d/bli.svg)][crates.io]
[![](https://img.shields.io/badge/MSRV-1.85.0-green?logo=rust)][whatrustisit]

[actions]: https://github.com/koushiro/rust-bips/actions
[docs.rs]: https://docs.rs/bli
[crates.io]: https://crates.io/crates/bli
[whatrustisit]: https://www.whatrustisit.com

A CLI for BIP-0032 keys and BIP-0039 mnemonics, backed by the sibling
[`bip0032`](https://docs.rs/bip0032) and [`bip0039`](https://docs.rs/bip0039) libraries.

## Install from this repository

```sh
cargo install --path bli --locked
bli --help
```

Alternatively, run without installing:

```sh
cargo run --manifest-path bli/Cargo.toml --locked -- --help
```

## Commands

| Command    | Alias  | Purpose                                                                            |
| ---------- | ------ | ---------------------------------------------------------------------------------- |
| `generate` | `gen`  | Generate a mnemonic, entropy, and seed                                             |
| `inspect`  | `info` | Validate a mnemonic or extended key and display metadata                           |
| `derive`   | -      | Derive extended keys from a hex seed, extended private key, or extended public key |

Each command listed above accepts `-o, --output <FORMAT>`, with `text` (default)
and `json` as supported formats. This is a command-local option and must appear
after the command, for example `bli generate -o json` or
`bli generate --output json`. Use `bli <command> --help` for command-specific help.

### generate

```sh
bli generate
bli gen --words 24 --lang japanese
bli gen --passphrase-prompt --output json
bli gen --passphrase-file passphrase.txt --output json
```

| Option                     | Meaning                                          | Default   |
| -------------------------- | ------------------------------------------------ | --------- |
| `-w, --words <WORDS>`      | 12, 15, 18, 21, or 24 words                      | `12`      |
| `-l, --lang <LANG>`        | Mnemonic wordlist                                | `english` |
| `--passphrase-prompt`      | Read a hidden passphrase twice from the terminal | Off       |
| `--passphrase-file <PATH>` | Read a UTF-8 passphrase; `-` reads stdin         | None      |

The two passphrase options are mutually exclusive. Without either option, the
passphrase is empty; generation never prompts implicitly for a passphrase.

Supported languages: `english`, `chinese-simplified`, `chinese-traditional`,
`czech`, `french`, `italian`, `japanese`, `korean`, `portuguese`, and `spanish`.

Entropy is sampled once using the library's OS-seeded cryptographic RNG. The
mnemonic encodes that entropy, and the 64-byte seed is derived from that mnemonic
and the selected passphrase using BIP39 normalization.

Text output contains labeled `mnemonic`, `entropy`, `seed`, `language`, and
`words` fields. JSON contains exactly these fields plus the actual `passphrase`:

```json
{
  "mnemonic": "<generated mnemonic>",
  "entropy": "0x<lowercase hex>",
  "seed": "0x<128 lowercase hex digits>",
  "passphrase": "<actual passphrase, or an empty string>",
  "language": "english",
  "words": 12
}
```

This example uses placeholders. Actual JSON deliberately includes the passphrase
so the complete generation inputs are preserved. It retains the supplied Unicode
spelling, while seed derivation applies BIP39 NFKD normalization. Text output does
not display the passphrase.

### inspect

```sh
bli inspect --input mnemonic.txt
bli info --input mnemonic.txt --lang english --output json
bli inspect --input account.xpub --output json
bli inspect --input account.xprv
```

`-i, --input <PATH>` reads a mnemonic or extended key from a UTF-8 file; `-` reads
stdin explicitly. Without `--input`, piped stdin is read automatically; on an
interactive terminal, input is requested without echoing it.

`-l, --lang <LANG>` optionally selects a mnemonic wordlist. Otherwise all built-in
wordlists are checked. A mnemonic can be valid in multiple languages (notably the
Chinese wordlists); every match is reported instead of arbitrarily selecting one.
Language selection is not applicable to extended keys.

For a mnemonic, JSON returns `type` (`"mnemonic"`), `words`, and a
`languages` array. Validation checks both the wordlist and checksum.

For an extended key, JSON returns `type` (`"extended-key"`), `key_type`
(`"private"` or `"public"`), `network`, `version`, `version_bytes`, `depth`,
`child_number`, and `parent_fingerprint`. The child number is the raw unsigned
32-bit index, including the hardened bit. The fingerprint is lowercase hex.
Validation checks encoding, checksum, root metadata, and secp256k1 key data.
Unrecognized versions have `null` network/version names and retain their raw
version bytes if the library accepts their encoding.

Neither format echoes the original mnemonic, private key, or other secret
material. Invalid input produces a diagnostic on stderr, not a partial report.
Seeds are not supported by `inspect`.

### derive

```sh
bli derive --input seed.hex --path "m/84'/0'/0'" --output json
bli derive --input account.xprv --path "0/0"
bli derive --input account.xpub --path "0/0"
bli derive --input seed.hex --path m --network testnet
```

| Option                       | Meaning                                                            |
| ---------------------------- | ------------------------------------------------------------------ |
| `-i, --input <PATH>`         | Input file, or `-` for stdin; otherwise read piped stdin or prompt |
| `-p, --path <PATH>`          | Required derivation path                                           |
| `--network mainnet\|testnet` | Network for seed input only; defaults to `mainnet`                 |

Input type is detected automatically:

- **Hex seed:** 16–64 bytes, with an optional `0x` prefix. Outputs both private and public extended keys.
- **Extended private key:** outputs both private and public child extended keys.
- **Extended public key:** outputs only a public child extended key; hardened derivation is rejected.

There are no `--from` or `--private` flags. Mnemonics and passphrases are not
accepted by `derive`.

An absolute path, such as `m/0'/1`, requires a master key (depth zero). A relative
path, such as `0/1`, starts at the supplied key. `--path m` returns the master
key pair when given a seed. Hardened indices accept `'`, `h`, or `H`; quote paths
containing apostrophes in your shell. Resulting depth must not exceed 255.

An existing extended key retains its version; specifying `--network` for it is
an error. Recognized version families include `x`, `t`, `y`, `Y`, `z`, `Z`, `u`,
`U`, `v`, and `V`. These prefixes are preserved, not used to infer a derivation
path or generate addresses. Private input must have a recognized public/private
version pair to produce both outputs.

Text output contains `private: ...` and `public: ...` lines, omitting the private
line for public input. JSON always has exactly two fields:

```json
{
  "private_key": null,
  "public_key": "<derived extended public key>"
}
```

For seed/private input, `private_key` contains the derived extended private key
instead of `null`.

## Input, output, and exit status

- Results go to stdout; errors go to stderr. Hidden interactive prompts use the terminal.
- File/stdin input must be UTF-8 and is limited to 64 KiB.
- File/stdin reading removes exactly one trailing LF or CRLF. Passphrases retain
  all other whitespace, including leading/trailing spaces and additional newlines.
- Mnemonics, keys, and seeds additionally ignore surrounding whitespace.
- Exit status: `0` for success, `1` for invalid content, I/O, or runtime failure,
  and `2` for incorrect command usage. A closed output pipe is treated as success.

Generated output and derived private keys are secret material. JSON generation
includes the actual passphrase by design, not a redacted value. Protect the entire
output as a wallet backup; avoid CI logs, shell tracing, and shared terminals.
Input is passed through files, stdin, or hidden prompts rather than command-line
secret values. Owned secret buffers use best-effort zeroization; this does not
prevent terminal history, OS buffering, swap, or copies made by other processes.
