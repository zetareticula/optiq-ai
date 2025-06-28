use crate::wok::wokcore::design::{WorkloadProfile, InfraProfile, LSMTree, DesignAtom};
use crate::wok::wokcore::search::DesignSearch;
use crate::plansense::fingerprint::{ExplainPlan, QueryFingerprint, generate_fingerprint};
use crate::plansense::clustering::dbscan::DBSCAN;
use crate::plansense::clustering::gmm::GMM;
use crate::plansense::caching::wok_cache::PlanSenseCache;
use std::collections::HashMap;

pub struct PlanSense {
    cache: PlanSenseCache,
}

impl PlanSense {
    pub fn new(workload: &WorkloadProfile, infra: &InfraProfile) -> Self {
        let mut search = DesignSearch::new();
        search.add_design(Box::new(LSMTree {
            compaction: "Tiered".to_string(),
            accelerators: vec![super::wok::wokcore::design::Accelerator {
                kind: "Cache".to_string(),
                config: HashMap::from([("type".to_string(), "ARC".to_string()), ("size".to_string(), "5MB".to_string())]),
            }],
        }));
        let (design, _) = search.search(workload, infra, 50).expect("Design search failed");
        PlanSense {
            cache: PlanSenseCache::new(workload, infra, design),
        }
    }

    pub fn cluster_and_cache(
        &self,
        plans: Vec<ExplainPlan>,
        query_ids: Vec<String>,
        use_gmm: bool,
    ) -> Result<Vec<Option<usize>>, String> {
        let fingerprints: Vec<QueryFingerprint> = plans
            .iter()
            .zip(query_ids.iter())
            .map(|(plan, id)| generate_fingerprint(plan, id))
            .collect();

        // Cluster plans
        let labels = if use_gmm {
            let gmm = GMM::new(3, 100, 1e-6);
            gmm.fit(&fingerprints).into_iter().map(Some).collect()
        } else {
            let dbscan = DBSCAN::new(0.5, 2);
            dbscan.cluster(&fingerprints)
        };

        // Cache plans
        for (fingerprint, plan) in fingerprints.iter().zip(plans.iter()) {
            self.cache.cache_plan(fingerprint, plan)?;
        }

        Ok(labels)
    }

    pub fn get_cached_plan(&self, plan_hash: &str) -> Result<Option<ExplainPlan>, String> {
        self.cache.get_plan(plan_hash)
    }
}