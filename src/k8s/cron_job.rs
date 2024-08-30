use crate::k8s::commons::Metadata;
use crate::k8s::kube_object::ManagedObject;
use crate::k8s::pod::PodTemplate;
use serde::Deserialize;

use super::kube_object::ObjectManager;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: CronJobSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    pub schedule: String,
    pub job_template: JobTemplate,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplate {
    pub spec: JobSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    pub template: PodTemplate,
}

impl ObjectManager for CronJob {
    fn replicas(&self) -> u32 {
        1
    }
    fn objects(&self) -> Vec<ManagedObject> {
        self.spec
            .job_template
            .spec
            .template
            .spec
            .containers
            .iter()
            .map(|container| ManagedObject::Container(container))
            .collect()
    }
    fn kind(&self) -> &String {
        &self.kind
    }
    fn name(&self) -> &String {
        &self.metadata.name
    }
}
