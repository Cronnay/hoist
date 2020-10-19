use bollard::Docker;
pub mod image;

pub fn get_docker_instance() -> Docker {
    let docker = Docker::connect_with_unix_defaults();
    match docker {
        Ok(d) => d,
        Err(_err) => panic!("Could not connect to docker!"),
    }
}
