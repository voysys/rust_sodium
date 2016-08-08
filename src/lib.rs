//! Rust bindings to the [sodium library](https://github.com/jedisct1/libsodium).
//!
//! Sodium is a portable implementation of Dan Bernsteins [NaCl: Networking and
//! Cryptography library](http://nacl.cr.yp.to)
//!
//! For most users, if you want public-key (asymmetric) cryptography you should use
//! the functions in `crypto::box_` for encryption/decryption.
//!
//! If you want secret-key (symmetric) cryptography you should be using the
//! functions in `crypto::secretbox` for encryption/decryption.
//!
//! For public-key signatures you should use the functions in `crypto::sign` for
//! signature creation and verification.
//!
//! Unless you know what you're doing you most certainly don't want to use the
//! functions in `crypto::scalarmult`, `crypto::stream`, `crypto::auth` and
//! `crypto::onetimeauth`.
//!
//! ## Thread Safety
//! All functions in this library are thread-safe provided that the `init()`
//! function has been called during program execution.
//!
//! If `init()` hasn't been called then all functions except the random-number
//! generation functions and the key-generation functions are thread-safe.
//!
//! # Public-key cryptography
//!  `crypto::box_`
//!
//!  `crypto::sign`
//!
//! # Sealed boxes
//!  `crypto::sealedox`
//!
//! # Secret-key cryptography
//!  `crypto::secretbox`
//!
//!  `crypto::stream`
//!
//!  `crypto::auth`
//!
//!  `crypto::onetimeauth`
//!
//! # Low-level functions
//!  `crypto::hash`
//!
//!  `crypto::verify`
//!
//!  `crypto::shorthash`
#![warn(missing_docs)]
#![warn(non_upper_case_globals)]
#![warn(non_camel_case_types)]
#![warn(unused_qualifications)]

extern crate rust_sodium_sys as ffi;
extern crate libc;
extern crate rand;
#[cfg(any(test, feature = "rustc-serialize"))]
extern crate rustc_serialize;
#[cfg(any(test, feature = "serde"))]
extern crate serde;

mod marshal;
#[macro_use]
mod newtype_macros;
pub mod randombytes;
pub mod utils;
pub mod version;

#[cfg(test)]
mod test_utils;

/// Cryptographic functions
pub mod crypto {
    pub mod box_;
    pub mod sealedbox;
    pub mod sign;
    pub mod scalarmult;
    pub mod auth;
    pub mod hash;
    pub mod secretbox;
    pub mod onetimeauth;
    pub mod pwhash;
    pub mod stream;
    pub mod shorthash;
    pub mod verify;
}

/// Initialises libsodium and chooses faster versions of the primitives if possible.  Also makes the
/// random number generation functions (`gen_key`, `gen_keypair`, `gen_nonce`, `randombytes`,
/// `randombytes_into`) thread-safe.
///
/// Returns `false` if initialisation failed.
pub fn init() -> bool {
    unsafe { ffi::sodium_init() != -1 }
}

/// Sets [libsodium's `randombytes_implementation`]
/// (https://download.libsodium.org/doc/advanced/custom_rng.html) to use a
/// [Rust `Rng` implementation](../rand/trait.Rng.html) and initialises libsodium.
/// See [the rust_sodium-sys' docs](../rust_sodium_sys/fn.init_with_rng.html) for further details.
pub fn init_with_rng<T: rand::Rng>(rng: &mut T) -> Result<(), i32> {
    ffi::init_with_rng(rng)
}