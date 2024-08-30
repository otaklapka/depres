use comfy_table::Table;
use serde::{ser::SerializeSeq, Deserialize, Serialize};
use regex::Regex;
use crate::k8s::pod::{Container};
use crate::k8s::hpa::{ScaleTargetRef};

use super::pvc::{self, PersistentVolumeClaim};

#[derive(Debug,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
}

pub fn parse_memory_str_to_mib(res: &str) -> Option<f64> {
    let re = Regex::new(r"(\d+)(Gi|G|Mi|M)").unwrap();
    if let Some(caps) = re.captures(res) {
        if let Some(res_val) = caps.get(1) {
                    if let Ok(val) = res_val.as_str().parse::<f64>() {
                        if let Some(res_unit) = caps.get(2) {
                            return match res_unit.as_str() {
                                "G" => Some(val * 953.674316),
                                "Gi" => Some(val * 1024 as f64),
                                "M" => Some(val * 0.953674316),
                                "Mi" => Some(val),
                                _ => None
                            }
                        }
                    } 
                } 
    }
    None
}

pub fn parse_cpu_str_to_base(res: &str) -> Option<f64> {
    let re = Regex::new(r"(\d+)(m)?").unwrap();
    if let Some(caps) = re.captures(res) {
        if let Some(res_val) = caps.get(1) {
                    if let Ok(int_val) = res_val.as_str().parse::<f64>() {
                        if let Some(res_unit) = caps.get(2) {
                            return match res_unit.as_str() {
                                "m" => Some(int_val / 1000 as f64),
                                _ => Some(int_val)
                            }
                        }
                    } 
                } 
    }
    None
}

pub trait ContainerManager {
    fn replicas(&self) -> u32;
    fn containers(&self) -> &Vec<Container>;

    fn limits_cpu(&self, multiplier: Option<u32>) -> Option<f64> {
        let mut cpu_sum: f64 = 0.0;
        for container in self.containers() {
            if let Some(limits_cpu) = container.limits_cpu() {
                cpu_sum += limits_cpu;
            }
        }

        if cpu_sum > 0.0 {
            return Some(cpu_sum * multiplier.unwrap_or(self.replicas()) as f64)
        }
        None
    }

    fn limits_memory(&self, multiplier: Option<u32>) -> Option<f64> {
        let mut memory_sum: f64 = 0.0;
        for container in self.containers() {
            if let Some(limits_memory) = container.limits_memory() {
                memory_sum += limits_memory;
            }
        }

        if memory_sum > 0.0 {
            return Some(memory_sum * multiplier.unwrap_or(self.replicas()) as f64)
        }
        None
    }

    fn requests_cpu(&self, multiplier: Option<u32>) -> Option<f64> {
        let mut cpu_sum: f64 = 0.0;
        for container in self.containers() {
            if let Some(requests_cpu) = container.requests_cpu() {
                cpu_sum += requests_cpu;
            }
        }

        if cpu_sum > 0.0 {
            return Some(cpu_sum * multiplier.unwrap_or(self.replicas()) as f64)
        }
        None
    }

    fn requests_memory(&self, multiplier: Option<u32>) -> Option<f64> {
        let mut memory_sum: f64 = 0.0;
        for container in self.containers() {
            if let Some(requests_memory) = container.requests_memory() {
                memory_sum += requests_memory;
            }
        }

        if memory_sum > 0.0 {
            return Some(memory_sum * multiplier.unwrap_or(self.replicas()) as f64)
        }
        None
    }
}

pub trait PrintResources {
    fn print_resources(&self, table: &mut Table);
}

pub trait HPATarget {
    fn name(&self) -> &String;
    fn kind(&self) -> &String;
}

pub fn map_to_table_value(val: &Option<impl ToString>) -> String {
    if let Some(val) = val {
        return val.to_string();
    }
    String::new()
}