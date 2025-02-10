use crate::types::nested::ManifestFindings;
use crate::types::root::CargoRootManifestFinding;
use glob::glob;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn filter_findings_by_workspace(
    root_manifest_finding: &CargoRootManifestFinding,
    findings: ManifestFindings,
) -> ManifestFindings {
    let root_manifest = &root_manifest_finding.manifest;
    let root_dir = &root_manifest_finding.path;

    if let Some(workspace) = &root_manifest.workspace {
        // Normalize and collect all member paths (including wildcard matches)
        let members = collect_member_paths(root_dir, workspace.members.as_ref());

        // Normalize and collect exclusions
        let excludes: HashSet<PathBuf> = workspace
            .exclude
            .as_ref()
            .map(|excludes| {
                excludes
                    .iter()
                    .map(|exclude| normalize_path(root_dir, exclude))
                    .collect()
            })
            .unwrap_or_default();

        // Filter findings by membership and exclusion
        findings
            .into_iter()
            .filter(|finding| {
                let normalized_path = finding
                    .path
                    .canonicalize()
                    .unwrap_or_else(|_| finding.path.clone());
                members.contains(&normalized_path) && !excludes.contains(&normalized_path)
            })
            .collect()
    } else {
        // No workspace defined, return all findings
        findings
    }
}

/// Collect all paths from the `members` field, resolving wildcards if present.
fn collect_member_paths(base: &Path, members: Option<&Vec<String>>) -> HashSet<PathBuf> {
    let mut paths = HashSet::new();

    if let Some(members) = members {
        for member in members {
            let full_path = base.join(member);

            if member.contains('*') || member.contains('?') {
                // Handle wildcard pattern
                let pattern = full_path.to_string_lossy().to_string();
                for entry in glob(&pattern).unwrap().flatten() {
                    paths.insert(entry.canonicalize().unwrap_or(entry));
                }
            } else {
                // No wildcard, just normalize and add the path
                paths.insert(full_path.canonicalize().unwrap_or(full_path));
            }
        }
    }

    paths
}

fn normalize_path(base: &Path, member: &str) -> PathBuf {
    base.join(member)
        .canonicalize()
        .unwrap_or_else(|_| base.join(member))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::commons::Package;
    use crate::types::nested::{Manifest, ManifestFinding};
    use crate::types::root::{CargoRootManifest, Workspace};
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use std::fs::File;
    use std::io::Write;

    fn create_manifest(path: &PathBuf, name: &str) {
        let package_content = format!(
            r#"
            [package]
            name = "{}"
            version = "0.1.0"
        "#,
            name
        );

        let mut file = File::create(path.join("Cargo.toml")).unwrap();
        file.write_all(package_content.as_bytes()).unwrap();
    }

    #[test]
    fn test_filter_findings_by_workspace() {
        // Setup temporary workspace
        let temp_workspace = TempDir::new().unwrap();
        let root_dir = temp_workspace.path().to_path_buf();

        // Create some nested manifests
        let member1_dir = temp_workspace.child("member1");
        member1_dir.create_dir_all().unwrap();
        create_manifest(&member1_dir.to_path_buf(), "member1");

        let member2_dir = temp_workspace.child("member2");
        member2_dir.create_dir_all().unwrap();
        create_manifest(&member2_dir.to_path_buf(), "member2");

        let member3_dir = temp_workspace.child("member3");
        member3_dir.create_dir_all().unwrap();
        create_manifest(&member3_dir.to_path_buf(), "member3");

        let member4_dir = temp_workspace.child("member4");
        member4_dir.create_dir_all().unwrap();
        create_manifest(&member4_dir.to_path_buf(), "member4");

        // Define a root manifest finding with workspace configuration
        let root_manifest_finding = CargoRootManifestFinding {
            path: root_dir.clone(),
            manifest: CargoRootManifest {
                workspace: Some(Workspace {
                    members: Some(vec!["member1".to_string(), "member2".to_string()]),
                    exclude: Some(vec!["member4".to_string()]),
                    default_members: None,
                }),
                dependencies: None,
            },
        };

        // Simulate collected manifest findings
        let findings: ManifestFindings = vec![
            ManifestFinding {
                path: member1_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "member1".to_string(),
                    },
                    dependencies: None,
                },
            },
            ManifestFinding {
                path: member2_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "member2".to_string(),
                    },
                    dependencies: None,
                },
            },
            ManifestFinding {
                path: member3_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "member3".to_string(),
                    },
                    dependencies: None,
                },
            },
            ManifestFinding {
                path: member4_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "member4".to_string(),
                    },
                    dependencies: None,
                },
            },
        ];

        // Filter the findings based on the root manifest
        let filtered_findings = filter_findings_by_workspace(&root_manifest_finding, findings);

        // Validate the output
        let filtered_paths: Vec<_> = filtered_findings.iter().map(|f| f.path.clone()).collect();

        assert_eq!(filtered_findings.len(), 2);
        assert!(filtered_paths.contains(&member1_dir.to_path_buf()));
        assert!(filtered_paths.contains(&member2_dir.to_path_buf()));
        assert!(!filtered_paths.contains(&member3_dir.to_path_buf()));
        assert!(!filtered_paths.contains(&member4_dir.to_path_buf()));

        // Clean up
        temp_workspace.close().unwrap();
    }

    #[test]
    fn test_filter_findings_with_wildcards() {
        // Setup temporary workspace
        let temp_workspace = TempDir::new().unwrap();
        let root_dir = temp_workspace.path().to_path_buf();

        // Create nested manifests under a wildcard path
        let crates_dir = temp_workspace.child("crates");
        crates_dir.create_dir_all().unwrap();

        let member1_dir = crates_dir.child("crate1");
        member1_dir.create_dir_all().unwrap();
        create_manifest(&member1_dir.to_path_buf(), "crate1");

        let member2_dir = crates_dir.child("crate2");
        member2_dir.create_dir_all().unwrap();
        create_manifest(&member2_dir.to_path_buf(), "crate2");

        // Define a root manifest finding with wildcard members
        let root_manifest_finding = CargoRootManifestFinding {
            path: root_dir.clone(),
            manifest: CargoRootManifest {
                workspace: Some(Workspace {
                    members: Some(vec!["crates/*".to_string()]), // Use wildcard here
                    exclude: None,
                    default_members: None,
                }),
                dependencies: None,
            },
        };

        // Simulate collected manifest findings
        let findings: ManifestFindings = vec![
            ManifestFinding {
                path: member1_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "crate1".to_string(),
                    },
                    dependencies: None,
                },
            },
            ManifestFinding {
                path: member2_dir.to_path_buf(),
                manifest: Manifest {
                    package: Package {
                        name: "crate2".to_string(),
                    },
                    dependencies: None,
                },
            },
        ];

        // Filter the findings based on the root manifest
        let filtered_findings = filter_findings_by_workspace(&root_manifest_finding, findings);

        // Validate the output
        let filtered_paths: Vec<_> = filtered_findings.iter().map(|f| f.path.clone()).collect();

        assert_eq!(filtered_findings.len(), 2);
        assert!(filtered_paths.contains(&member1_dir.to_path_buf()));
        assert!(filtered_paths.contains(&member2_dir.to_path_buf()));

        // Clean up
        temp_workspace.close().unwrap();
    }
}
