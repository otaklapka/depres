use crate::k8s::commons::{map_to_table_value, parse_memory_str_to_mib, Metadata};
use comfy_table::Table;
use serde::Deserialize;

use super::commons::PrintResources;

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
    pub fn requests_storage(&self) -> Option<f64> {
        parse_memory_str_to_mib(&self.spec.resources.requests.storage)
    }
    pub fn limits_storage(&self) -> Option<f64> {
        let limits_storage = self.spec.resources.limits.as_ref()?.storage.as_ref();
        parse_memory_str_to_mib(limits_storage)
    }
}

impl PrintResources for PersistentVolumeClaim {
    fn print_resources(&self, table: &mut Table) {
        table.add_row(vec![
            format!("  {}", self.metadata.name),
            String::from("PVC"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            map_to_table_value(&self.requests_storage()),
            map_to_table_value(&self.limits_storage()),
        ]);
    }
}
