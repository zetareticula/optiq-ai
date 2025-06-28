use super::wok::wokcore::design::{WorkloadProfile, InfraProfile};
use super::wok::woksynth::WokSynth;
use super::fingerprint::{ExplainPlan, QueryFingerprint, generate_fingerprint};
use super::clustering::dbscan::DBSCAN;
use super::clustering::gmm::GMM;

pub fn cluster_plans(plans: Vec<ExplainPlan>, query_ids: Vec<String>) -> Vec<Option<usize>> {
    let fingerprints: Vec<QueryFingerprint> = plans
        .into_iter()
        .zip(query_ids)
        .map(|(plan, id)| generate_fingerprint(&plan, &id))
        .collect();

    let dbscan = DBSCAN::new(0.5, 2);
    dbscan.cluster(&fingerprints)
}

pub fn cluster_plans_gmm(plans: Vec<ExplainPlan>, query_ids: Vec<String>) -> Vec<usize> {
    let fingerprints: Vec<QueryFingerprint> = plans
        .into_iter()
        .zip(query_ids)
        .map(|(plan, id)| generate_fingerprint(&plan, &id))
        .collect();

    let gmm = GMM::new(3, 100, 1e-6);
    gmm.fit(&fingerprints)
}

pub fn cache_plans(workload: &WorkloadProfile, infra: &InfraProfile) -> String {
    let mut search = super::wok::wokcore::DesignSearch::new();
    search.add_design(Box::new(super::wok::wokcore::LSMTree {
        compaction: "Tiered".to_string(),
        accelerators: vec![],
    }));
    let (design, _) = search.search(workload, infra, 50).expect("Design search failed");
    let synth = WokSynth;
    synth.generate(workload, infra, &*design)
}