use crate::k8s::commons::{parse_memory_str_to_mib, Metadata};
use comfy_table::Table;
use serde::Deserialize;

use super::resources_usage::{ResourceUsage, ToComfyTableValue};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaim {
    pub metadata: Metadata,
    pub spec: PersistentVolumeClaimSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimSpec {
    pub resources: PvcResourceRequirements,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PvcResourceRequirements {
    pub requests: PvcResourceDefinition,
    pub limits: Option<PvcResourceDefinition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PvcResourceDefinition {
    pub storage: String,
}

impl PersistentVolumeClaim {
    pub fn resources_usage(&self) -> ResourceUsage {
        ResourceUsage {
            requests_cpu: None,
            limits_cpu: None,

            requests_memory: None,
            limits_memory: None,

            requests_storage: self.requests_storage(),
            limits_storage: self.limits_storage(),
        }
    }

    pub fn requests_storage(&self) -> Option<f64> {
        parse_memory_str_to_mib(&self.spec.resources.requests.storage)
    }
    pub fn limits_storage(&self) -> Option<f64> {
        self.spec
            .resources
            .limits
            .as_ref()
            .and_then(|limits_storage| parse_memory_str_to_mib(&limits_storage.storage))
    }
    pub fn print_resources(&self, table: &mut Table) {
        let resources = self.resources_usage();

        table.add_row(vec![
            format!("  {}", self.metadata.name),
            String::from("PVC"),
            String::new(),
            resources.requests_cpu.to_comfy_table_value(),
            resources.limits_cpu.to_comfy_table_value(),
            resources.requests_memory.to_comfy_table_value(),
            resources.limits_memory.to_comfy_table_value(),
            resources.requests_storage.to_comfy_table_value(),
            resources.limits_storage.to_comfy_table_value(),
        ]);
    }
}
