use rand::Rng;

pub struct SLAWorkload {
    pub plans: Vec<(String, f32, f32)>, // (plan_hash, cost, latency)
    pub sla_latency_ms: f32,
    pub sla_budget: f32,
}

impl SLAWorkload {
    pub fn generate_tpch_skew(n_plans: usize, skew_factor: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut plans = vec![];
        for i in 0..n_plans {
            let cost = rng.gen_range(10.0..1000.0) * (1.0 + skew_factor);
            let latency = cost * 0.01; // Simplified
            plans.push((format!("tpch_plan{}", i), cost, latency));
        }
        SLAWorkload {
            plans,
            sla_latency_ms: 5.0,
            sla_budget: 0.02,
        }
    }

    pub fn generate_ycsb(n_plans: usize, write_ratio: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut plans = vec![];
        for i in 0..n_plans {
            let cost = rng.gen_range(10.0..500.0) * (1.0 + write_ratio);
            let latency = cost * 0.015; // Write-heavy plans have higher latency
            plans.push((format!("ycsb_plan{}", i), cost, latency));
        }
        SLAWorkload {
            plans,
            sla_latency_ms: 3.0,
            sla_budget: 0.01,
        }
    }
}