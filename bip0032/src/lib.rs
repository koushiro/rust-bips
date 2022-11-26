//! Another Rust implementation of [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) standard.

#![deny(unused_imports)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
