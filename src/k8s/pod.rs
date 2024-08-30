use std::fmt::format;

use comfy_table::Table;
use serde::{Deserialize};
use crate::k8s::commons::{parse_memory_str_to_mib, parse_cpu_str_to_base, map_to_table_value};

use super::commons::PrintResources;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    pub spec: PodSpec,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    pub containers: Vec<Container>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: String,
    pub resources: Option<ContainerResourceRequirements>
}

#[derive(Debug,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceRequirements {
    pub requests: Option<ContainerResourceDefinition>,
    pub limits: Option<ContainerResourceDefinition>
}

#[derive(Debug,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceDefinition {
    pub cpu: Option<String>,
    pub memory: Option<String>,
}

impl Container {
    pub fn limits_cpu(&self) -> Option<f64> {
        if let Some(cpu) = &self.resources.as_ref()?.limits.as_ref()?.cpu {
            return parse_cpu_str_to_base(&cpu);
        }
        None
    }

    pub fn limits_memory(&self) -> Option<f64> {
        if let Some(memory) = &self.resources.as_ref()?.limits.as_ref()?.memory {
            return parse_memory_str_to_mib(&memory);
        }
        None
    }

    pub fn requests_cpu(&self) -> Option<f64> {
        if let Some(cpu) = &self.resources.as_ref()?.requests.as_ref()?.cpu {
            return parse_cpu_str_to_base(&cpu);
        }
        None
    }

    pub fn requests_memory(&self) -> Option<f64> {
        if let Some(memory) = &self.resources.as_ref()?.requests.as_ref()?.memory {
            return parse_memory_str_to_mib(&memory);
        }
        None
    }
}

impl PrintResources for Container {
    fn print_resources(&self, table:  &mut Table) {
        table
        .add_row(vec![
                format!("  {}", self.name), 
                String::from("Container"), 
                String::new(), 
                map_to_table_value(&self.requests_cpu()), 
                map_to_table_value(&self.limits_cpu()), 
                map_to_table_value(&self.requests_memory()), 
                map_to_table_value(&self.limits_memory()), 
            ]);
    }
}