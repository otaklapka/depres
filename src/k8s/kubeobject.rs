use serde::{Deserialize};
use crate::k8s::{hpa::HorizontalPodAutoscaler, pvc::PersistentVolumeClaim, cronjob::CronJob, statefulset::StatefulSet, deployment::Deployment};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KubeObject {
    StatefulSet(StatefulSet),
    Deployment(Deployment),
    CronJob(CronJob),
    HorizontalPodAutoscaler(HorizontalPodAutoscaler),
    PersistentVolumeClaim(PersistentVolumeClaim),
}