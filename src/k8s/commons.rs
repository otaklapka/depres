use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
}

pub trait HPATarget {
    fn name(&self) -> &String;
    fn kind(&self) -> &String;
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
                        _ => None,
                    };
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
                        _ => Some(int_val),
                    };
                }
            }
        }
    }
    None
}
