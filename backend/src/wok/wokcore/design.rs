use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Workload and infrastructure profiles
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadProfile {
    read_ratio: f32,
    write_ratio: f32,
    skew_factor: f32,
    dataset_size: u64, // in bytes
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfraProfile {
    cloud: String, // e.g., "aws", "azure"
    storage_type: String, // e.g., "ebs-gp3", "nvme"
    budget: f32, // $/1000 ops
    cpu_cores: u32,
    memory_mb: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CostPerfMetrics {
    latency_ms: f32,
    throughput_ops: f32,
    cost_per_1000_ops: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataStructure {
    Learned { kind: String }, // PGM, FITing-Tree, RadixSpline
    Classical { kind: String }, // LSMTree, BTree, LSHTable
    Cleared { learned: String, classical: String }, // Hybrid
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Accelerator {
    kind: String, // e.g., "Buffer", "Filter", "Cache"
    config: HashMap<String, String>, // e.g., {"size": "5MB", "type": "ARC"}
}

#[typetag::serde(tag = "type")]
pub trait DesignAtom {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom>;
    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics;
    fn get_layout(&self) -> Vec<DataStructure>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LSMTree {
    compaction: String,
    accelerators: Vec<Accelerator>,
}

#[typetag::serde]
impl DesignAtom for LSMTree {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom> {
        let other_layout = other.get_layout();
        let mut new_accelerators = self.accelerators.clone();
        new_accelerators.extend(other_layout.iter().filter_map(|ds| {
            match ds {
                DataStructure::Learned { kind } => Some(Accelerator {
                    kind: "LearnedIndex".to_string(),
                    config: HashMap::from([("type".to_string(), kind.clone())]),
                }),
                _ => None,
            }
        }));

        if other_layout.iter().any(|ds| matches!(ds, DataStructure::Learned { .. })) {
            Box::new(ClearnedStructure {
                learned: other_layout.iter().find_map(|ds| {
                    if let DataStructure::Learned { kind } = ds {
                        Some(kind.clone())
                    } else {
                        None
                    }
                }).unwrap_or("PGM".to_string()),
                classical: self.compaction.clone(),
                accelerators: new_accelerators,
            })
        } else {
            Box::new(LSMTree {
                compaction: self.compaction.clone(),
                accelerators: new_accelerators,
            })
        }
    }

    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics {
        let io_model = super::wokeval::DistributionAwareIOModel::new();
        io_model.evaluate(self, workload, infra)
    }

    fn get_layout(&self) -> Vec<DataStructure> {
        vec![DataStructure::Classical {
            kind: "LSMTree".to_string(),
        }]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClearedStructure {
    learned: String,
    classical: String,
    accelerators: Vec<Accelerator>,
}

#[typetag::serde]
impl DesignAtom for ClearedStructure {
    fn compose(&self, other: &dyn DesignAtom) -> Box<dyn DesignAtom> {
        let mut new_accelerators = self.accelerators.clone();
        new_accelerators.extend(other.get_layout().iter().filter_map(|ds| {
            match ds {
                DataStructure::Classical { kind } => Some(Accelerator {
                    kind: kind.clone(),
                    config: HashMap::new(),
                }),
                _ => None,
            }
        }));
        Box::new(ClearnedStructure {
            learned: self.learned.clone(),
            classical: self.classical.clone(),
            accelerators: new_accelerators,
        })
    }

    fn evaluate(&self, workload: &WorkloadProfile, infra: &InfraProfile) -> CostPerfMetrics {
        let io_model = super::wokeval::DistributionAwareIOModel::new();
        io_model.evaluate(self, workload, infra)
    }

    fn get_layout(&self) -> Vec<DataStructure> {
        vec![
            DataStructure::Learned {
                kind: self.learned.clone(),
            },
            DataStructure::Classical {
                kind: self.classical.clone(),
            },
        ]
    }
}