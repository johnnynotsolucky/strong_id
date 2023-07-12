//! Strongly typed IDs which optionally satisfy the [TypeID](https://github.com/jetpack-io/typeid) specification.
//!
//! A StrongId is any type which implements `StrongId<T: Id>`.
//!
//! The `Id` trait is implemented for `u8`, `u16`, `u32`, `u64`, `u128`, `usize` and when the `"uuid"` feature is enabled,
//! `Uuid`.
//!
//! ## Examples
//!
//! ### Dynamic StrongIds
//!
//! #### ID with a prefix
//! ```rust
//! use strong_id::{prefix, DynamicStrongId};
//!
//! let user_id = DynamicStrongId::<u16>::new(prefix!("user"), 3203).unwrap();
//! println!("{}", user_id); // user_0343
//!
//! let user_id = "user_0343".parse::<DynamicStrongId<u16>>().unwrap();
//! println!("{:#?}", user_id);
//! // DynamicStrongId {
//! //     prefix: Some(
//! //        Prefix {
//! //           inner: "user",
//! //        ,
//! //     ),
//! //     suffix: 3203,
//! // }
//! ```
//!
//! #### ID without a prefix
//! ```rust
//! use strong_id::{prefix, DynamicStrongId};
//!
//! let id = DynamicStrongId::<u16>::new_plain(3203);
//! println!("{}", id); // 0343
//!
//! let id = "0343".parse::<DynamicStrongId<u16>>().unwrap();
//! println!("{:#?}", id);
//! // DynamicStrongId {
//! //     prefix: None,
//! //     suffix: 3203,
//! // }
//! ```
//!
//! #### TypeId with a prefix
//!
//! ```rust,ignore
//! use strong_id::{prefix, DynamicStrongId};
//! use uuid::Uuid;
//!
//! let user_id = DynamicStrongId::<Uuid>::now_v7(prefix!("user")).unwrap();
//! println!("{}", user_id); // user_01h536gfwffx2rm6pa0xg63337
//!
//! let user_id = "user_01h536gfwffx2rm6pa0xg63337"
//!     .parse::<DynamicStrongId<Uuid>>()
//!     .unwrap();
//! println!("{:#?}", user_id);
//! // DynamicStrongId {
//! //     prefix: Some(
//! //        Prefix {
//! //            inner: "user",
//! //        },
//! //     ),
//! //     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
//! // }
//! ```
//!
//! #### TypeId without a prefix
//!
//! ```rust,ignore
//! use strong_id::{prefix, DynamicStrongId};
//! use uuid::Uuid;
//!
//! let id = DynamicStrongId::<Uuid>::now_v7_plain();
//! println!("{}", id); // 01h536gfwffx2rm6pa0xg63337
//!
//! let id = "01h536gfwffx2rm6pa0xg63337"
//!     .parse::<DynamicStrongId<Uuid>>()
//!     .unwrap();
//! println!("{:#?}", id);
//! // DynamicStrongId {
//! //     prefix: None,
//! //     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
//! // }
//! ```
//!
//! ### Generated StrongIds
//!
//! #### ID with a prefix
//! ```rust
//! use strong_id::strong_id;
//!
//! strong_id!(pub struct UserId(u16 => "user"));
//!
//! let user_id = UserId::from(3203);
//! println!("{}", user_id); // user_0343
//!
//! let user_id = "user_0343".parse::<UserId>().unwrap();
//! println!("{:#?}", user_id);
//! // UserId {
//! //     suffix: 3203,
//! // }
//! ```
//!
//! #### ID without a prefix
//!
//! ```rust
//! use strong_id::strong_id;
//!
//! strong_id!(pub struct Id(u16));
//!
//! let id = Id::from(3203);
//! println!("{}", id); // user_0343
//!
//! let id = "0343".parse::<Id>().unwrap();
//! println!("{:#?}", id);
//! // Id {
//! //     suffix: 3203,
//! // }
//! ```
//!
//! #### Generated TypeId with a prefix
//!
//! ```rust,ignore
//! use strong_id::{strong_uuid, StrongUuid};
//!
//! strong_uuid!(pub struct UserId("user"));
//!
//! let user_id = UserId::now_v7();
//! println!("{}", user_id); // user_01h536z8abez196j2nzz06y8c8
//!
//! let user_id = "user_01h536z8abez196j2nzz06y8c8".parse::<UserId>().unwrap();
//! println!("{:#?}", user_id);
//! // UserId {
//! //     suffix: 0189466f-a14b-77c2-9348-55ffc06f2188,
//! // }
//! ```
//!
//! #### Generated TypeId without a prefix
//!
//! ```rust,ignore
//! use strong_id::{strong_uuid, StrongUuid};
//!
//! strong_uuid!(pub struct Id);
//!
//! let id = Id::now_v7();
//! println!("{}", id); // 01h5372sq2egxb6ps3taq7p6np
//!
//! let id = "01h5372sq2egxb6ps3taq7p6np".parse::<Id>().unwrap();
//! println!("{:#?}", id);
//! // UserId {
//! //     suffix: 01894671-66e2-743a-b35b-23d2ae7b1ab6,
//! // }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate self as strong_id;

