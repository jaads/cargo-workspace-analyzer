use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

type PackageName = String;

#[derive(PartialEq, Debug)]
pub struct Metrics {
    pub fan_in: usize,
    pub fan_out: usize,
    pub instability: f32,
}

pub type CouplingMetric = HashMap<PackageName, Metrics>;

impl Graph {
    pub fn calculate_coupling(&self) -> CouplingMetric {
        let mut fan_in_map: HashMap<String, usize> = HashMap::new();
        let mut coupling_map: HashMap<String, Metrics> = HashMap::new();

        for (package, dependencies) in &self.adjacency_list {
            let unique_dependencies: HashSet<_> =
                dependencies.iter().filter(|dep| !dep.is_empty()).collect();
            let fan_out = unique_dependencies.len();

            for dependency in unique_dependencies {
                *fan_in_map.entry(dependency.clone()).or_insert(0) += 1;
            }

            coupling_map.insert(
                package.clone(),
                Metrics {
                    fan_in: 0,
                    fan_out,
                    instability: 0.0,
                },
            );
        }

        for (package, fan_in) in fan_in_map {
            if let Some(entry) = coupling_map.get_mut(&package) {
                entry.fan_in = fan_in;
            }
        }

        for metrics in coupling_map.values_mut() {
            if metrics.fan_in == 0 && metrics.fan_out > 0 {
                metrics.instability = 1.0;
            } else if metrics.fan_in + metrics.fan_out > 0 {
                metrics.instability =
                    metrics.fan_out as f32 / (metrics.fan_in + metrics.fan_out) as f32;
            }
        }

        coupling_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();
        let coupling = graph.calculate_coupling();
        assert!(coupling.is_empty());
    }

    #[test]
    fn test_single_package_no_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert("package_a".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 0,
                instability: 0.0
            })
        );
    }

    #[test]
    fn test_duplicate_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_b".to_string()],
        );
        graph.adjacency_list.insert("package_b".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 1,
                instability: 1.0
            })
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 0,
                instability: 0.0
            })
        );
    }

    #[test]
    fn test_dependency_only_node() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 1,
                instability: 1.0
            })
        );
    }

    #[test]
    fn test_multiple_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_c".to_string()],
        );
        graph
            .adjacency_list
            .insert("package_b".to_string(), vec!["package_c".to_string()]);
        graph.adjacency_list.insert("package_c".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 2,
                instability: 1.0
            })
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 1,
                instability: 0.5
            })
        );
        assert_eq!(
            coupling.get("package_c"),
            Some(&Metrics {
                fan_in: 2,
                fan_out: 0,
                instability: 0.0
            })
        );
    }

    #[test]
    fn test_circular_dependency() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);
        graph
            .adjacency_list
            .insert("package_b".to_string(), vec!["package_a".to_string()]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 1,
                instability: 0.5
            })
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 1,
                instability: 0.5
            })
        );
    }

    #[test]
    fn test_disconnected_components() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);
        graph
            .adjacency_list
            .insert("package_c".to_string(), vec!["package_d".to_string()]);
        graph.adjacency_list.insert("package_b".to_string(), vec![]);
        graph.adjacency_list.insert("package_d".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 1,
                instability: 1.0
            })
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 0,
                instability: 0.0
            })
        );
        assert_eq!(
            coupling.get("package_c"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 1,
                instability: 1.0
            })
        );
        assert_eq!(
            coupling.get("package_d"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 0,
                instability: 0.0
            })
        );
    }

    #[test]
    fn test_instability_metric() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_c".to_string()],
        );
        graph
            .adjacency_list
            .insert("package_b".to_string(), vec!["package_c".to_string()]);
        graph.adjacency_list.insert("package_c".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&Metrics {
                fan_in: 0,
                fan_out: 2,
                instability: 1.0
            })
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&Metrics {
                fan_in: 1,
                fan_out: 1,
                instability: 0.5
            })
        );
        assert_eq!(
            coupling.get("package_c"),
            Some(&Metrics {
                fan_in: 2,
                fan_out: 0,
                instability: 0.0
            })
        );
    }
}
