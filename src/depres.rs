use crate::k8s::kube_object::{KubeObject, ObjectManager};
use crate::k8s::{
    hpa::HorizontalPodAutoscaler,
    resources_usage::{ResourceUsage, ToComfyTableValue},
};
use comfy_table::{Attribute, Cell, Table};
use serde::Deserialize;

pub fn read_deployment_resources(
    file_contents: Vec<String>,
) -> Result<ResourceUsage, serde_yaml::Error> {
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

    let mut resources = ResourceUsage::new();

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

                resources += statefulset.resources_usage(hpa_max_replicas);

                if let Some(hpa) = hpa {
                    hpa.print_resources(&mut table)
                }
                statefulset.print_resources(&mut table);
            }
            KubeObject::Deployment(deployment) => {
                let hpa = hpas.iter().find(|hpa| hpa.try_match(deployment));
                let hpa_max_replicas = hpa.and_then(|hpa| Some(hpa.spec.max_replicas));

                resources += deployment.resources_usage(hpa_max_replicas);

                if let Some(hpa) = hpa {
                    hpa.print_resources(&mut table);
                }
                deployment.print_resources(&mut table);
            }
            KubeObject::CronJob(cronjob) => {
                resources += cronjob.resources_usage(None);
                cronjob.print_resources(&mut table);
            }
            KubeObject::PersistentVolumeClaim(pvc) => {
                resources += pvc.resources_usage();
                pvc.print_resources(&mut table);
            }
            _ => {}
        }
    }

    table.add_row(vec![
        "Total",
        "",
        "",
        &resources.requests_cpu.to_comfy_table_value(),
        &resources.limits_cpu.to_comfy_table_value(),
        &resources.requests_memory.to_comfy_table_value(),
        &resources.limits_memory.to_comfy_table_value(),
        &resources.requests_storage.to_comfy_table_value(),
        &resources.limits_storage.to_comfy_table_value(),
    ]);

    println!("{table}");

    Ok(resources)
}
