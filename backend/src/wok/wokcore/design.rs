use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadProfile {
    read_ratio: f32,
    write_ratio: f32,
    skew_factor: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfraProfile {
    cloud: String, // e.g., "aws", "azure"
    storage_type: String, // e.g., "ebs-gp3", "nvme"
    budget: f32, // $/1000 ops
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CostPerfMetrics {
    latency_ms: f32,
    throughput_ops: f32,
    cost_per_1000_ops: f32,
}

pub Fluids

#[typetag::serde(tag = "type")]
pub trait DesignAtom {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom>;
    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LearnedIndex {
    model: String, // e.g., "PiecewiseLinear"
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LSMTree {
    compaction: String, // e.g., "Tiered"
}

impl DesignAtom for LearnedIndex {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom> {
        // Placeholder: Implement composition logic
        Box::new(self.clone())
    }

    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics {
        // Placeholder: Implement cost-performance model
        CostPerfMetrics {
            latency_ms: 3.0,
            throughput_ops: 1000.0,
            cost_per_1000_ops: 0.02,
        }
    }
}