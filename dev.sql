drop table if exists aaa cascade;
drop table if exists bbb cascade;

create table aaa (
	aaa_a int, aaa_b int not null,
	unique (aaa_a, aaa_b),
	aaa_c int unique
);
create table bbb (
	bbb_a int not null, bbb_b int not null,
	foreign key (bbb_a, bbb_b) references aaa (aaa_a, aaa_b),
	bbb_c int references aaa(aaa_c)
);

select
	pg_constraint.conname::text,
	referring_sch.nspname::text as referring_schema,
	referring_tab.relname::text as referring_table,
	array_agg(referring_col.attname::text order by u1.ordinality) as referring_columns,
	referred_sch.nspname::text as referred_schema,
	referred_tab.relname::text as referred_table,
	array_agg(referred_col.attname::text order by u2.ordinality) as referred_columns
from
	pg_constraint
	join pg_class as referring_tab on pg_constraint.conrelid = referring_tab.oid
	join pg_namespace as referring_sch on referring_tab.relnamespace = referring_sch.oid
	join pg_class as referred_tab on pg_constraint.confrelid = referred_tab.oid
	join pg_namespace as referred_sch on referred_tab.relnamespace = referred_sch.oid
	cross join lateral unnest(pg_constraint.conkey) with ordinality as u1(attnum, ordinality)
	cross join lateral unnest(pg_constraint.confkey) with ordinality as u2(attnum, ordinality)
	join pg_attribute as referring_col on referring_col.attrelid = referring_tab.oid and referring_col.attnum = u1.attnum
	join pg_attribute as referred_col on referred_col.attrelid = referred_tab.oid and referred_col.attnum = u2.attnum
where
	pg_constraint.contype = 'f' and u1.ordinality = u2.ordinality
	and referring_sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
	and referred_sch.nspname not in ('pg_catalog', 'information_schema', 'pg_toast')
group by pg_constraint.conname, referring_sch.nspname, referring_tab.relname, referred_sch.nspname, referred_tab.relname, pg_constraint.oid
;
