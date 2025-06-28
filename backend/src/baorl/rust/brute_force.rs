use crate::plansense::fingerprint::ExplainPlan;

pub struct BruteForceSelector {
    sla_latency_ms: f32,
    sla_budget: f32,
}

impl BruteForceSelector {
    pub fn new(sla_latency_ms: f32, sla_budget: f32) -> Self {
        BruteForceSelector { sla_latency_ms, sla_budget }
    }

    pub fn select_plan(&self, plans: &[ExplainPlan]) -> Option<(String, f32, f32)> {
        let mut best_plan = None;
        let mut best_score = f32::MAX;

        for (i, plan) in plans.iter().enumerate() {
            // Assume latency is proportional to total_cost (simplified)
            let latency = plan.total_cost * 0.01; // Placeholder
            let cost = plan.total_cost * 0.001; // Placeholder
            if latency <= self.sla_latency_ms && cost <= self.sla_budget {
                let score = latency + cost; // Simple objective
                if score < best_score {
                    best_score = score;
                    best_plan = Some((format!("plan{}", i), latency, cost));
                }
            }
        }

        best_plan
    }
}