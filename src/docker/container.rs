use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::models::{ContainerInspectResponse, ContainerSummaryInner};
use bollard::Docker;

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
