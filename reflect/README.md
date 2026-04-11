# `postgres-static-analyzer-reflect`

[![crates.io](https://img.shields.io/crates/v/postgres-static-analyzer-reflect.svg)](https://crates.io/crates/postgres-static-analyzer-reflect)
[![docs.rs](https://img.shields.io/docsrs/postgres-static-analyzer-reflect.svg)](https://docs.rs/crates/postgres-static-analyzer-reflect)
<!-- [![license](https://img.shields.io/crates/l/postgres-static-analyzer-ddl-catalog-structs.svg)](https://github.com/blainehansen/postgres-static-analyzer/blob/main/LICENSE) -->

Functions to actually read the catalog tables from a running postgres instance, and turn them into `ddl-catalog-structs`.

Uses [`tokio-postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/) through [`clorinde`](https://crates.io/crates/clorinde).
