use axum::{extract::Json, response::Json};
use serde::{Deserialize, Serialize};
use crate::wok::wokcore::{DesignSearch, WorkloadProfile, InfraProfile, LSMTree, LearnedIndex, ClearedStructure};
use crate::wok::woksynth::WokSynth;

#[derive(Deserialize)]
pub struct WokBuildRequest {
    workload: WorkloadProfile,
    infra: InfraProfile,
}

#[derive(Serialize)]
pub struct WokBuildResponse {
    code: String,
    metrics: CostPerfMetrics,
}

pub async fn build_wok(Json(payload): Json<WokBuildRequest>) -> Json<WokBuildResponse> {
    let mut search = DesignSearch::new();
    search.add_design(Box::new(LearnedIndex::new("PGM", 5_000_000)));
    search.add_design(Box::new(LSMTree {
        compaction: "Tiered".to_string(),
        accelerators: vec![Accelerator {
            kind: "Cache".to_string(),
            config: HashMap::from([("type".to_string(), "ARC".to_string())]),
        }],
    }));
    search.add_design(Box::new(ClearnedStructure {
        learned: "PGM".to_string(),
        classical: "LSMTree".to_string(),
        accelerators: vec![Accelerator {
            kind: "Buffer".to_string(),
            config: HashMap::from([("size".to_string(), "10MB".to_string())]),
        }],
    }));

    let (design, metrics) = search
        .search(&payload.workload, &payload.infra, 100)
        .expect("Design search failed");

    let synth = WokSynth;
    let code = synth.generate(&payload.workload, &payload.infra, &*design);
    Json(WokBuildResponse { code, metrics })
}