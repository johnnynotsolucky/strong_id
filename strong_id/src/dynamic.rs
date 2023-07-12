use crate::{Error, Id, StrongId};
use core::fmt::{Display, Formatter};
use std::borrow::Cow;

#[cfg(feature = "uuid")]
use uuid::Uuid;

fn map_prefix<'p, I: Into<Prefix<'p>>>(prefix: I) -> Result<Prefix<'p>, Error> {
	let prefix = prefix.into();
	if prefix.inner.len() >= 64 {
		return Err(Error::PrefixTooLong(prefix.inner.len()));
	}

	for b in prefix.inner.as_bytes() {
		if cfg!(feature = "delimited") && *b == b'_' {
			continue;
		} else if !b.is_ascii_lowercase() {
			return Err(Error::IncorrectPrefixCharacter(*b as char));
		}
	}
	if prefix.inner.is_empty() {
		return Err(Error::PrefixExpected);
	}
	Ok(prefix)
}

/// A StrongId with runtime validation
///
/// ## Examples
///
/// #### ID with a prefix
/// ```rust
/// # fn main() {
/// use strong_id::{prefix, DynamicStrongId};
///
/// let user_id = DynamicStrongId::<u16>::new(prefix!("user"), 3203).unwrap();
/// println!("{}", user_id); // user_0343
///
/// let user_id = "user_0343".parse::<DynamicStrongId<u16>>().unwrap();
/// println!("{:#?}", user_id);
/// // DynamicStrongId {
/// //     prefix: Some(
/// //        Prefix {
/// //           inner: "user",
/// //        },
/// //     ),
/// //     suffix: 3203,
/// // }
/// # }
/// ```
///
/// #### ID without a prefix
/// ```rust
/// # fn main() {
/// use strong_id::{prefix, DynamicStrongId};
///
/// let user_id = DynamicStrongId::<u16>::new_plain(3203);
/// println!("{}", user_id); // 0343
///
/// let user_id = "0343".parse::<DynamicStrongId<u16>>().unwrap();
/// println!("{:#?}", user_id);
/// // DynamicStrongId {
/// //     prefix: None,
/// //     suffix: 3203,
/// // }
/// # }
/// ```
///
/// #### TypeId with a prefix
///
/// ```rust,ignore
/// # fn main() {
/// use strong_id::{prefix, DynamicStrongId};
/// use uuid::Uuid;
///
/// let user_id = DynamicStrongId::<Uuid>::now_v7(prefix!("user")).unwrap();
/// println!("{}", user_id); // user_01h536gfwffx2rm6pa0xg63337
///
/// let user_id = "user_01h536gfwffx2rm6pa0xg63337"
///  .parse::<DynamicStrongId<Uuid>>()
///  .unwrap();
/// println!("{:#?}", user_id);
/// // DynamicStrongId {
/// //     prefix: Some(
/// //        Prefix {
/// //           inner: "user",
/// //        },
/// //     ),
/// //     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
/// // }
/// # }
/// ```
///
/// #### TypeId without a prefix
///
/// ```rust,ignore
/// # fn main() {
/// use strong_id::{prefix, DynamicStrongId};
/// use uuid::Uuid;
///
/// let user_id = DynamicStrongId::<Uuid>::now_v7_plain();
/// println!("{}", user_id); // 01h536gfwffx2rm6pa0xg63337
///
/// let user_id = "01h536gfwffx2rm6pa0xg63337"
///  .parse::<DynamicStrongId<Uuid>>()
///  .unwrap();
/// println!("{:#?}", user_id);
/// // DynamicStrongId {
/// //     prefix: None,
/// //     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
/// // }
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DynamicStrongId<'p, T: Id> {
	prefix: Option<Prefix<'p>>,
	suffix: T,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[doc(hidden)]
pub struct Prefix<'p> {
	inner: Cow<'p, str>,
}

impl<'p> Display for Prefix<'p> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.inner)
	}
}

impl<'p> From<&'p str> for Prefix<'p> {
	fn from(value: &'p str) -> Self {
		Self {
			inner: Cow::Borrowed(value),
		}
	}
}

impl<'p> From<String> for Prefix<'p> {
	fn from(value: String) -> Self {
		Self {
			inner: Cow::Owned(value),
		}
	}
}

impl<'p, T: Id> DynamicStrongId<'p, T> {
	/// Create a new ID from a given value with a prefix
	pub fn new<I: Into<Prefix<'p>>>(prefix: I, value: T) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: value,
		})
	}

	/// Create a new ID from a given value without a prefix
	pub fn new_plain(value: T) -> Self {
		Self {
			prefix: None,
			suffix: value,
		}
	}
}

#[cfg(feature = "uuid")]
impl<'p> From<DynamicStrongId<'p, Uuid>> for Uuid {
	fn from(value: DynamicStrongId<Uuid>) -> Self {
		value.suffix
	}
}

