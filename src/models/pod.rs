use serde::{Serialize};

#[derive(Debug, Serialize, Copy, Clone, Hash)]
pub enum PodState {
    Active,
    Dead,
}

#[derive(Debug, Serialize, Hash, Clone)]
pub struct Pod {
    pub name: String,
    pub slug: String,
    pub state: PodState,
    pub docker_image: String,
    pub docker_tag: String,
    port: MappedPort
}

#[derive(Debug, Serialize, Copy, Clone, Hash)]
pub struct MappedPort {
    exported_port: u16,
    mapped_port: Option<u16>
}

impl MappedPort {
    pub fn new(exported_port: u16, mapped_port: Option<u16>) -> MappedPort {
        MappedPort {
            exported_port,
            mapped_port
        }
    }
}


impl Pod {
    pub fn new(name: String, slug: String, docker_image: String, docker_tag: String, port: MappedPort) -> Pod {
        Pod {
            name,
            docker_image,
            docker_tag,
            state: PodState::Active,
            slug,
            port
        }
    }

    pub fn add_mapped_port(&mut self, map_to_port: u16) {
        self.port = MappedPort {
            exported_port: self.port.exported_port,
            mapped_port: Some(map_to_port)
        }
    }

    pub fn get_mapped_port(&self) -> Option<u16> {
        self.port.mapped_port
    }

    pub fn get_exposed_port(&self) -> u16 {
        self.port.exported_port
    }
}