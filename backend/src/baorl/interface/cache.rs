use redis::Commands;
use crate::plansense::fingerprint::ExplainPlan;
use crate::wok::wokcore::design::{WorkloadProfile, InfraProfile};
use crate::wok::woksynth::WokSynth;

pub struct BaoRLInterface {
    redis_client: redis::Client,
    wok_kv: crate::plansense::caching::wok_cache::WokKVStore,
}

impl BaoRLInterface {
    pub fn new(redis_url: &str, workload: &WorkloadProfile, infra: &InfraProfile, design: Box<dyn crate::wok::wokcore::design::DesignAtom>) -> Self {
        let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
        let wok_kv = crate::plansense::caching::wok_cache::WokKVStore::new(workload, infra, &*design);
        BaoRLInterface { redis_client, wok_kv }
    }

    pub fn cache_plan(&self, plan_hash: &str, plan: &ExplainPlan) -> Result<(), String> {
        let mut conn = self.redis_client.get_connection().map_err(|e| e.to_string())?;
        let plan_data = serde_json::to_string(plan).map_err(|e| e.to_string())?;
        conn.set(plan_hash, plan_data.clone()).map_err(|e| e.to_string())?;
        self.wok_kv.put(plan_hash, plan_data.as_bytes())
    }

    pub fn get_plan(&self, plan_hash: &str) -> Result<Option<ExplainPlan>, String> {
        // Try Redis first
        let mut conn = self.redis_client.get_connection().map_err(|e| e.to_string())?;
        if let Ok(plan_data) = conn.get::<_, String>(plan_hash) {
            return Ok(Some(serde_json::from_str(&plan_data).map_err(|e| e.to_string())?));
        }
        // Fall back to Wok KV store
        self.wok_kv.get(plan_hash).map(|opt| {
            opt.map(|data| serde_json::from_slice(&data).expect("Failed to deserialize plan"))
        })
    }
}