// Utility functions for calling Uuid `new_` and `now_` functions when a [`DynamicStrongId`] is
// backed by a [`Uuid`].
#[cfg(feature = "uuid")]
impl<'p> DynamicStrongId<'p, Uuid> {
	/// Create a new UUID-backed ID from a u128 with a prefix
	pub fn from_u128<I: Into<Prefix<'p>>>(prefix: I, v: u128) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::from_u128(v),
		})
	}

	/// Create a new UUID-backed ID from a u128 without a prefix
	pub fn from_u128_plain(v: u128) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::from_u128(v),
		}
	}

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	/// Create a new UUID-backed ID by generating a v1 UUID with a prefix
	///
	/// See [`Uuid::new_v1`]
	pub fn new_v1<I: Into<Prefix<'p>>>(
		prefix: I,
		ts: uuid::Timestamp,
		node_id: &[u8; 6],
	) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v1(ts, node_id),
		})
	}

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	/// Create a new UUID-backed ID by generating a v1 UUID without a prefix
	///
	/// See [`Uuid::new_v1`]
	pub fn new_v1_plain(ts: uuid::Timestamp, node_id: &[u8; 6]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v1(ts, node_id),
		}
	}

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	/// Create a new UUID-backed ID by generating a v1 UUID with a prefix
	///
	/// See [`Uuid::now_v1`]
	pub fn now_v1<I: Into<Prefix<'p>>>(prefix: I, node_id: &[u8; 6]) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::now_v1(node_id),
		})
	}

	#[cfg(feature = "uuid-v1")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v1")))]
	/// Create a new UUID-backed ID by generating a v1 UUID without a prefix
	///
	/// See [`Uuid::now_v1`]
	pub fn now_v1_plain(node_id: &[u8; 6]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::now_v1(node_id),
		}
	}

	#[cfg(feature = "uuid-v3")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v3")))]
	/// Create a new UUID-backed ID by generating a v3 UUID with a prefix
	///
	/// See [`Uuid::new_v3`]
	pub fn new_v3<I: Into<Prefix<'p>>>(
		prefix: I,
		namespace: &Uuid,
		name: &[u8],
	) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v3(namespace, name),
		})
	}

	#[cfg(feature = "uuid-v3")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v3")))]
	/// Create a new UUID-backed ID by generating a v3 UUID without a prefix
	///
	/// See [`Uuid::new_v3`]
	pub fn new_v3_plain(namespace: &Uuid, name: &[u8]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v3(namespace, name),
		}
	}

	#[cfg(feature = "uuid-v4")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v4")))]
	/// Create a new UUID-backed ID by generating a v4 UUID with a prefix
	///
	/// See [`Uuid::new_v4`]
	pub fn new_v4<I: Into<Prefix<'p>>>(prefix: I) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v4(),
		})
	}

	#[cfg(feature = "uuid-v4")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v4")))]
	/// Create a new UUID-backed ID by generating a v4 UUID without a prefix
	///
	/// See [`Uuid::new_v4`]
	pub fn new_v4_plain() -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v4(),
		}
	}

	#[cfg(feature = "uuid-v5")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v5")))]
	/// Create a new UUID-backed ID by generating a v5 UUID with a prefix
	///
	/// See [`Uuid::new_v5`]
	pub fn new_v5<I: Into<Prefix<'p>>>(
		prefix: I,
		namespace: &Uuid,
		name: &[u8],
	) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v5(namespace, name),
		})
	}

	#[cfg(feature = "uuid-v5")]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v5")))]
	/// Create a new UUID-backed ID by generating a v5 UUID without a prefix
	///
	/// See [`Uuid::new_v5`]
	pub fn new_v5_plain(namespace: &Uuid, name: &[u8]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v5(namespace, name),
		}
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v6")))]
	/// Create a new UUID-backed ID by generating a v6 UUID with a prefix
	///
	/// See [`Uuid::new_v6`]
	pub fn new_v6<I: Into<Prefix<'p>>>(
		prefix: I,
		ts: ::uuid::Timestamp,
		node_id: &[u8; 6],
	) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v6(ts, node_id),
		})
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v6")))]
	/// Create a new UUID-backed ID by generating a v6 UUID without a prefix
	///
	/// See [`Uuid::new_v6`]
	pub fn new_v6_plain(ts: ::uuid::Timestamp, node_id: &[u8; 6]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v6(ts, node_id),
		}
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v6")))]
	/// Create a new UUID-backed ID by generating a v6 UUID with a prefix
	///
	/// See [`Uuid::now_v6`]
	pub fn now_v6<I: Into<Prefix<'p>>>(prefix: I, node_id: &[u8; 6]) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::now_v6(node_id),
		})
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v6"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v6")))]
	/// Create a new UUID-backed ID by generating a v6 UUID without a prefix
	///
	/// See [`Uuid::now_v6`]
	pub fn now_v6_plain(node_id: &[u8; 6]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::now_v6(node_id),
		}
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v7")))]
	/// Create a new UUID-backed ID by generating a v7 UUID with a prefix
	///
	/// See [`Uuid::new_v7`]
	pub fn new_v7<I: Into<Prefix<'p>>>(prefix: I, ts: ::uuid::Timestamp) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v7(ts),
		})
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v7")))]
	/// Create a new UUID-backed ID by generating a v7 UUID without a prefix
	///
	/// See [`Uuid::new_v7`]
	pub fn new_v7_plain(ts: ::uuid::Timestamp) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v7(ts),
		}
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v7")))]
	/// Create a new UUID-backed ID by generating a v7 UUID with a prefix
	///
	/// See [`Uuid::now_v7`]
	pub fn now_v7<I: Into<Prefix<'p>>>(prefix: I) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::now_v7(),
		})
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v7"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v7")))]
	/// Create a new UUID-backed ID by generating a v7 UUID without a prefix
	///
	/// See [`Uuid::now_v7`]
	pub fn now_v7_plain() -> Self {
		Self {
			prefix: None,
			suffix: Uuid::now_v7(),
		}
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v8"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v8")))]
	/// Create a new UUID-backed ID by generating a v7 UUID with a prefix
	///
	/// See [`Uuid::new_v8`]
	pub fn new_v8<I: Into<Prefix<'p>>>(prefix: I, buf: [u8; 16]) -> Result<Self, Error> {
		Ok(Self {
			prefix: Some(map_prefix(prefix)?),
			suffix: Uuid::new_v8(buf),
		})
	}

	#[cfg(all(uuid_unstable, feature = "uuid-v8"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "uuid-v8")))]
	/// Create a new UUID-backed ID by generating a v7 UUID without a prefix
	///
	/// See [`Uuid::new_v8`]
	pub fn new_v8_plain(buf: [u8; 16]) -> Self {
		Self {
			prefix: None,
			suffix: Uuid::new_v8(buf),
		}
	}
}

