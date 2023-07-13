# `strong_id`

[<img alt="github" src="https://img.shields.io/badge/johnnynotsolucky/strong_id-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/johnnynotsolucky/strong_id)
[<img alt="crates.io" src="https://img.shields.io/crates/v/strong_id.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/strong_id)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-strong_id-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/strong_id)
[<img alt="ci status" src="https://img.shields.io/github/actions/workflow/status/johnnynotsolucky/strong_id/ci.yaml?branch=main&style=for-the-badge&label=CI" height="20">](https://github.com/johnnynotsolucky/strong_id/actions/workflows/ci.yaml)
[<img alt="typeid spec status" src="https://img.shields.io/github/actions/workflow/status/johnnynotsolucky/strong_id/typeid_spec.yaml?branch=main&style=for-the-badge&label=TypeID" height="20">](https://github.com/johnnynotsolucky/strong_id/actions/workflows/typeid_spec.yaml)

Strongly typed IDs which optionally satisfy the [TypeID](https://github.com/jetpack-io/typeid) specification.

A StrongId is any type which implements `StrongId<T: Id>`. 

The `Id` trait is implemented for `u8`, `u16`, `u32`, `u64`, `u128`, `usize` and when the `"uuid"` feature is enabled, 
`Uuid`.


## TypeID

With `default-features = false` and the `typeid` feature enabled, StrongID will follow the TypeID specification. 

StrongID is verified against the TypeID spec with the 
[typeid](https://github.com/johnnynotsolucky/strong_id/actions/workflows/typeid_spec.yaml) workflow every 6 hours.

## Examples

### Dynamic StrongIds

#### ID with a prefix
```rust
use strong_id::{prefix, DynamicStrongId};

let user_id = DynamicStrongId::<u16>::new(prefix!("user"), 3203).unwrap();
println!("{}", user_id); // user_0343

let user_id = "user_0343".parse::<DynamicStrongId<u16>>().unwrap();
println!("{:#?}", user_id);
// DynamicStrongId {
//     prefix: Some(
//        Prefix {
//           inner: "user",
//        },
//     ),
//     suffix: 3203,
// }
```

#### ID without a prefix
```rust
use strong_id::{prefix, DynamicStrongId};

let id = DynamicStrongId::<u16>::new_plain(3203);
println!("{}", id); // 0343

let id = "0343".parse::<DynamicStrongId<u16>>().unwrap();
println!("{:#?}", id);
// DynamicStrongId {
//     prefix: None,
//     suffix: 3203,
// }
```

#### TypeId with a prefix

```rust
use strong_id::{prefix, DynamicStrongId};

let user_id = DynamicStrongId::<Uuid>::now_v7(prefix!("user")).unwrap();
println!("{}", user_id); // user_01h536gfwffx2rm6pa0xg63337

let user_id = "user_01h536gfwffx2rm6pa0xg63337"
  .parse::<DynamicStrongId<Uuid>>()
  .unwrap();
println!("{:#?}", user_id);
// DynamicStrongId {
//     prefix: Some(
//        Prefix {
//           inner: "user",
//        },
//     ),
//     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
// }
```

#### TypeId without a prefix

```rust
use strong_id::{prefix, DynamicStrongId};

let id = DynamicStrongId::<Uuid>::now_v7_plain();
println!("{}", id); // 01h536gfwffx2rm6pa0xg63337

let id = "01h536gfwffx2rm6pa0xg63337"
  .parse::<DynamicStrongId<Uuid>>()
  .unwrap();
println!("{:#?}", id);
// DynamicStrongId {
//     prefix: None,
//     suffix: 01894668-3f8f-7f45-8a1a-ca0760618c67,
// }
```

### Generated StrongIds

#### ID with a prefix
```rust
use strong_id::strong_id;

strong_id!(pub struct UserId(u16 => "user"));

let user_id = UserId::from(3203);
println!("{}", user_id); // user_0343

let user_id = "user_0343".parse::<UserId>().unwrap();
println!("{:#?}", user_id);
// UserId {
//     suffix: 3203,
// }
```

#### ID without a prefix

```rust
use strong_id::strong_id;

strong_id!(pub struct Id(u16));

let id = Id::from(3203);
println!("{}", id); // user_0343

let id = "0343".parse::<Id>().unwrap();
println!("{:#?}", id);
// Id {
//     suffix: 3203,
// }
```

#### Generated TypeId with a prefix

```rust
use strong_id::{strong_uuid, StrongUuid};

strong_uuid!(pub struct UserId(pub Uuid => "user"));
// strong_uuid!(struct UserId(Uuid => "user"));
/*
strong_id! {
    #[derive(StrongUuid)]
    pub struct UserId(pub Uuid => "user")    
}
*/

let user_id = UserId::now_v7();
println!("{}", user_id); // user_01h536z8abez196j2nzz06y8c8

let user_id = "user_01h536z8abez196j2nzz06y8c8".parse::<UserId>().unwrap();
println!("{:#?}", user_id);
// UserId {
//     suffix: 0189466f-a14b-77c2-9348-55ffc06f2188,
// }
```

#### Generated TypeId without a prefix

```rust
use strong_id::{strong_uuid, StrongUuid};

strong_uuid!(pub struct Id(pub Uuid));
// strong_uuid!(struct Id(Uuid));
/*
strong_id! {
    #[derive(StrongUuid)]
    pub struct Id(pub Uuid)    
}
*/

let id = Id::now_v7();
println!("{}", id); // 01h5372sq2egxb6ps3taq7p6np

let id = "01h5372sq2egxb6ps3taq7p6np".parse::<Id>().unwrap();
println!("{:#?}", id);
// UserId {
//     suffix: 01894671-66e2-743a-b35b-23d2ae7b1ab6,
// }
```

## Features

- `delimited` - Enables underscore delimited prefixes. On by default.
- `serde` - Enables serde support in code generation.
- `uuid` - Enable uuid functionality.
  - `uuid-v1` - corresponds with uuid "v1" feature
  - `uuid-v3` - corresponds with uuid "v3" feature
  - `uuid-v4` - corresponds with uuid "v4" feature
  - `uuid-v5` - corresponds with uuid "v5" feature
  - `uuid-v6` - corresponds with uuid "v6" feature
  - `uuid-v7` - corresponds with uuid "v7" feature
  - `uuid-v8` - corresponds with uuid "v8" feature
- `typeid` - Enable features which satisfy the TypeId specification.
