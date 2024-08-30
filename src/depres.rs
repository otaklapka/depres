use crate::k8s::{
    commons::ContainerManager, commons::PrintResources, hpa::HorizontalPodAutoscaler,
    kubeobject::KubeObject,
};
use comfy_table::{Attribute, Cell, Table};
use serde::Deserialize;

pub fn depres(file_contents: Vec<String>) -> Result<(), serde_yaml::Error> {
    let mut kube_objects: Vec<KubeObject> = vec![];
    let mut hpas: Vec<HorizontalPodAutoscaler> = vec![];

    for file_content in file_contents {
        for document in serde_yaml::Deserializer::from_str(&file_content) {
            if let Ok(kube_object) = KubeObject::deserialize(document) {
                match kube_object {
                    KubeObject::HorizontalPodAutoscaler(hpa) => hpas.push(hpa),
                    _ => kube_objects.push(kube_object),
                }
            }
        }
    }

    let mut total_request_memory: f64 = 0.0;
    let mut total_request_cpu: f64 = 0.0;
    let mut total_limit_memory: f64 = 0.0;
    let mut total_limit_cpu: f64 = 0.0;
    let mut total_request_storage: f64 = 0.0;
    let mut total_limit_storage: f64 = 0.0;

    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Name").add_attribute(Attribute::Bold),
            Cell::new("Kind").add_attribute(Attribute::Bold),
            Cell::new("Replicas").add_attribute(Attribute::Bold),
            Cell::new("cpu.requests").add_attribute(Attribute::Bold),
            Cell::new("cpu.limits").add_attribute(Attribute::Bold),
            Cell::new("memory.requests").add_attribute(Attribute::Bold),
            Cell::new("memory.limits").add_attribute(Attribute::Bold),
            Cell::new("storage.requests").add_attribute(Attribute::Bold),
            Cell::new("storage.limits").add_attribute(Attribute::Bold),
        ])
        .load_preset(comfy_table::presets::NOTHING);

    for kube_object in kube_objects.iter() {
        match kube_object {
            KubeObject::StatefulSet(statefulset) => {
                let hpa = hpas.iter().find(|hpa| hpa.try_match(statefulset));
                let hpa_max_replicas = hpa.and_then(|hpa| Some(hpa.spec.max_replicas));

                if let Some(hpa) = hpa {
                    hpa.print_resources(&mut table)
                }
                statefulset.print_resources(&mut table);

                total_limit_cpu += statefulset.limits_cpu(hpa_max_replicas).unwrap_or(0.0);
                total_limit_memory += statefulset.limits_memory(hpa_max_replicas).unwrap_or(0.0);
                total_request_cpu += statefulset.requests_cpu(hpa_max_replicas).unwrap_or(0.0);
                total_request_memory +=
                    statefulset.requests_memory(hpa_max_replicas).unwrap_or(0.0);
                total_limit_storage += statefulset.limits_storage(hpa_max_replicas).unwrap_or(0.0);
                total_request_storage += statefulset
                    .requests_storage(hpa_max_replicas)
                    .unwrap_or(0.0);
            }
            KubeObject::Deployment(deployment) => {
                let hpa = hpas.iter().find(|hpa| hpa.try_match(deployment));
                let hpa_max_replicas = hpa.and_then(|hpa| Some(hpa.spec.max_replicas));

                if let Some(hpa) = hpa {
                    hpa.print_resources(&mut table);
                }
                deployment.print_resources(&mut table);

                total_limit_cpu += deployment.limits_cpu(hpa_max_replicas).unwrap_or(0.0);
                total_limit_memory += deployment.limits_memory(hpa_max_replicas).unwrap_or(0.0);
                total_request_cpu += deployment.requests_cpu(hpa_max_replicas).unwrap_or(0.0);
                total_request_memory += deployment.requests_memory(hpa_max_replicas).unwrap_or(0.0);
            }
            KubeObject::CronJob(cronjob) => {
                cronjob.print_resources(&mut table);

                total_limit_cpu += cronjob.limits_cpu(None).unwrap_or(0.0);
                total_limit_memory += cronjob.limits_memory(None).unwrap_or(0.0);
                total_request_cpu += cronjob.requests_cpu(None).unwrap_or(0.0);
                total_request_memory += cronjob.requests_memory(None).unwrap_or(0.0);
            }
            KubeObject::PersistentVolumeClaim(pvc) => {
                pvc.print_resources(&mut table);

                total_limit_storage += pvc.limits_storage().unwrap_or(0.0);
                total_request_storage += pvc.requests_storage().unwrap_or(0.0);
            }
            _ => {}
        }
    }

    table.add_row(vec![
        "Total",
        "",
        "",
        &total_request_cpu.to_string(),
        &total_limit_cpu.to_string(),
        &total_request_memory.to_string(),
        &total_limit_memory.to_string(),
        &total_request_storage.to_string(),
        &total_limit_storage.to_string(),
    ]);

    println!("{table}");

    Ok(())
}
