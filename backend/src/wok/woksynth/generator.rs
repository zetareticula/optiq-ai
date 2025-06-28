use super::wokcore::design::{DesignAtom, WorkloadProfile, InfraProfile, DataStructure, CostPerfMetrics};

pub struct WokSynth;

impl WokSynth {
    pub fn generate(
        &self,
        workload: &WorkloadProfile,
        infra: &InfraProfile,
        design: &dyn DesignAtom,
    ) -> String {
        let layout = design.get_layout();
        let metrics = design.evaluate(workload, infra);
        let mut code = String::from("#[macro_use]\nextern crate wok_engine;\n\n#[derive(WokEngine)]\npub struct MyKVStore {\n");

        for ds in layout {
            match ds {
                DataStructure::Learned { kind } => {
                    code.push_str(&format!("    index: LearnedIndex<{}>,\n", kind));
                }
                DataStructure::Classical { kind } => {
                    code.push_str(&format!("    storage: {}<Tiered>,\n", kind));
                }
                DataStructure::Cleared { learned, classical } => {
                    code.push_str(&format!("    index: ClearedIndex<{}, {}>,\n", learned, classical));
                }
            }
        }

        code.push_str("    buffer: COOPBuffer<10MB>,\n");
        code.push_str("    cache: ARCCache<5MB>,\n");
        code.push_str(&format!(
            "    // Performance: {}ms latency, {} ops/s, ${}/1000 ops\n",
            metrics.latency_ms, metrics.throughput_ops, metrics.cost_per_1000_ops
        ));
        code.push_str("}\n");
        code
    }
}