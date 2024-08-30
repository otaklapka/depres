use comfy_table::Table;
use serde::{Deserialize};
use crate::k8s::pod::{PodTemplate, Container};
use crate::k8s::commons::{Metadata};
use super::commons::{ContainerManager, PrintResources, HPATarget, map_to_table_value};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: DeploymentSpec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    pub replicas: u32,
    pub template: PodTemplate,
}

impl ContainerManager for Deployment {
    fn replicas(&self) -> u32 {
        self.spec.replicas
    }
    fn containers(&self) -> &Vec<Container> {
        &self.spec.template.spec.containers
    }
}

impl PrintResources for Deployment {
    fn print_resources(&self, table: &mut Table) {
        table
        .add_row(vec![
            format!("{} (x{})", self.metadata.name, self.spec.replicas), 
            String::from("Deployment"), 
            self.replicas().to_string(), 
            map_to_table_value(&self.requests_cpu(None)), 
            map_to_table_value(&self.limits_cpu(None)), 
            map_to_table_value(&self.requests_memory(None)), 
            map_to_table_value(&self.limits_memory(None)), 
            ]);
        for container in &self.spec.template.spec.containers {
            container.print_resources(table);
        }
    }
}

impl HPATarget for Deployment {
    fn name(&self) -> &String {
        &self.metadata.name
    }
    fn kind(&self) -> &String {
        &self.kind
    }
}
    