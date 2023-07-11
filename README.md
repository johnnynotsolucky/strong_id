# `strong_id`

[<img alt="github" src="https://img.shields.io/badge/johnnynotsolucky/strong_id-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/johnnynotsolucky/strong_id)
[<img alt="crates.io" src="https://img.shields.io/crates/v/strong_id.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/strong_id)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-strong_id-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/strong_id)
[<img alt="ci status" src="https://img.shields.io/github/actions/workflow/status/johnnynotsolucky/strong_id/ci.yaml?branch=main&style=for-the-badge" height="20">](https://github.com/johnnynotsolucky/strong_id/actions/workflows/ci.yaml)
[<img alt="typeid spec status" src="https://img.shields.io/github/actions/workflow/status/johnnynotsolucky/strong_id/typeid_spec.yaml?branch=main&style=for-the-badge" height="20">](https://github.com/johnnynotsolucky/strong_id/actions/workflows/typeid_spec.yaml)

Strongly typed IDs which optionally satisfy the [TypeID](https://github.com/jetpack-io/typeid) specification.

## TODO

A StrongId is any type which implements `StrongId<T: Id>`. 

`strong_id` implements traits for `u8`, `u16`, `u32`, `u64`, `u128`, `usize` and with the `"uuid"` feature, `Uuid`.

