use super::explain::ExplainPlan;

#[derive(Clone, Debug)]
pub struct QueryFingerprint {
    pub vector: Vec<f32>, // Feature vector for clustering
    pub query_id: String,
    pub plan_hash: String,
}

pub fn generate_fingerprint(plan: &ExplainPlan, query_id: &str) -> QueryFingerprint {
    let mut vector = vec![];
    // Extract features: operator counts, costs, row estimates
    let mut op_counts: HashMap<String, u32> = HashMap::new();
    for op in &plan.operators {
        *op_counts.entry(op.kind.clone()).or_insert(0) += 1;
        vector.push(op.cost);
        vector.push(op.rows as f32);
    }
    vector.push(plan.total_cost);
    vector.push(plan.estimated_rows as f32);

    // Normalize vector (simplified)
    let max_val = vector.iter().fold(f32::MIN, |a, &b| a.max(b));
    if max_val > 0.0 {
        vector = vector.iter().map(|x| x / max_val).collect();
    }

    QueryFingerprint {
        vector,
        query_id: query_id.to_string(),
        plan_hash: format!("{:x}", md5::compute(format!("{:?}", plan))),
    }
}