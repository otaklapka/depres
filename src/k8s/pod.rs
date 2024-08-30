use crate::k8s::commons::{parse_cpu_str_to_base, parse_memory_str_to_mib};
use comfy_table::Table;
use serde::Deserialize;

use super::resources_usage::{ResourceUsage, ToComfyTableValue};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    pub spec: PodSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    pub containers: Vec<Container>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: String,
    pub resources: Option<ContainerResourceRequirements>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceRequirements {
    pub requests: Option<ContainerResourceDefinition>,
    pub limits: Option<ContainerResourceDefinition>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceDefinition {
    pub cpu: Option<String>,
    pub memory: Option<String>,
}

impl Container {
    pub fn resources_usage(&self) -> ResourceUsage {
        ResourceUsage {
            requests_cpu: self.requests_cpu(),
            limits_cpu: self.limits_cpu(),

            requests_memory: self.requests_memory(),
            limits_memory: self.limits_memory(),

            requests_storage: None,
            limits_storage: None,
        }
    }

    pub fn limits_cpu(&self) -> Option<f64> {
        self.resources
            .as_ref()?
            .limits
            .as_ref()?
            .cpu
            .as_ref()
            .and_then(|cpu| parse_cpu_str_to_base(&cpu))
    }

    pub fn limits_memory(&self) -> Option<f64> {
        self.resources
            .as_ref()?
            .limits
            .as_ref()?
            .memory
            .as_ref()
            .and_then(|memory| parse_memory_str_to_mib(&memory))
    }

    pub fn requests_cpu(&self) -> Option<f64> {
        self.resources
            .as_ref()?
            .requests
            .as_ref()?
            .cpu
            .as_ref()
            .and_then(|cpu| parse_cpu_str_to_base(&cpu))
    }

    pub fn requests_memory(&self) -> Option<f64> {
        self.resources
            .as_ref()?
            .requests
            .as_ref()?
            .memory
            .as_ref()
            .and_then(|memory| parse_memory_str_to_mib(&memory))
    }

    pub fn print_resources(&self, table: &mut Table) {
        let resources = self.resources_usage();

        table.add_row(vec![
            format!("  {}", self.name),
            String::from("Container"),
            String::new(),
            resources.requests_cpu.to_comfy_table_value(),
            resources.limits_cpu.to_comfy_table_value(),
            resources.requests_memory.to_comfy_table_value(),
            resources.limits_memory.to_comfy_table_value(),
        ]);
    }
}
