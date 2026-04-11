# `postgres-static-analyzer-ddl-catalog-structs`

[![crates.io](https://img.shields.io/crates/v/postgres-static-analyzer-ddl-catalog-structs.svg)](https://crates.io/crates/postgres-static-analyzer-ddl-catalog-structs)
[![docs.rs](https://img.shields.io/docsrs/postgres-static-analyzer-ddl-catalog-structs.svg)](https://docs.rs/postgres-static-analyzer-ddl-catalog-structs)
<!-- [![license](https://img.shields.io/crates/l/postgres-static-analyzer-ddl-catalog-structs.svg)](https://github.com/blainehansen/postgres-static-analyzer/blob/main/LICENSE) -->

Struct definitions for all the [postgres catalog tables](https://www.postgresql.org/docs/17/catalogs.html) that are [**DDL only**](https://en.wikipedia.org/wiki/Data_definition_language), meaning only the tables and columns that describe the "schema" of the database are included. No `oid`s, no transient server state or like clustering or tablespaces etc, and of course no actual table data.

`oid`s pointing to other tables are translated either to strings (as [`SmolStr`](https://docs.rs/smol_str/latest/smol_str/struct.SmolStr.html)) if they aren't contained in a [namespace](https://www.postgresql.org/docs/17/catalog-pg-namespace.html) (confusingly created with `create schema`), or a "qualified name" struct [`Qual`](https://docs.rs/postgres-static-analyzer-ddl-catalog-structs/latest/struct.Qual.html) for those that are.
