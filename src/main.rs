use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// Define the YAML structure
#[derive(Debug, Deserialize)]
struct WifiNetworks {
    my_wifi_networks: Vec<WifiNetwork>,
}

#[derive(Debug, Deserialize)]
struct WifiNetwork {
    ssid: String,
    password: String,
}

// Function to check if connected to a WiFi network
fn is_connected() -> bool {
    let output = Command::new("iwgetid")
        .arg("-r")
        .output()
        .expect("Failed to execute iwgetid");

    !output.stdout.is_empty()
}

// Function to connect to a WiFi network
fn connect_to_wifi(networks: WifiNetworks) {
    for network in networks.my_wifi_networks {
        println!("Trying to connect to WiFi: {}", network.ssid);

        let output = Command::new("nmcli")
            .args([
                "device",
                "wifi",
                "connect",
                &network.ssid,
                "password",
                &network.password,
                "ifname",
                "wlan0",
            ])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("Successfully connected to {}", network.ssid);
                    return;
                } else {
                    println!(
                        "Failed to connect to {}: {}",
                        network.ssid,
                        String::from_utf8_lossy(&result.stderr)
                    );
                }
            }
            Err(e) => {
                println!("Error connecting to {}: {}", network.ssid, e);
            }
        }

        sleep(Duration::from_secs(5)); // Wait before trying the next network
    }

    println!("Could not connect to any networks.");
}

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_yaml_file>", args[0]);
        std::process::exit(1);
    }

    let wifi_config_path = &args[1];

    // Check if already connected
    if is_connected() {
        println!("Already connected to WiFi.");
        return;
    }

    // Read and parse YAML file
    let yaml_content = fs::read_to_string(wifi_config_path)
        .expect("Failed to read the specified YAML file");

    let wifi_networks: WifiNetworks = serde_yaml::from_str(&yaml_content)
        .expect("Failed to parse YAML file");

    // Try to connect
    connect_to_wifi(wifi_networks);
}
