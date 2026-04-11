# `postgres-static-analyzer`

TODO note that foreign server user mapping reflection will change based on what user is performing the reflection!


This library gives tools for *statically* analyzing postgres sql code for type checking, permissions checking, and diff generation. It can't detect whether *data* operations will always succeed based on *constraints*, but it can detect if they will succeed based on inferred types.

Here are some of the top level types and functions:

`DbState`

A type declaratively representing what database objects exist in a database (data isn't included).

`SqlBlock`

A type representing a chunk of sql commands. (is this just an alias or a trait or something?)

```rust
struct ApplyOutcome {
  db_state: DbState,
  destroys_objects: bool,
  mutates_objects: bool,
  destroys_data: bool,
  mutates_data: bool,
  errors: Vec<Error>,
}

struct RejectFlags {
  destroy_objects: bool,
  destroy_data: bool,
  mutate_objects: bool,
  mutate_data: bool,
}
impl Default for RejectFlags {
  fn default() -> Self {
    Self { destroy_objects: false, destroy_data: false, mutate_objects: false, mutate_data: false }
  }
}
```

---

`try_seq(seq: Vec<SqlBlock>, stop_on_error: bool) -> ApplyOutcome`

This runs through the sequence of sql blocks, checking if each is valid, assuming starting from a "default" database, at the end returning the final state the database would be in, given errors are rejected and so don't occur, including the entirety of failed transactions, or if stopped at the first error.

<!-- probably better idea to include a const SqlBlock representing various different default states of databases -->

It also emits whether any objects or data are destroyed or mutated, and which ones.

---

`demand_seq(seq: Vec<SqlBlock>, reject_options: RejectFlags) -> Result<DbState, Error>`

Same as above, but assumes `stop_on_error = true`, and returns a `Result` containing that first `Error` if encountered.


---

`try_role_seq(seq: Vec<(Role, SqlBlock)>) -> (DbState, Vec<Error>)`

This checks that the given sequence of sql blocks, each with an adjoining `Role` designating *who* is executing them, is valid. Doing this allows permission checking alongside normal type checking.

---

`demand_role_seq(seq: Vec<(Role, SqlBlock)>) -> Result<DbState, Error>`

---

`compute_diff(from_seq: Vec<SqlBlock>, to_seq: Vec<SqlBlock>) -> SqlBlock`

Determines the sql block necessary to migrate from the state implied by `from_seq` to that implied by `to_seq`.

---

`reflect_db_url(db_url: impl AsRef<str>) -> impl Future<Output=postgres::Result<DbState>>`
`reflect_db_config(db_config: &postgres::Config) -> impl Future<Output=postgres::Result<DbState>>`
`reflect_db_client(db_client: impl postgres::GenericClient) -> impl Future<Output=postgres::Result<DbState>>`

Connects to the database and reads its actual current state.

Since `DbState` implements `Into<SqlBlock>`, you can use this to run `compute_diff` against the state of a real db.

---

`backfill_missing(target: SqlBlock) -> Result<SqlBlock, Error>`

figures out a command block necessary to ensure that `target` could succeed, if that's at all possible (`target` isn't self-contradictory or simply malformed)

<!--
this becomes a lot more straightforward if you can go from Error to SqlBlock! if Error has enough information to specify "missing" objects
missing objects are th only ones you can do this for, as long as they're specified in sufficient detail
so if there's a discrete Error type for missing objects with enough information about what the thing they were *trying* to reference was, then you can reconstruct it. the hard part is you need to infer at the missing site any necessary type information about what's needed

this is acceptable is you have a "NotImplemented" variant of the error being returned from backfill_missing, so that you can punt certain kinds of possible but annoying reconstructions
-->

---


`apply_command(db_state: DbState, sql_command: SqlCommand) -> ApplyOutcome`


---

How have you ensured this library is correct?


to e2e test `compute_diff`, you randomly generate some (valid) DbState (or for every kind of item in the DbState), then `compute_diff` from empty to it, then apply that diff to a real database, then `reflect_db_client` and ensure the two states are exactly the same

Then with `compute_diff` in hand, then you can e2e test the above `seq` functions, by randomly generating any two DbStates, computing the diff between them, and demanding that it is valid, and that you can execute all those states/diffs and end up in the right place (a form of further testing for `compute_diff`).

And given the presence of `backfill_missing`, it's possible for every kind of top level command enumerated by sqlparser, to create a dummy version of it, backfill whatever's necessary for it, and then demand that the `seq` functions line up with the others

Or rather the better way to think about this is using these techniques we can generate a massive corpus of test examples.


Also if you can be given a `DbState` and randomly generate a command to run against it, then it's actually fine to have ones that are both invalid and valid, since you can just demand that the validity found by the library is the same as that for the real database.

https://readyset.io/blog/stateful-property-testing-in-rust
https://proptest-rs.github.io/proptest/
https://crates.io/crates/proptest-stateful

https://proptest-rs.github.io/proptest/proptest/tutorial/transforming-strategies.html



---



default privileges for objects as witnessed by acldefault

to sum up:
the owner is given all privileges.
column privileges aren't specified in situations where table privileges are in place that make them unnecessary?
by default, public has TEMPORARY, CONNECT on all databases
by default, public has EXECUTE on all functions
by default, public has USAGE on all languages
by default, public has USAGE on all types/domains

```
echo 'COLUMN'
PGPASSWORD='devpassword' psql -U devuser -h localhost devdb <<EOF
select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('c', 'devuser'::regrole::oid));

select
  case when grantee = 0 then 'public' else pg_get_userbyid(grantee)::text end as grantee,
  privilege_type, is_grantable,
  pg_get_userbyid(grantor)::text as grantor
from aclexplode(acldefault('c', 'a'::regrole::oid));
EOF

etc...
```
