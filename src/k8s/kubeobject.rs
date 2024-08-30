use crate::k8s::{
    cronjob::CronJob, deployment::Deployment, hpa::HorizontalPodAutoscaler,
    pvc::PersistentVolumeClaim, statefulset::StatefulSet,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum KubeObject {
    StatefulSet(StatefulSet),
    Deployment(Deployment),
    CronJob(CronJob),
    HorizontalPodAutoscaler(HorizontalPodAutoscaler),
    PersistentVolumeClaim(PersistentVolumeClaim),
}
