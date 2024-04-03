extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr, Type};

fn assert_prefix_valid(prefix: &str) {
	assert!(prefix.len() < 64, "prefix is longer than 63 characters");

	for b in prefix.as_bytes() {
		if cfg!(feature = "delimited") && *b == b'_' {
			continue;
		}

		assert!(
			b.is_ascii_lowercase(),
			"prefix contains non ascii lowercase characters"
		);
	}
}

struct Attributes {
	prefix: Option<String>,
	suffix_type: Type,
}

fn from_input(input: &DeriveInput) -> Result<Attributes, syn::Error> {
	let mut prefix = None;
	let mut suffix = None;

	for attr in &input.attrs {
		if attr.path().is_ident("strong_id") {
			attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("prefix") {
					let value = meta.value()?.parse::<LitStr>()?.value();

					if value.is_empty() {
						prefix = None
					} else {
						prefix = Some(value)
					}
				}

				if meta.path.is_ident("suffix") {
					let value = meta.value()?.parse::<LitStr>()?.value();

					if value.is_empty() {
						suffix = None
					} else {
						suffix = Some(value)
					}
				}

				Ok(())
			})?;
		}
	}

	let field = match &input.data {
		Data::Struct(data_struct) => match &data_struct.fields {
			Fields::Unnamed(fields) => match fields.unnamed.first() {
				Some(field) => field,
				None => panic!("tuple should have a single field"),
			},
			_ => panic!("must be a tuple struct with a single field"),
		},
		_ => panic!("type must be a struct"),
	};

	let attributes = Attributes {
		prefix,
		suffix_type: field.ty.clone(),
	};

	Ok(attributes)
}

#[proc_macro_derive(StrongId, attributes(strong_id))]
pub fn derive_strong_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;

	let attributes = match from_input(&input) {
		Err(error) => {
			let error = error.to_compile_error();
			return quote!(#error).into();
		}
		Ok(attributes) => attributes,
	};

	let prefix_expr = match attributes.prefix {
		Some(prefix) => {
			assert_prefix_valid(&prefix);
			quote!(Some(#prefix))
		}
		None => {
			quote!(None)
		}
	};

	let suffix_type = attributes.suffix_type;
	let suffix_type = quote!(#suffix_type);

	let serde = if cfg!(feature = "serde") {
		quote! {
			impl ::strong_id::serde::Serialize for #name {
				 fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
				 where
					  S: ::strong_id::serde::Serializer,
				 {
					  serializer.serialize_str(&self.to_string())
				 }
			}

			impl<'de> ::strong_id::serde::Deserialize<'de> for #name {
				 fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
				 where
					  D: ::strong_id::serde::Deserializer<'de>,
				 {
					  ::std::string::String::deserialize(deserializer)?
						   .parse::<Self>()
						   .map_err(|error| ::strong_id::serde::de::Error::custom(error.to_string()))
				 }
			}
		}
	} else {
		quote!()
	};

	let expanded = quote! {
		impl ::strong_id::StrongId<#suffix_type> for #name {
			fn prefix(&self) -> Option<&str> {
				#prefix_expr
			}

			fn id(&self) -> &#suffix_type {
				&self.0
			}
		}

		#serde
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(StrongUuid)]
pub fn derive_strong_id_uuid(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;

	let uuid_v1_impl = if cfg!(feature = "uuid-v1") {
		quote! {
			fn new_v1(ts: ::strong_id::uuid::Timestamp, node_id: &[u8; 6]) -> Self {
				Self(::strong_id::uuid::Uuid::new_v1(ts, node_id))
			}

			fn now_v1(node_id: &[u8; 6]) -> Self {
				Self(::strong_id::uuid::Uuid::now_v1(node_id))
			}
		}
	} else {
		quote!()
	};

	let uuid_v3_impl = if cfg!(feature = "uuid-v3") {
		quote! {
			fn new_v3(namespace: &::strong_id::uuid::Uuid, name: &[u8]) -> Self {
				Self (::strong_id::uuid::Uuid::new_v3(namespace, name))
			}
		}
	} else {
		quote!()
	};

	let uuid_v4_impl = if cfg!(feature = "uuid-v4") {
		quote! {
			fn new_v4() -> Self {
				Self(::strong_id::uuid::Uuid::new_v4())
			}
		}
	} else {
		quote!()
	};

	let uuid_v5_impl = if cfg!(feature = "uuid-v5") {
		quote! {
			fn new_v5(namespace: &::strong_id::uuid::Uuid, name: &[u8]) -> Self {
				Self(::strong_id::uuid::Uuid::new_v5(namespace, name))
			}
		}
	} else {
		quote!()
	};

	let uuid_v6_impl = if cfg!(all(uuid_unstable, feature = "uuid-v6")) {
		quote! {
			fn new_v6(ts: ::strong_id::uuid::Timestamp, node_id: &[u8; 6]) -> Self {
				Self(::strong_id::uuid::Uuid::new_v6(ts, node_id))
			}

			fn now_v6(node_id: &[u8; 6]) -> Self {
				Self(::strong_id::uuid::Uuid::now_v6(node_id))
			}
		}
	} else {
		quote!()
	};

	let uuid_v7_impl = if cfg!(all(uuid_unstable, feature = "uuid-v7")) {
		quote! {
			fn new_v7(ts: ::strong_id::uuid::Timestamp) -> Self {
				Self(::strong_id::uuid::Uuid::new_v7(ts))
			}

			fn now_v7() -> Self {
				Self(::strong_id::uuid::Uuid::now_v7())
			}
		}
	} else {
		quote!()
	};

	let uuid_v8_impl = if cfg!(all(uuid_unstable, feature = "uuid-v8")) {
		quote! {
			fn new_v8(buf: [u8; 16]) -> Self {
				Self(::strong_id::uuid::Uuid::new_v8(buf))
			}
		}
	} else {
		quote!()
	};

	let expanded = quote! {
		impl ::strong_id::StrongUuid for #name {
			fn from_u128(v: u128) -> Self {
				Self(::strong_id::uuid::Uuid::from_u128(v))
			}

			#uuid_v1_impl
			#uuid_v3_impl
			#uuid_v4_impl
			#uuid_v5_impl
			#uuid_v6_impl
			#uuid_v7_impl
			#uuid_v8_impl
		}
	};

	proc_macro::TokenStream::from(expanded)
}

/// Validate `&'static str` prefixes at compile-time
///
/// ```
/// # fn main() {
/// # use strong_id_macros::prefix;
/// prefix!("user");
/// prefix!("user_account");
/// # }
///
///
/// // prefix!("USER");
/// ```
#[proc_macro]
pub fn prefix(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as LitStr);
	let value = input.value();

	let expanded = if value.is_empty() {
		quote!(None)
	} else {
		assert_prefix_valid(&value);
		quote!(#value)
	};

	proc_macro::TokenStream::from(expanded)
}
