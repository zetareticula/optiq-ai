use super::design::{DesignAtom, WorkloadProfile, InfraProfile, CostPerfMetrics};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct COOPBuffer {
    capacity: u64, // in bytes
    delta_buffer: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearnedIndex {
    model: String, // e.g., "PGM", "FITing-Tree"
    coop_buffer: COOPBuffer,
}

impl LearnedIndex {
    pub fn new(model: &str, capacity: u64) -> Self {
        LearnedIndex {
            model: model.to_string(),
            coop_buffer: COOPBuffer {
                capacity,
                delta_buffer: vec![],
            },
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), String> {
        if self.coop_buffer.delta_buffer.len() as u64 + data.len() as u64 <= self.coop_buffer.capacity {
            self.coop_buffer.delta_buffer.extend_from_slice(data);
            Ok(())
        } else {
            // Trigger flush or retraining (simplified)
            self.coop_buffer.delta_buffer.clear();
            Err("COOP buffer full, flushed".to_string())
        }
    }
}

#[typetag::serde]
impl DesignAtom for LearnedIndex {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom> {
        Box::new(ClearnedStructure {
            learned: self.model.clone(),
            classical: "LSMTree".to_string(),
            accelerators: vec![Accelerator {
                kind: "Buffer".to_string(),
                config: HashMap::from([("size".to_string(), format!("{}", self.coop_buffer.capacity))]),
            }],
        })
    }

    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics {
        let io_model = super::wokeval::DistributionAwareIOModel::new();
        let mut metrics = io_model.evaluate(self, workload, infra);
        // COOP optimization: reduce write latency by 70%
        if workload.write_ratio > 0.0 {
            metrics.latency_ms *= 0.3;
            metrics.throughput_ops *= 1.7;
        }
        metrics
    }

    fn get_layout(&self) -> Vec<DataStructure> {
        vec![DataStructure::Learned {
            kind: self.model.clone(),
        }]
    }
}