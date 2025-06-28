#[cfg(test)]
mod tests {
    use super::super::super::plansense::clustering::dbscan::DBSCAN;
    use super::super::super::plansense::fingerprint::{ExplainPlan, Operator, QueryFingerprint, generate_fingerprint};

    #[test]
    fn test_dbscan_clustering() {
        let plans = vec![
            ExplainPlan {
                operators: vec![Operator { kind: "Seq Scan".to_string(), cost: 100.0, rows: 1000 }],
                total_cost: 100.0,
                estimated_rows: 1000,
            },
            ExplainPlan {
                operators: vec![Operator { kind: "Seq Scan".to_string(), cost: 110.0, rows: 1100 }],
                total_cost: 110.0,
                estimated_rows: 1100,
            },
            ExplainPlan {
                operators: vec![Operator { kind: "Index Scan".to_string(), cost: 50.0, rows: 100 }],
                total_cost: 50.0,
                estimated_rows: 100,
            },
        ];

        let fingerprints: Vec<QueryFingerprint> = plans
            .iter()
            .enumerate()
            .map(|(i, plan)| generate_fingerprint(plan, &format!("q{}", i)))
            .collect();

        let dbscan = DBSCAN::new(0.5, 2);
        let labels = dbscan.cluster(&fingerprints);

        // Expect two clusters: Seq Scan plans together, Index Scan as outlier
        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0], labels[1]); // Same cluster
        assert_ne!(labels[0], labels[2]); // Different cluster
    }
}