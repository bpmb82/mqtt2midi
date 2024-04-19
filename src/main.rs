use toml;
use serde::Deserialize;
use std::fs;
use clap::Parser;
use tokio;
use tokio::time::Duration;
use in_range::in_range;

use midir::{MidiOutput, MidiOutputPort};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Run in daemon mode
    #[arg(short='d', long, default_value = "false")]
    daemon: bool,

    /// List MIDI devices
    #[arg(short='l', long, default_value = "false")]
    list: bool,

}

#[derive(Debug, Deserialize)]
struct Config {
    mqtt: Mqtt,
    midi: Midi
}

#[derive(Debug, Deserialize)]
struct Mqtt {
    host: String,
    port: i32,
    topic: String,
    qos: i32
}

#[derive(Debug, Deserialize)]
struct Midi {
    port: String
}

fn print_help(msg: &str) {
    eprintln!("{}

Example config.toml:

[mqtt]
host = '127.0.0.1'
port = 1883
qos = 0
topic = 'example/topic/#'

[midi]
port = 'MIDI Out 1'", msg)
}

fn get_config_from_toml() -> Config {
    let Ok(toml_str) = fs::read_to_string("config.toml") else {
        print_help("ERROR: could not read config.toml file, this file needs to exist next to the executable.");
        std::process::exit(0x0100);
    };
    let Ok(config) = toml::from_str(&toml_str) else {
        print_help("ERROR: could not parse config.toml file.");
        std::process::exit(0x0100);
    };
    config
}

fn get_qos(qos: i32) -> QoS {
    match qos {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        2 => QoS::ExactlyOnce,
        _ => panic!("Invalid QoS value, needs to be a value of 0, 1 or 2"),
    }
}

fn list_midi_ports() {
    let Ok(midi_out) = MidiOutput::new("mqtt2midi") else {
        eprintln!("ERROR: could not query MIDI devices");
        std::process::exit(0x0100);
    };
    let out_ports = midi_out.ports();
    if out_ports.len() == 0 {
        eprintln!("ERROR: No MIDI output ports found!");
        std::process::exit(0x0100);
    };
    println!("\nAvailable output ports:");
    for port in out_ports.iter() {
        println!("{}", midi_out.port_name(port).unwrap());
    }
    std::process::exit(0x0000);
}

async fn daemon_mode() {
    let config = get_config_from_toml();
    println!("INFO: running in daemon mode");
    println!("INFO: Config file found and loaded");
    let Ok(midi_out) = MidiOutput::new("mqtt2midi") else {
        eprintln!("ERROR: could not query MIDI devices");
        std::process::exit(0x0100);
    };
    let midiports = midi_out.ports();
    let mut midiport: &MidiOutputPort = &midiports[0];
    for (i, port) in midiports.iter().enumerate() {
        if midi_out.port_name(port).unwrap() == config.midi.port {
            midiport = &midiports[i];
        }
    }
    let Ok(mut conn_out) = midi_out.connect(midiport, "mqtt2midi") else {
        eprintln!("ERROR: could not open MIDI port {}", config.midi.port);
        std::process::exit(0x0100);
    };
    println!("INFO: MIDI port connected: {}", config.midi.port);
    let mut mqttoptions = MqttOptions::new("mqtt2midi", config.mqtt.host, config.mqtt.port as u16);
    mqttoptions.set_keep_alive(Duration::from_secs(2));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let Ok(_) = client.subscribe(&config.mqtt.topic, get_qos(config.mqtt.qos)).await else {
        eprintln!("ERROR: unable to subscribe to topic {}", &config.mqtt.topic);
        std::process::exit(0x0100);
    };
    println!("INFO: MQTT connected and listening on topic '{}'", config.mqtt.topic);

    let mut play_midi = |channel: u8, control: u8, value: u8| {
        let _ = conn_out.send(&[channel, control, value]);
    };

    loop {
        let notification = eventloop.poll().await;
        if let Event::Incoming(Packet::Publish(event)) = notification.unwrap() {
            let topic = String::from(&event.topic);
            if topic.split("/").count() >= 3 as usize {
                let Some(control) = topic.split("/").nth(topic.split("/").count() -1) else {
                    eprintln!("ERROR: Could not get 'control' from topic");
                    continue;
                };
                let Some(channel) = topic.split("/").nth(topic.split("/").count() -2) else {
                    eprintln!("ERROR: Could not get 'channel' from topic");
                    continue;
                };
                let Ok(control) = control.parse::<i32>() else {
                    eprintln!("ERROR: Could not parse control!");
                    continue;
                };
                let Ok(channel) = channel.parse::<i32>() else {
                    eprintln!("ERROR: Could not parse channel!");
                    continue;
                };
                let Ok(value) = String::from_utf8(event.payload.to_ascii_lowercase()) else {
                    eprintln!("ERROR: Could not parse payload value");
                    continue;
                };
                let Ok(value) = value.parse::<i32>() else {
                    eprintln!("ERROR: Could not parse payload value");
                    continue;
                };
                if in_range(value, 0, 127) {
                    play_midi(channel as u8, control as u8, value as u8);
                    println!("CH {} | CC {} | VAL {}", channel, control, value);
                } else {
                    eprintln!("ERROR: Control value out of bounds, keep value between 0-127")
                }
            }
            if topic.split("/").count() == 2 as usize {
                let Some(channel) = topic.split("/").nth(topic.split("/").count() -1) else {
                    eprintln!("ERROR: Could not get 'channel' from topic");
                    continue;
                };
                let Ok(channel) = channel.parse::<i32>() else {
                    eprintln!("ERROR: Could not parse channel!");
                    continue;
                };
                let Ok(value) = String::from_utf8(event.payload.to_ascii_lowercase()) else {
                    eprintln!("ERROR: Could not parse payload value");
                    continue;
                };
                let Ok(value) = value.parse::<i32>() else {
                    eprintln!("ERROR: Could not parse payload value");
                    continue;
                };
                if in_range(value, 0,127) {
                    play_midi(channel as u8, value as u8, value as u8);
                    println!("CH {} | PC {}", channel, &value);
                } else {
                    eprintln!("ERROR: Control value out of bounds, keep value between 0-127")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if cli.daemon && cli.list {
        println!("ERROR: Cannot run in daemon and list mode simultaneously")
    } else if cli.daemon {
        daemon_mode().await;
    } else if cli.list {
        list_midi_ports();
    } else {
        println!("Run 'mqtt2midi --help' for instructions")
    };
}