mod base32;
mod dynamic;

use crate::base32::encoded_len;
pub use dynamic::*;
use thiserror::Error;

pub use base32::Base32Error;
pub use strong_id_macros::*;

#[cfg(feature = "uuid")]
pub use uuid;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "serde")]
pub use serde;

/// Represents a type which can be encoded and decoded
pub trait Id {
	/// Encode the value into a `String`
	fn encode(&self) -> String;
	/// Decode the value from a `str`
	fn decode<T: AsRef<str>>(val: T) -> Result<Self, Error>
	where
		Self: Sized;
}

/// Represents a type which can be used as a StrongId
pub trait StrongId<T: Id>: core::fmt::Display + core::str::FromStr {
	fn prefix(&self) -> Option<&str>;
	fn id(&self) -> &T;
}

/// Utility trait to allow StrongId's backed by a Uuid to wrap calls to Uuid `new_` and `now_`
/// functions.
#[cfg(feature = "uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
pub trait StrongUuid {
	fn from_u128(v: u128) -> Self;

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	fn new_v1(ts: uuid::Timestamp, node_id: &[u8; 6]) -> Self;

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	fn now_v1(node_id: &[u8; 6]) -> Self;

	#[cfg(feature = "uuid-v3")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v3")))]
	fn new_v3(namespace: &Uuid, name: &[u8]) -> Self;

	#[cfg(feature = "uuid-v4")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v4")))]
	fn new_v4() -> Self;

	#[cfg(feature = "uuid-v5")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v5")))]
	fn new_v5(namespace: &Uuid, name: &[u8]) -> Self;

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(all(uuid_unstable, feature = "uuid-v6"))))]
	fn new_v6(ts: uuid::Timestamp, node_id: &[u8; 6]) -> Self;

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(all(uuid_unstable, feature = "uuid-v6"))))]
	fn now_v6(node_id: &[u8; 6]) -> Self;

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(all(uuid_unstable, feature = "uuid-v7"))))]
	fn new_v7(ts: uuid::Timestamp) -> Self;

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(all(uuid_unstable, feature = "uuid-v7"))))]
	fn now_v7() -> Self;

	#[cfg(all(uuid_unstable, feature = "uuid-v8"))]
	#[cfg_attr(docsrs, doc(cfg(all(uuid_unstable, feature = "uuid-v8"))))]
	fn new_v8(buf: [u8; 16]) -> Self;
}

macro_rules! impl_strong_uint {
	($t:ty) => {
		impl Id for $t {
			fn encode(&self) -> String {
				let mut out = [0u8; encoded_len::<$t>()];
				base32::encode(&self.to_be_bytes(), &mut out);
				let encoded = unsafe { ::core::str::from_utf8_unchecked(&out) };
				format!("{encoded}")
			}

			fn decode<T: AsRef<str>>(val: T) -> Result<Self, Error> {
				let val = val.as_ref();
				if val.len() != encoded_len::<$t>() {
					return Err(Error::InvalidLength(encoded_len::<$t>(), val.len()));
				}
				let mut out = [0; core::mem::size_of::<$t>()];
				base32::decode(val.as_bytes(), &mut out)?;

				Ok(Self::from_be_bytes(out))
			}
		}
	};
}

impl_strong_uint!(u8);
impl_strong_uint!(u16);
impl_strong_uint!(u32);
impl_strong_uint!(u64);
impl_strong_uint!(u128);
impl_strong_uint!(usize);

#[cfg(feature = "uuid")]
impl Id for Uuid {
	fn encode(&self) -> String {
		let mut out = [0; 26];
		base32::encode(self.as_bytes(), &mut out);
		let encoded = unsafe { core::str::from_utf8_unchecked(&out) };
		format!("{encoded}")
	}

	fn decode<T: AsRef<str>>(val: T) -> Result<Self, Error> {
		let val = val.as_ref();
		if val.len() != 26 {
			return Err(Error::InvalidLength(26, val.len()));
		}
		let mut out = [0; 16];
		base32::decode(val.as_bytes(), &mut out)?;

		Ok(Self::from_bytes(out))
	}
}

