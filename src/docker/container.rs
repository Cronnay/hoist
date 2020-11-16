use crate::models::pod::Pod;
use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions,
    StartContainerOptions,
};
use bollard::models::{ContainerCreateResponse, ContainerInspectResponse, ContainerSummaryInner};
use bollard::Docker;
use bollard_stubs::models::{HostConfig, PortBinding, PortMap};
use std::collections::HashMap;

pub async fn get_docker_container(
    name_or_id: &str,
    docker: &Docker,
) -> Result<ContainerInspectResponse, bollard::errors::Error> {
    let options = InspectContainerOptions { size: false };
    Ok(docker.inspect_container(&name_or_id, Some(options)).await?)
}

pub async fn get_docker_containers(
    docker: &Docker,
) -> Result<Vec<ContainerSummaryInner>, bollard::errors::Error> {
    let options = ListContainersOptions::<&str> {
        all: true,
        ..Default::default()
    };
    let containers = docker.list_containers(Some(options)).await?;
    Ok(containers.clone())
}

pub async fn create_and_start_docker_container(docker: &Docker, created_pod: &Pod) {
    let options = Some(CreateContainerOptions {
        name: &created_pod.slug,
    });

    let mut export_ports_hash = HashMap::new();
    export_ports_hash.insert(
        format!("{}/tcp", created_pod.get_exposed_port()),
        HashMap::new(),
    );

    let mut mapped_ports_hash: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();
    mapped_ports_hash.insert(
        format!("{}/tcp", created_pod.get_exposed_port()),
        Some(vec![PortBinding {
            host_port: Some(created_pod.get_mapped_port().unwrap().to_string()),
            ..Default::default()
        }]),
    );

    let config = Config {
        image: Some(format!(
            "{}:{}",
            created_pod.docker_image, created_pod.docker_tag
        )),
        exposed_ports: Some(export_ports_hash),
        host_config: Some(HostConfig {
            port_bindings: Some(mapped_ports_hash),
            ..Default::default()
        }),
        ..Default::default()
    };

    let created_container = docker.create_container(options, config).await;
    match created_container {
        Ok(container) => {
            docker
                .start_container(&container.id, None::<StartContainerOptions<String>>)
                .await
                .unwrap();
        }
        Err(err) => panic!(format!("could not create response: {}", err)),
    }
}
