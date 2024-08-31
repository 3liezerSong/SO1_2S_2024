use serde::Deserialize;
use std::{fs::File, io::{self, Read}, path::Path};

#[derive(Deserialize)]
struct ContainerInfo {
    name: String,
    cpu_usage: f32,
    ram_usage: f32,
}

#[derive(Debug, thiserror::Error)]
enum MonitorError {
    #[error("Error leyendo el archivo: {0}")]
    Io(#[from] io::Error),
    #[error("Error al parsear JSON: {0}")]
    Json(#[from] serde_json::Error),
}

fn main() -> Result<(), MonitorError> {
    let path = "/proc/sysinfo_2000";

    match read_json_file(path) {
        Ok(containers) => {
            let (high_consumption, low_consumption) = identify_consumption(containers);
            print_consumption("Alto consumo", &high_consumption);
            print_consumption("Bajo consumo", &low_consumption);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<ContainerInfo>, MonitorError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let containers: Vec<ContainerInfo> = serde_json::from_str(&contents)?;
    Ok(containers)
}

fn identify_consumption(containers: Vec<ContainerInfo>) -> (Vec<ContainerInfo>, Vec<ContainerInfo>) {
    let (high_consumption, low_consumption): (Vec<_>, Vec<_>) = containers
        .into_iter()
        .partition(|container| container.cpu_usage > 70.0 || container.ram_usage > 70.0);

    (high_consumption, low_consumption)
}

fn print_consumption(title: &str, containers: &[ContainerInfo]) {
    println!("Contenedores de {}:", title);
    for container in containers {
        println!(
            "Nombre: {}, CPU: {}%, RAM: {}%",
            container.name, container.cpu_usage, container.ram_usage
        );
    }
    println!();
}
