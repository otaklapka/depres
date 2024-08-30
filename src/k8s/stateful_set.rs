use crate::k8s::commons::Metadata;
use crate::k8s::pod::PodTemplate;
use crate::k8s::pvc::PersistentVolumeClaim;
use serde::Deserialize;

use super::commons::HPATarget;
use super::kube_object::{ManagedObject, ObjectManager};

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

impl ObjectManager for StatefulSet {
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
        let objects = self
            .spec
            .template
            .spec
            .containers
            .iter()
            .map(|container| ManagedObject::Container(&container));

        if let Some(pvc_templates) = &self.spec.volume_claim_templates {
            return objects
                .chain(
                    pvc_templates
                        .iter()
                        .map(|pvc| ManagedObject::PersistentVolumeClaim(pvc)),
                )
                .collect();
        }

        objects.collect()
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
