use std::iter::Sum;

#[derive(Debug, PartialEq)]
pub struct ResourceUsage {
    pub requests_cpu: Option<f64>,
    pub limits_cpu: Option<f64>,

    pub requests_memory: Option<f64>,
    pub limits_memory: Option<f64>,

    pub requests_storage: Option<f64>,
    pub limits_storage: Option<f64>,
}

impl ResourceUsage {
    pub fn new() -> ResourceUsage {
        ResourceUsage {
            requests_cpu: None,
            limits_cpu: None,

            requests_memory: None,
            limits_memory: None,

            requests_storage: None,
            limits_storage: None,
        }
    }
}

impl std::ops::AddAssign for ResourceUsage {
    fn add_assign(&mut self, other: Self) {
        self.requests_cpu = add_options(self.requests_cpu, other.requests_cpu);
        self.limits_cpu = add_options(self.limits_cpu, other.limits_cpu);

        self.requests_memory = add_options(self.requests_memory, other.requests_memory);
        self.limits_memory = add_options(self.limits_memory, other.limits_memory);

        self.requests_storage = add_options(self.requests_storage, other.requests_storage);
        self.limits_storage = add_options(self.limits_storage, other.limits_storage);
    }
}

impl std::ops::MulAssign<u32> for ResourceUsage {
    fn mul_assign(&mut self, rhs: u32) {
        if let Some(requests_cpu) = &mut self.requests_cpu {
            *requests_cpu *= rhs as f64
        }
        if let Some(limits_cpu) = &mut self.limits_cpu {
            *limits_cpu *= rhs as f64
        }

        if let Some(requests_memory) = &mut self.requests_memory {
            *requests_memory *= rhs as f64
        }
        if let Some(limits_memory) = &mut self.limits_memory {
            *limits_memory *= rhs as f64
        }

        if let Some(requests_storage) = &mut self.requests_storage {
            *requests_storage *= rhs as f64
        }
        if let Some(limits_storage) = &mut self.limits_storage {
            *limits_storage *= rhs as f64
        }
    }
}

impl Sum for ResourceUsage {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ResourceUsage::new(), |mut acc, usage| {
            acc += usage;
            acc
        })
    }
}

fn add_options(opt1: Option<f64>, opt2: Option<f64>) -> Option<f64> {
    match (opt1, opt2) {
        (Some(val1), Some(val2)) => Some(val1 + val2),
        (Some(val), None) | (None, Some(val)) => Some(val),
        _ => None,
    }
}

pub trait ToComfyTableValue {
    fn to_comfy_table_value(&self) -> String;
}

impl<T> ToComfyTableValue for Option<T>
where
    T: ToString,
{
    fn to_comfy_table_value(&self) -> String {
        if let Self::Some(val) = self {
            return val.to_string();
        }
        String::new()
    }
}