/// Errors which may occur when creating or parsing StrongIds
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
	/// Created from a [`Base32Error`]
	#[error(transparent)]
	Base32Error(#[from] Base32Error),
	/// A prefix was expected, but was not found
	#[error("expected prefix `{0}`")]
	MissingPrefix(String),
	/// The given prefix did not match the expected prefix
	#[error("invalid prefix. expected {0}, found {1}")]
	InvalidPrefix(String, String),
	/// A prefix was expected to be configured, but was not found
	#[error("no prefix was given, but one was expected")]
	PrefixExpected,
	/// A prefix was given, but none was expected
	#[error("found prefix `{0}`, none expected")]
	NoPrefixExpected(String),
	/// The length of the encoded value to be decoded was incorrect
	#[error("invalid length. expected {0}, found {1}")]
	InvalidLength(usize, usize),
	/// The prefix is too long
	#[error("prefix too long. should be less than 64 characters, found {0}")]
	PrefixTooLong(usize),
	/// A non-alphanumeric, non-lowercase character was found. When the "delimited" feature is
	/// enabled, this will not include the `'_'` character.
	#[error("prefix may only contain lowercase ascii characters, found `{0}`")]
	IncorrectPrefixCharacter(char),
}

/// Generate a StrongId
///
/// TODO examples
#[macro_export]
macro_rules! strong_id {
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident($inner:ty)
    ) => {
        $crate::_internal_strong_id! {
            $(#[$outer])*
            $vis struct $t($inner)
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident($inner:ty => $prefix:literal)
    ) => {
        $crate::_internal_strong_id! {
            $(#[$outer])*
            $vis struct $t($inner => $prefix)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_strong_id {
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident($inner:ty$( => $prefix:literal)?)
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
        #[derive($crate::StrongId)]
		#[strong_id($(prefix = $prefix, )?suffix = "suffix")]
        $vis struct $t {
            suffix: $inner,
        }

		$crate::_internal_impl_common!(@@internal $t($inner));

		$crate::_internal_impl_from_str!(@@internal $t($inner => $($prefix)?));

        impl From<$t> for $inner {
            fn from(value: $t) -> Self {
                value.suffix
            }
        }

        impl From<$inner> for $t {
            fn from(value: $inner) -> Self {
                Self {
                    suffix: value,
                }
            }
        }
    };
}

/// Generate a StrongId backed by a [`Uuid`]
#[cfg(feature = "uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
#[macro_export]
macro_rules! strong_uuid {
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident
    ) => {
        $crate::_internal_strong_uuid! {
            $(#[$outer])*
            $vis struct $t()
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident($prefix:literal)
    ) => {
        $crate::_internal_strong_uuid! {
            $(#[$outer])*
            $vis struct $t($prefix)
        }
    };
}
#[cfg(feature = "uuid")]
#[macro_export]
#[doc(hidden)]
macro_rules! _internal_strong_uuid {
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident($($prefix:literal)?)
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
        #[derive($crate::StrongId, $crate::StrongUuid)]
		#[strong_id($(prefix = $prefix, )?suffix = "suffix")]
        $vis struct $t {
            suffix: $crate::uuid::Uuid,
        }

		$crate::_internal_impl_common!(@@internal $t($crate::uuid::Uuid));

		$crate::_internal_impl_from_str!(@@internal $t($crate::uuid::Uuid => $($prefix)?));

        impl From<$t> for $crate::uuid::Uuid {
            fn from(value: $t) -> Self {
                value.suffix
            }
        }

        impl From<$crate::uuid::Uuid> for $t {
            fn from(uuid: $crate::uuid::Uuid) -> Self {
                Self {
                    suffix: uuid,
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_impl_common {
	(@@internal $t:ident($inner:ty)) => {
		impl ::core::fmt::Display for $t {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				use $crate::{Id, StrongId};
				match self.prefix() {
					Some(prefix) => write!(f, "{}_{}", prefix, self.suffix.encode()),
					None => write!(f, "{}", self.suffix.encode()),
				}
			}
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_impl_from_str {
	(@@internal $t:ident($inner:ty => $($prefix:literal)?)) => {
        impl ::core::str::FromStr for $t {
			type Err = $crate::Error;

            #[inline]
            fn from_str(value: &str) -> Result<Self, Self::Err> {
				let split = value.rsplit_once('_');

				#[allow(unused_mut)]
				#[allow(unused_assignments)]
				let mut prefix: Option<&str> = None;
				$(prefix = Some($prefix);)?

                let suffix = match prefix {
                    Some(prefix) => {
						 match split {
							  None => return Err($crate::Error::MissingPrefix(prefix.into())),
							  Some((parsed_prefix, _suffix)) if parsed_prefix.is_empty() => return Err($crate::Error::MissingPrefix(prefix.into())),
							  Some((parsed_prefix, suffix)) => {
								  if parsed_prefix != prefix {
									  return Err($crate::Error::InvalidPrefix(prefix.to_string(), parsed_prefix.to_string()));
								  }

								  <$inner as $crate::Id>::decode(suffix)?
							  },
						 }
					},
                    None => {
						 match split {
							  Some((parsed_prefix, _suffix)) => return Err($crate::Error::NoPrefixExpected(parsed_prefix.to_string())),
							  None => <$inner as $crate::Id>::decode(value)?
						 }
					}
                };

				Ok(Self {
					suffix,
				})
            }
        }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn u32_prefix_valid() {
		strong_id!(pub struct PrefixU32(u32 => "prefix"));

		assert_eq!(PrefixU32::from(0).prefix(), Some("prefix"));

		struct Case(&'static str, u32);
		let cases = vec![
			Case("prefix_0000000", u32::MIN),
			Case("prefix_3zzzzzz", u32::MAX),
			Case("prefix_000009d", 301),
		];

		for case in cases {
			let id = PrefixU32::from(case.1);
			assert_eq!(&format!("{id}"), case.0);
			assert_eq!(*id.id(), case.1);

			let parsed = case.0.parse::<PrefixU32>().unwrap();
			assert_eq!(parsed.suffix, case.1);
			assert_eq!(*parsed.id(), case.1);
		}
	}

	#[test]
	fn u32_prefix_invalid() {
		strong_id!(pub struct PrefixU32(u32 => "prefix"));

		struct Case(&'static str, Error);
		let cases = vec![
			Case("0000000", Error::MissingPrefix("prefix".into())),
			Case("000009d", Error::MissingPrefix("prefix".into())),
			Case("3zzzzzz", Error::MissingPrefix("prefix".into())),
			Case("prefix_0000000000", Error::InvalidLength(7, 10)),
			Case(
				"prefix_zzzzzzz",
				Error::Base32Error(Base32Error::InvalidFirstByte),
			),
			Case(
				"prefix_z000000",
				Error::Base32Error(Base32Error::InvalidFirstByte),
			),
			Case("prefix_09d", Error::InvalidLength(7, 3)),
			Case("zzzzzzz", Error::MissingPrefix("prefix".into())),
			Case("dyn_3000000", Error::InvalidPrefix("prefix".into(), "dyn".into())),
		];

		for case in cases {
			let error = case.0.parse::<PrefixU32>().unwrap_err();
			assert_eq!(error, case.1);
		}
	}

	#[test]
	fn u16_no_prefix_valid() {
		strong_id!(pub struct NoPrefixU16(u16));

		assert_eq!(NoPrefixU16::from(0).prefix(), None);

		struct Case(&'static str, u16);
		let cases = vec![
			Case("0000", u16::MIN),
			Case("1zzz", u16::MAX),
			Case("009d", 301),
		];

		for case in cases {
			let id = NoPrefixU16::from(case.1);
			assert_eq!(&format!("{id}"), case.0);
			assert_eq!(*id.id(), case.1);

			let parsed = case.0.parse::<NoPrefixU16>().unwrap();
			assert_eq!(parsed.suffix, case.1);
			assert_eq!(*parsed.id(), case.1);
		}
	}

	#[test]
	fn u16_no_prefix_invalid() {
		strong_id!(pub struct NoPrefixU16(u16));

		struct Case(&'static str, Error);
		let cases = vec![
			Case("00000", Error::InvalidLength(4, 5)),
			Case("prefix_00000", Error::NoPrefixExpected("prefix".into())),
			Case("zzzz", Error::Base32Error(Base32Error::InvalidFirstByte)),
			Case("z000", Error::Base32Error(Base32Error::InvalidFirstByte)),
			Case("09d", Error::InvalidLength(4, 3)),
		];

		for case in cases {
			let error = case.0.parse::<NoPrefixU16>().unwrap_err();
			assert_eq!(error, case.1);
		}
	}

	#[cfg(feature = "serde")]
	#[test]
	fn u32_prefix_serde() {
		strong_id!(pub struct PrefixU32(u32 => "prefix"));

		let value: PrefixU32 = serde_json::from_str("\"prefix_000009d\"").unwrap();
		assert_eq!(*value.id(), 301);

		let value = serde_json::to_string(&value).unwrap();
		assert_eq!("\"prefix_000009d\"", value);
	}
}
