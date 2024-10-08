use super::commons::HPATarget;
use super::kube_object::ObjectManager;
use crate::k8s::commons::Metadata;
use crate::k8s::kube_object::ManagedObject;
use crate::k8s::pod::PodTemplate;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: DeploymentSpec,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    pub replicas: u32,
    pub template: PodTemplate,
}

impl ObjectManager for Deployment {
    fn replicas(&self) -> u32 {
        self.spec.replicas
    }
    fn kind(&self) -> &String {
        &self.kind
    }
    fn name(&self) -> &String {
        &self.metadata.name
    }
    fn objects(&self) -> Vec<ManagedObject> {
        self.spec
            .template
            .spec
            .containers
            .iter()
            .map(|container| ManagedObject::Container(container))
            .collect()
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
