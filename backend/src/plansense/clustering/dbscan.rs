use super::fingerprint::QueryFingerprint;

pub struct DBSCAN {
    eps: f32, // Neighborhood radius
    min_pts: usize, // Minimum points to form a cluster
}

impl DBSCAN {
    pub fn new(eps: f32, min_pts: usize) -> Self {
        DBSCAN { eps, min_pts }
    }

    pub fn cluster(&self, fingerprints: &[QueryFingerprint]) -> Vec<Option<usize>> {
        let mut labels = vec![None; fingerprints.len()];
        let mut cluster_id = 0;

        for i in 0..fingerprints.len() {
            if labels[i].is_some() {
                continue;
            }

            let neighbors = self.get_neighbors(i, fingerprints);
            if neighbors.len() >= self.min_pts {
                labels[i] = Some(cluster_id);
                self.expand_cluster(i, &neighbors, &mut labels, cluster_id, fingerprints);
                cluster_id += 1;
            }
        }

        labels
    }

    fn get_neighbors(&self, index: usize, fingerprints: &[QueryFingerprint]) -> Vec<usize> {
        let mut neighbors = vec![];
        let point = &fingerprints[index].vector;

        for (i, fp) in fingerprints.iter().enumerate() {
            if i != index && self.euclidean_distance(point, &fp.vector) <= self.eps {
                neighbors.push(i);
            }
        }
        neighbors
    }

    fn expand_cluster(
        &self,
        index: usize,
        neighbors: &[usize],
        labels: &mut [Option<usize>],
        cluster_id: usize,
        fingerprints: &[QueryFingerprint],
    ) {
        let mut stack = neighbors.to_vec();
        while let Some(neighbor_idx) = stack.pop() {
            if labels[neighbor_idx].is_some() {
                continue;
            }

            labels[neighbor_idx] = Some(cluster_id);
            let new_neighbors = self.get_neighbors(neighbor_idx, fingerprints);
            if new_neighbors.len() >= self.min_pts {
                stack.extend(new_neighbors);
            }
        }
    }

    fn euclidean_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}