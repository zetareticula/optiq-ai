use super::fingerprint::QueryFingerprint;
use rand::Rng;

pub struct GMM {
    n_components: usize,
    max_iter: usize,
    tol: f32,
}

impl GMM {
    pub fn new(n_components: usize, max_iter: usize, tol: f32) -> Self {
        GMM {
            n_components,
            max_iter,
            tol,
        }
    }

    pub fn fit(&self, fingerprints: &[QueryFingerprint]) -> Vec<usize> {
        let n_samples = fingerprints.len();
        let n_features = fingerprints[0].vector.len();
        let mut means: Vec<Vec<f32>> = vec![vec![0.0; n_features]; self.n_components];
        let mut weights = vec![1.0 / self.n_components as f32; self.n_components];
        let mut covariances = vec![vec![1.0; n_features]; self.n_components];

        // Initialize means randomly
        let mut rng = rand::thread_rng();
        for mean in means.iter_mut() {
            for i in 0..n_features {
                mean[i] = rng.gen_range(0.0..1.0);
            }
        }

        // Simplified EM algorithm
        let mut responsibilities = vec![vec![0.0; self.n_components]; n_samples];
        for _ in 0..self.max_iter {
            // E-step: Compute responsibilities
            for i in 0..n_samples {
                let point = &fingerprints[i].vector;
                let mut total_prob = 0.0;
                for k in 0..self.n_components {
                    responsibilities[i][k] = weights[k] * self.gaussian(point, &means[k], &covariances[k]);
                    total_prob += responsibilities[i][k];
                }
                for k in 0..self.n_components {
                    responsibilities[i][k] /= total_prob;
                }
            }

            // M-step: Update parameters
            let mut new_means = vec![vec![0.0; n_features]; self.n_components];
            let mut new_weights = vec![0.0; self.n_components];
            let mut new_covariances = vec![vec![0.0; n_features]; self.n_components];

            for k in 0..self.n_components {
                let mut total_resp = 0.0;
                for i in 0..n_samples {
                    total_resp += responsibilities[i][k];
                    for j in 0..n_features {
                        new_means[k][j] += responsibilities[i][k] * fingerprints[i].vector[j];
                    }
                }
                new_means[k].iter_mut().for_each(|m| *m /= total_resp);
                new_weights[k] = total_resp / n_samples as f32;

                for i in 0..n_samples {
                    for j in 0..n_features {
                        let diff = fingerprints[i].vector[j] - new_means[k][j];
                        new_covariances[k][j] += responsibilities[i][k] * diff * diff;
                    }
                }
                new_covariances[k].iter_mut().for_each(|c| *c /= total_resp);
            }

            means = new_means;
            weights = new_weights;
            covariances = new_covariances;
        }

        // Assign clusters
        let mut labels = vec![0; n_samples];
        for i in 0..n_samples {
            let mut max_resp = 0.0;
            for k in 0..self.n_components {
                if responsibilities[i][k] > max_resp {
                    max_resp = responsibilities[i][k];
                    labels[i] = k;
                }
            }
        }
        labels
    }

    fn gaussian(&self, x: &[f32], mean: &[f32], cov: &[f32]) -> f32 {
        let n = x.len() as f32;
        let det = cov.iter().product::<f32>();
        let exp_term = x.iter()
            .zip(mean.iter())
            .zip(cov.iter())
            .map(|((x, m), c)| -0.5 * (x - m).powi(2) / c)
            .sum::<f32>();
        (1.0 / (2.0 * std::f32::consts::PI).powf(n / 2.0) / det.sqrt()) * exp_term.exp()
    }
}