use depres::{self, depres::read_deployment_resources, k8s::resources_usage::ResourceUsage};
use std::fs;

#[test]
fn test_stateful_set() {
    let contents =
        fs::read_to_string("tests/test_data/stateful_set.yml").expect("Failed to read the file");
    let res = read_deployment_resources(vec![contents]);
    let expected = ResourceUsage {
        requests_cpu: Some(1.25),
        limits_cpu: Some(2.5),
        requests_memory: Some(2560.0),
        limits_memory: Some(5120.0),
        requests_storage: Some(25600.0),
        limits_storage: None,
    };

    assert!(res.is_ok());

    if let Ok(res) = res {
        assert_eq!(res, expected)
    }
}

#[test]
fn test_cron_job() {
    let contents =
        fs::read_to_string("tests/test_data/cron_job.yml").expect("Failed to read the file");
    let res = read_deployment_resources(vec![contents]);
    let expected = ResourceUsage {
        requests_cpu: Some(0.1),
        limits_cpu: Some(0.2),
        requests_memory: Some(256.0),
        limits_memory: Some(512.0),
        requests_storage: None,
        limits_storage: None,
    };

    assert!(res.is_ok());

    if let Ok(res) = res {
        assert_eq!(res, expected)
    }
}

#[test]
fn test_deployments() {
    let contents =
        fs::read_to_string("tests/test_data/deployments.yml").expect("Failed to read the file");
    let res = read_deployment_resources(vec![contents]);
    let expected = ResourceUsage {
        requests_cpu: Some(2.75),
        limits_cpu: Some(4.0),
        requests_memory: Some(5632.0),
        limits_memory: Some(8192.0),
        requests_storage: Some(5120.0),
        limits_storage: None,
    };

    assert!(res.is_ok());

    if let Ok(res) = res {
        assert_eq!(res, expected)
    }
}

#[test]
fn test_all() {
    let files = [
        "tests/test_data/stateful_set.yml",
        "tests/test_data/cron_job.yml",
        "tests/test_data/deployments.yml",
    ]
    .iter()
    .map(|path| fs::read_to_string(path).expect("Failed to read the file"))
    .collect();

    let res = read_deployment_resources(files);
    let expected = ResourceUsage {
        requests_cpu: Some(4.1),
        limits_cpu: Some(6.7),
        requests_memory: Some(8448.0),
        limits_memory: Some(13824.0),
        requests_storage: Some(30720.0),
        limits_storage: None,
    };

    assert!(res.is_ok());

    if let Ok(res) = res {
        assert_eq!(res, expected)
    }
}
