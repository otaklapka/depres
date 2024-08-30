use crate::k8s::commons::Metadata;
use comfy_table::Table;
use serde::Deserialize;

use super::commons::HPATarget;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleTargetRef {
    pub kind: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HPASpec {
    pub max_replicas: u32,
    pub min_replicas: u32,
    pub scale_target_ref: ScaleTargetRef,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscaler {
    pub metadata: Metadata,
    pub spec: HPASpec,
}

impl HorizontalPodAutoscaler {
    pub fn try_match(&self, target: &impl HPATarget) -> bool {
        &self.spec.scale_target_ref.kind == target.kind()
            && &self.spec.scale_target_ref.name == target.name()
    }

    pub fn print_resources(&self, table: &mut Table) {
        table.add_row(vec![
            &self.metadata.name,
            &String::from("HPA"),
            &format!("{} -> {}", self.spec.min_replicas, self.spec.max_replicas),
        ]);
    }
}
