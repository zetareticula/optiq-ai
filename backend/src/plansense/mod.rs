use crate::wok::wokcore::design::{WorkloadProfile, InfraProfile};
use crate::wok::woksynth::WokSynth;

pub fn cache_plans(workload: &WorkloadProfile, infra: &InfraProfile) -> String {
    let mut search = crate::wok::wokcore::DesignSearch::new();
    search.add_design(Box::new(crate::wok::wokcore::LSMTree {
        compaction: "Tiered".to_string(),
        accelerators: vec![],
    }));
    let (design, _) = search.search(workload, infra, 50).expect("Design search failed");
    let synth = WokSynth;
    synth.generate(workload, infra, &*design) // Use Wok-synthesized KV store for caching
}