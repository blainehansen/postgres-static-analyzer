# `postgres-static-analyzer`

The goal of this repo is to produce a static analyzer for postgres.

:warning: No static analyzer exists yet, I'm working up to it in stages. :warning:

If you're curious about what exactly a postgres static analyzer is and why I'm building one, [read this blog post](https://blainehansen.me/post/database-static-analyzers/).

The repo contains the following crates which could be used today:

- [`postgres-static-analyzer-ddl-catalog-structs`](https://github.com/blainehansen/postgres-static-analyzer/blob/main/catalog_structs)
- [`postgres-static-analyzer-reflect`](https://github.com/blainehansen/postgres-static-analyzer/blob/main/reflect)

<!--
### `temp_container_utils`

[README](TODO) | [crates.io]() | [docs.rs]()
 -->

## Future crates

The long term plan is to also make crates to:

- Use [`arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/) to generate random database schema states (defined in terms of `ddl-catalog-structs`), and random [sql commands](https://www.postgresql.org/docs/17/sql-commands.html) (both valid and invalid) given a database schema state.
- Build a schema diff tool reminiscent of [migra](https://github.com/djrobstep/migra), but that is fully static (doesn't require a running database). Will use [stateful property-based tests](https://readyset.io/blog/stateful-property-testing-in-rust) to validate the implementation against the real postgres.
- Build the actual static analyzer! Will also use [stateful property-based tests](https://readyset.io/blog/stateful-property-testing-in-rust) to validate the implementation against the real postgres.

## Generation of structs and reflection functions

Since there are so many catalog tables and so many columns in them (and I didn't want to blindly trust an LLM), the struct and reflection function definitions are largely generated using the [`gen_pg_info_decisions.ts` script](https://github.com/blainehansen/postgres-static-analyzer/blob/main/gen_pg_info_decisions.ts) alongside some [overrides](https://github.com/blainehansen/postgres-static-analyzer/blob/main/gen_pg_info_decisions.pre.toml) and a handful of tables where I wrote the struct and reflection functions manually.

The script encodes a bunch of heuristics that seem reasonable and produce successful reflection of the (not completely exhaustive) [`populate_all.sql` schema](https://github.com/blainehansen/postgres-static-analyzer/blob/main/reflect/populate_all.sql) (which was generated using an LLM).

I haven't personally audited every single column decision, but a representation of them is stored in [`gen_pg_info_decisions.toml`](https://github.com/blainehansen/postgres-static-analyzer/blob/main/gen_pg_info_decisions.toml), so (human-checked) insight and contributions are appreciated!

The reflection results are snapshot tested using [insta](https://github.com/mitsuhiko/insta).
