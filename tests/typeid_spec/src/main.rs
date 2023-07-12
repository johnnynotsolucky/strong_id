use libtest_mimic::{Arguments, Trial};
use serde::{de::DeserializeOwned, Deserialize};
use std::str::FromStr;
use strong_id::{strong_uuid, DynamicStrongId, StrongId};
use uuid::Uuid;

pub trait Case {
	const NAME: &'static str;
}

#[derive(Clone, Debug, Deserialize)]
struct ValidCase {
	name: String,
	typeid: String,
	prefix: String,
	uuid: String,
}

impl Case for ValidCase {
	const NAME: &'static str = "valid";
}

impl ValidCase {
	fn prefix(&self) -> Option<String> {
		if self.prefix.is_empty() {
			None
		} else {
			Some(self.prefix.clone())
		}
	}
}

#[derive(Clone, Debug, Deserialize)]
struct InvalidCase {
	name: String,
	typeid: String,
	description: String,
}

impl Case for InvalidCase {
	const NAME: &'static str = "invalid";
}

fn fetch_cases<T: Case + DeserializeOwned>() -> Vec<T> {
	let case = T::NAME;
	reqwest::blocking::get(format!(
		"https://raw.githubusercontent.com/jetpack-io/typeid/main/spec/{case}.json"
	))
	.unwrap()
	.json::<Vec<T>>()
	.unwrap()
}

strong_uuid!(struct NoPrefix);
strong_uuid!(struct Prefix("prefix"));

fn main() {
	let valid_cases = fetch_cases::<ValidCase>();
	let invalid_cases = fetch_cases::<InvalidCase>();

	let tests = valid_cases
		.clone()
		.into_iter()
		.map(|case| {
			Trial::test(format!("valid::dynamic::{}", case.name), move || {
				let uuid = Uuid::from_str(&case.uuid).unwrap();
				// encode
				let encoded = match case.prefix() {
					Some(prefix) => DynamicStrongId::new(prefix, uuid).unwrap(),
					None => DynamicStrongId::new_plain(uuid),
				};
				assert_eq!(encoded.to_string(), case.typeid);
				assert_eq!(*encoded.id(), uuid);

				// decode
				let encoded = DynamicStrongId::<Uuid>::from_str(&case.typeid).unwrap();
				assert_eq!(encoded.to_string(), case.typeid);
				assert_eq!(*encoded.id(), uuid);
				assert_eq!(encoded.prefix(), case.prefix().as_deref());

				Ok(())
			})
		})
		.chain(valid_cases.into_iter().map(|case| {
			Trial::test(format!("valid::static::{}", case.name), move || {
				match case.prefix() {
					Some(_prefix) => {
						let uuid = Uuid::from_str(&case.uuid).unwrap();
						// encode
						let encoded = Prefix::from(uuid);
						assert_eq!(encoded.to_string(), case.typeid);
						assert_eq!(*encoded.id(), uuid);

						// decode
						let encoded = Prefix::from_str(&case.typeid).unwrap();
						assert_eq!(encoded.to_string(), case.typeid);
						assert_eq!(*encoded.id(), uuid);
					}
					None => {
						let uuid = Uuid::from_str(&case.uuid).unwrap();
						// encode
						let encoded = NoPrefix::from(uuid);
						assert_eq!(encoded.to_string(), case.typeid);
						assert_eq!(*encoded.id(), uuid);

						// decode
						let encoded = NoPrefix::from_str(&case.typeid).unwrap();
						assert_eq!(encoded.to_string(), case.typeid);
						assert_eq!(*encoded.id(), uuid);
					}
				}

				Ok(())
			})
		}))
		.chain(invalid_cases.clone().into_iter().map(|case| {
			Trial::test(format!("invalid::dynamic::{}", case.name), move || {
				if DynamicStrongId::<Uuid>::from_str(&case.typeid).is_ok() {
					return Err(case.description.into());
				}

				Ok(())
			})
		}))
		.chain(invalid_cases.into_iter().map(|case| {
			Trial::test(format!("invalid::static::{}", case.name), move || {
				if Prefix::from_str(&case.typeid).is_ok() {
					return Err(case.description.into());
				}

				Ok(())
			})
		}))
		.collect::<Vec<_>>();

	let args = Arguments::from_args();
	libtest_mimic::run(&args, tests).exit_if_failed();
}
