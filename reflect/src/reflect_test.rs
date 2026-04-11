use super::*;

#[tokio::test]
async fn test_reflect_pg_state() -> anyhow::Result<()> {
	temp_container_utils::with_temp_postgres_client(async |_, _, client| {
		let populate_all_sql = tokio::fs::read_to_string("./populate_all.sql").await?;
		client.batch_execute(&populate_all_sql).await?;

		let db_state = reflect_db_state(&client).await?;

		insta::assert_ron_snapshot!(
			db_state,
			{
				".**" => insta::sorted_redaction(),
			}
		);

		Ok::<_, anyhow::Error>(())
	}).await??;

	let snapshot_content = tokio::fs::read_to_string("./src/snapshots/postgres_static_analyzer_reflect__reflect_test__reflect_pg_state.snap").await?;
	assert!(!snapshot_content.contains("schema_name: \"pg_toast"));
	assert!(!snapshot_content.contains("schema_name: \"pg_temp"));
	assert!(!snapshot_content.contains(": \"pg_catalog."));

	assert!(snapshot_content.contains("aggfnoid: \"my_schema.catalog_sum(numeric)\","));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON DATABASE tempdb")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON SCHEMA my_schema")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TABLE parent_table")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON COLUMN parent_table.status")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON COLUMN parent_table.active_period")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON FUNCTION add_tax(numeric)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON SEQUENCE table_id_seq")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON VIEW parent_view")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TYPE status_enum")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TYPE float_range")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON INDEX parent_name_btree_idx")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TEXT SEARCH CONFIGURATION custom_ts_conf")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON OPERATOR = (point_composite, point_composite)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON ACCESS METHOD custom_index_am")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON AGGREGATE first_value_agg(anycompatible)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON CAST (text AS point_composite)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON COLLATION custom_collation")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON COLUMN parent_table.metrics")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON CONSTRAINT unique_name ON parent_table")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON CONSTRAINT con_point_composite_safe ON DOMAIN point_composite_safe")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON CONVERSION custom_conv")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON DOMAIN point_composite_safe")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON EXTENSION pageinspect")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON EXTENSION postgres_fdw")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON EVENT TRIGGER log_ddl_events")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON FOREIGN DATA WRAPPER postgres_fdw")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON FOREIGN TABLE ext_table")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON FUNCTION generate_ids(integer)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON INDEX parent_status_hash_idx")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON MATERIALIZED VIEW thing")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON OPERATOR < (point_composite, point_composite)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON OPERATOR CLASS custom_op_class USING btree")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON OPERATOR FAMILY custom_op_family USING btree")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON POLICY active_only_policy ON parent_table")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON LANGUAGE plpgsql")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON PROCEDURE archive_old_records(date)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON PUBLICATION test_pub")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON ROLE catalog_admin")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON ROUTINE add(integer, integer)")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON RULE parent_view_insert ON parent_view")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON SERVER ext_server")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON STATISTICS parent_table_stats")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON SUBSCRIPTION test_sub")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TABLE audit_log")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TEXT SEARCH DICTIONARY custom_dict")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TRIGGER parent_audit_trig ON parent_table")"#));
	assert!(snapshot_content.contains(r#"Some("COMMENT ON TYPE point_composite")"#));

	Ok(())
}
