use crate::k8s::commons::{map_to_table_value, ContainerManager, Metadata, PrintResources};
use crate::k8s::pod::{Container, PodTemplate};
use comfy_table::Table;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    pub kind: Option<String>,
    pub metadata: Metadata,
    pub spec: CronJobSpec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    pub schedule: String,
    pub job_template: JobTemplate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplate {
    pub spec: JobSpec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    pub template: PodTemplate,
}

impl ContainerManager for CronJob {
    fn replicas(&self) -> u32 {
        1
    }
    fn containers(&self) -> &Vec<Container> {
        &self.spec.job_template.spec.template.spec.containers
    }
}

impl PrintResources for CronJob {
    fn print_resources(&self, table: &mut Table) {
        for container in self.containers() {
            container.print_resources(table);
        }

        table.add_row(vec![
            &self.metadata.name,
            &String::from("CronJob"),
            &String::from("1"),
            &map_to_table_value(&self.requests_cpu(None)),
            &map_to_table_value(&self.limits_cpu(None)),
            &map_to_table_value(&self.requests_memory(None)),
            &map_to_table_value(&self.limits_memory(None)),
        ]);
    }
}