impl<'p, T: Id> Display for DynamicStrongId<'p, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match &self.prefix {
			Some(prefix) => write!(f, "{}_{}", prefix, self.suffix.encode()),
			None => write!(f, "{}", self.suffix.encode()),
		}
	}
}

impl<'p, T: Id> core::str::FromStr for DynamicStrongId<'p, T> {
	type Err = Error;

	#[inline]
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let split = value.rsplit_once('_');

		Ok(match split {
			Some((prefix, _suffix)) if prefix.trim().is_empty() => {
				return Err(Error::MissingPrefix(prefix.into()))
			}
			Some((prefix, suffix)) => Self {
				prefix: Some(map_prefix(prefix.to_string())?),
				suffix: T::decode(suffix)?,
			},
			None => Self {
				prefix: None,
				suffix: T::decode(value)?,
			},
		})
	}
}

impl<'p, T: Id> StrongId<T> for DynamicStrongId<'p, T> {
	fn prefix(&self) -> Option<&str> {
		match &self.prefix {
			Some(prefix) => Some(prefix.inner.as_ref()),
			None => None,
		}
	}

	fn id(&self) -> &T {
		&self.suffix
	}
}

#[cfg(feature = "serde")]
impl<'p, T: Id> serde::Serialize for DynamicStrongId<'p, T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

#[cfg(feature = "serde")]
impl<'p, 'de, T: Id> serde::Deserialize<'de> for DynamicStrongId<'p, T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.parse::<Self>()
			.map_err(|error| serde::de::Error::custom(error.to_string()))
	}
}

#[cfg(test)]
mod tests {
	use crate::{Base32Error, DynamicStrongId, Error, Prefix, StrongId};

	#[test]
	fn valid_u32() {
		struct Case(Option<Prefix<'static>>, &'static str, u32);
		let cases = vec![
			Case(Some("dyn".into()), "dyn_0000000", u32::MIN),
			Case(Some("dyn".into()), "dyn_3zzzzzz", u32::MAX),
			Case(Some("dyn".into()), "dyn_000009d", 301),
			Case(None, "000009d", 301),
			Case(None, "3zzzzzz", u32::MAX),
			Case(None, "0000000", u32::MIN),
		];

		for case in cases {
			let id = match &case.0 {
				Some(prefix) => DynamicStrongId::new(prefix.clone(), case.2).unwrap(),
				None => DynamicStrongId::new_plain(case.2),
			};
			assert_eq!(&format!("{id}"), case.1);
			assert_eq!(*id.id(), case.2);

			let parsed = case.1.parse::<DynamicStrongId<u32>>().unwrap();
			assert_eq!(parsed.suffix, case.2);
			assert_eq!(parsed.prefix, case.0);
			assert_eq!(*parsed.id(), case.2);
		}
	}

