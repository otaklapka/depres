use crate::k8s::{
    cron_job::CronJob, deployment::Deployment, hpa::HorizontalPodAutoscaler,
    pvc::PersistentVolumeClaim, stateful_set::StatefulSet,
};
use comfy_table::Table;
use serde::Deserialize;

use super::{
    pod::Container,
    resources_usage::{ResourceUsage, ToComfyTableValue},
};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum KubeObject {
    StatefulSet(StatefulSet),
    Deployment(Deployment),
    CronJob(CronJob),
    HorizontalPodAutoscaler(HorizontalPodAutoscaler),
    PersistentVolumeClaim(PersistentVolumeClaim),
}

pub trait ObjectManager {
    fn name(&self) -> &String;
    fn kind(&self) -> &String;
    fn replicas(&self) -> u32;
    fn objects(&self) -> Vec<ManagedObject>;

    fn resources_usage(&self, multiplier: Option<u32>) -> ResourceUsage {
        let mut resources = self
            .objects()
            .iter()
            .map(|object| match object {
                // avoid the overhead of dynamic dispatch with enum
                ManagedObject::Container(container) => container.resources_usage(),
                ManagedObject::PersistentVolumeClaim(pvc) => pvc.resources_usage(),
            })
            .sum::<ResourceUsage>();

        resources *= multiplier.unwrap_or(self.replicas());
        resources
    }

    fn print_resources(&self, table: &mut Table) {
        let resources = self.resources_usage(None);

        table.add_row(vec![
            &format!("{}", self.name()),
            self.kind(),
            &self.replicas().to_string(),
            &resources.requests_cpu.to_comfy_table_value(),
            &resources.limits_cpu.to_comfy_table_value(),
            &resources.requests_memory.to_comfy_table_value(),
            &resources.limits_memory.to_comfy_table_value(),
            &resources.requests_storage.to_comfy_table_value(),
            &resources.limits_storage.to_comfy_table_value(),
        ]);

        for object in self.objects() {
            match object {
                ManagedObject::Container(container) => container.print_resources(table),
                ManagedObject::PersistentVolumeClaim(pvc) => pvc.print_resources(table),
            }
        }
    }
}

pub enum ManagedObject<'a> {
    Container(&'a Container),
    PersistentVolumeClaim(&'a PersistentVolumeClaim),
}
