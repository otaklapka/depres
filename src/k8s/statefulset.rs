use crate::k8s::commons::{map_to_table_value, Metadata, PrintResources};
use crate::k8s::pod::{Container, PodTemplate};
use crate::k8s::pvc::PersistentVolumeClaim;
use comfy_table::Table;
use serde::Deserialize;

use super::commons::{ContainerManager, HPATarget};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSet {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: StatefulSetSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpec {
    pub replicas: u32,
    pub template: PodTemplate,
    pub volume_claim_templates: Option<Vec<PersistentVolumeClaim>>,
}

impl ContainerManager for StatefulSet {
    fn replicas(&self) -> u32 {
        self.spec.replicas
    }
    fn containers(&self) -> &Vec<Container> {
        &self.spec.template.spec.containers
    }
}

impl StatefulSet {
    pub fn requests_storage(&self, multiplier: Option<u32>) -> Option<f64> {
        let requests_storage_sum = self
            .spec
            .volume_claim_templates
            .as_ref()?
            .iter()
            .map(|pvc| pvc.requests_storage())
            .sum::<Option<f64>>()?;
        Some(requests_storage_sum * multiplier.unwrap_or(self.replicas()) as f64)
    }

    pub fn limits_storage(&self, multiplier: Option<u32>) -> Option<f64> {
        let limits_storage_sum = self
            .spec
            .volume_claim_templates
            .as_ref()?
            .iter()
            .map(|pvc| pvc.limits_storage())
            .sum::<Option<f64>>()?;
        Some(limits_storage_sum * multiplier.unwrap_or(self.replicas()) as f64)
    }
}

impl PrintResources for StatefulSet {
    fn print_resources(&self, table: &mut Table) {
        table.add_row(vec![
            format!("{} (x{})", &self.metadata.name, &self.spec.replicas),
            String::from("StatefulSet"),
            self.replicas().to_string(),
            map_to_table_value(&self.requests_cpu(None)),
            map_to_table_value(&self.limits_cpu(None)),
            map_to_table_value(&self.requests_memory(None)),
            map_to_table_value(&self.limits_memory(None)),
            map_to_table_value(&self.requests_storage(None)),
            map_to_table_value(&self.limits_storage(None)),
        ]);

        for container in &self.spec.template.spec.containers {
            container.print_resources(table);
        }

        if let Some(pvc_templates) = &self.spec.volume_claim_templates {
            for pvc in pvc_templates {
                pvc.print_resources(table)
            }
        }
    }
}

impl HPATarget for StatefulSet {
    fn name(&self) -> &String {
        &self.metadata.name
    }
    fn kind(&self) -> &String {
        &self.kind
    }
}