	#[test]
	fn valid_u16() {
		struct Case(Option<Prefix<'static>>, &'static str, u16);
		let cases = vec![
			Case(Some("dyn".into()), "dyn_0000", u16::MIN),
			Case(Some("dyn".into()), "dyn_1zzz", u16::MAX),
			Case(Some("dyn".into()), "dyn_009d", 301),
			Case(None, "009d", 301),
			Case(None, "1zzz", u16::MAX),
			Case(None, "0000", u16::MIN),
		];

		for case in cases {
			let id = match &case.0 {
				Some(prefix) => DynamicStrongId::new(prefix.clone(), case.2).unwrap(),
				None => DynamicStrongId::new_plain(case.2),
			};
			assert_eq!(&format!("{id}"), case.1);
			assert_eq!(*id.id(), case.2);

			let parsed = case.1.parse::<DynamicStrongId<u16>>().unwrap();
			assert_eq!(parsed.suffix, case.2);
			assert_eq!(parsed.prefix, case.0);
			assert_eq!(*parsed.id(), case.2);
		}
	}

	#[test]
	fn valid_usize() {
		struct Case(Option<Prefix<'static>>, &'static str, usize);
		let cases = vec![
			Case(Some("dyn".into()), "dyn_0000000000000", usize::MIN),
			Case(Some("dyn".into()), "dyn_fzzzzzzzzzzzz", usize::MAX),
			Case(Some("dyn".into()), "dyn_000000000009d", 301),
			Case(None, "000000000009d", 301),
			Case(None, "fzzzzzzzzzzzz", usize::MAX),
			Case(None, "0000000000000", usize::MIN),
		];

		for case in cases {
			let id = match &case.0 {
				Some(prefix) => DynamicStrongId::new(prefix.clone(), case.2).unwrap(),
				None => DynamicStrongId::new_plain(case.2),
			};
			assert_eq!(&format!("{id}"), case.1);
			assert_eq!(*id.id(), case.2);

			let parsed = case.1.parse::<DynamicStrongId<usize>>().unwrap();
			assert_eq!(parsed.suffix, case.2);
			assert_eq!(parsed.prefix, case.0);
			assert_eq!(*parsed.id(), case.2);
		}
	}

	#[test]
	fn invalid_usize() {
		struct Case(&'static str, Error);
		let cases = vec![
			Case("dyn_0000000000", Error::InvalidLength(13, 10)),
			Case(
				"dyn_zzzzzzzzzzzzz",
				Error::Base32Error(Base32Error::InvalidFirstByte),
			),
			Case(
				"dyn_z000000000000",
				Error::Base32Error(Base32Error::InvalidFirstByte),
			),
			Case("09d", Error::InvalidLength(13, 3)),
			Case(
				"zzzzzzzzzzzzz",
				Error::Base32Error(Base32Error::InvalidFirstByte),
			),
			Case("0000000000", Error::InvalidLength(13, 10)),
		];

		for case in cases {
			let error = case.0.parse::<DynamicStrongId<usize>>().unwrap_err();
			assert_eq!(error, case.1);
		}
	}

	#[test]
	fn invalid_u8() {
		struct Case(&'static str, Error);
		let cases = vec![
			Case("dyn_000", Error::InvalidLength(2, 3)),
			Case("dyn_8f", Error::Base32Error(Base32Error::InvalidFirstByte)),
			Case("dyn_zz", Error::Base32Error(Base32Error::InvalidFirstByte)),
			Case("09d", Error::InvalidLength(2, 3)),
			Case("8f", Error::Base32Error(Base32Error::InvalidFirstByte)),
			Case("000", Error::InvalidLength(2, 3)),
			Case("0l", Error::Base32Error(Base32Error::InvalidByte)),
			Case("Case_00", Error::IncorrectPrefixCharacter('C')),
			Case("00numeric_00", Error::IncorrectPrefixCharacter('0')),
			Case("case0_00", Error::IncorrectPrefixCharacter('0')),
		];

		for case in cases {
			let error = case.0.parse::<DynamicStrongId<u8>>().unwrap_err();
			assert_eq!(error, case.1);
		}
	}

	#[cfg(feature = "serde")]
	#[test]
	fn u32_prefix_serde() {
		let value: DynamicStrongId<u32> = serde_json::from_str("\"prefix_000009d\"").unwrap();
		assert_eq!(value.prefix, Some("prefix".into()));
		assert_eq!(*value.id(), 301);

		let value = serde_json::to_string(&value).unwrap();
		assert_eq!("\"prefix_000009d\"", value);
	}
}
