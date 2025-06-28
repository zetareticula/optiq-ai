use pgx::prelude::*;
use pgx::pg_sys;
use pgx::pg_module_magic;
use pgx::pg_extern;
use pgx::spi::Spi;

pg_module_magic!();

// This module provides a PostgreSQL extension to fetch and analyze query plans
// using the pg_stat_statements extension.
#[pg_schema]
mod plansense {
    pub mod fingerprint {
        pub mod explain {
            use serde_json::Value;

            // Function to parse EXPLAIN output from JSON
            pub fn parse_explain(json: &str) -> Result<ExplainPlan, String> {
                let value: Value = serde_json::from_str(json)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                let plan = value.get("Plan").ok_or("Missing 'Plan' key")?;
                Ok(ExplainPlan {
                    node_type: plan.get("Node Type").and_then(Value::as_str).unwrap_or("").to_string(),
                    total_cost: plan.get("Total Cost").and_then(Value::as_f64).unwrap_or(0.0),
                    plan_rows: plan.get("Plan Rows").and_then(Value::as_u64).unwrap_or(0) as u32,
                })
            }

            #[derive(Debug)]
            pub struct ExplainPlan {
                pub node_type: String,
                pub total_cost: f64,
                pub plan_rows: u32,
            }
        }
    }
}

#[pg_extern]
fn get_explain_output(query_id: &str) -> String {
    // Placeholder: Execute EXPLAIN on query from pg_stat_statements
    // In practice, use SPI to run EXPLAIN (JSON)
    format!(
        r#"{{"Plan": {{"Node Type": "Seq Scan", "Total Cost": 100.0, "Plan Rows": 1000}}}}"#
    )
}

pub fn fetch_plan(query_id: &str) -> Result<super::plansense::fingerprint::ExplainPlan, String> {
    let json = get_explain_output(query_id);
    super::plansense::fingerprint::explain::parse_explain(&json)
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use super::*;
    use pgx::pg_test;

    #[pg_test]
    fn test_get_explain_output() {
        let query_id = "test_query";
        let output = get_explain_output(query_id);
        assert!(output.contains("Node Type"));
        assert!(output.contains("Total Cost"));
        assert!(output.contains("Plan Rows"));
    }

    #[pg_test]
    fn test_parse_explain() {
        let json = r#"{"Plan": {"Node Type": "Seq Scan", "Total Cost": 100.0, "Plan Rows": 1000}}"#;
        let plan = plansense::fingerprint::explain::parse_explain(json).unwrap();
        assert_eq!(plan.node_type, "Seq Scan");
        assert_eq!(plan.total_cost, 100.0);
        assert_eq!(plan.plan_rows, 1000);
    }
}