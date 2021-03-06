extern crate dockworker;

use dockworker::{ContainerListOptions, Docker};

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let opts = ContainerListOptions::default();
    if let Some(container) = docker.containers(opts).unwrap().get(0) {
        for process in docker.processes(container.Id.as_str()).unwrap() {
            println!("{:#?}", process);
        }
    }
}
