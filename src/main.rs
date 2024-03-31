use clap::builder::Str;
use toml;
use serde::{Deserialize};
use std::fs;
use clap::Parser;
use tokio;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiInputPort, MidiOutput, MidiOutputPort};
use rumqttc::{AsyncClient, Event, MqttOptions, Publish, Packet, QoS};

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
    topic: String
}

#[derive(Debug, Deserialize)]
struct Midi {
    port: String
}

#[derive(Deserialize)]
struct MidiMessage {
    channel: i32,
    control: i32,
    value: i32
}

fn get_config_from_toml() -> Config {
    let Ok(toml_str) = fs::read_to_string("config.toml") else {
        println!("ERROR: could not read config.toml file, this file needs to exist next to the executable.");
        std::process::exit(0x0100);
    };
    let Ok(config) = toml::from_str(&toml_str) else {
        println!("ERROR: could not parse config.toml file.
        
    Example config.toml:

    [mqtt]
    host = '127.0.0.1'
    port = 1883
    [midi]
    port = 'MIDI Out 1'");
        std::process::exit(0x0100);
    };
    config
}

fn list_midi_ports() {
    let Ok(midi_out) = MidiOutput::new("mqtt2midi") else {
        println!("ERROR: could not query MIDI devices");
        std::process::exit(0x0100);
    };

    let out_ports = midi_out.ports();
    if out_ports.len() == 0 {
        println!("No MIDI output ports found!");
        std::process::exit(0x0100);
    };
    println!("\nAvailable output ports:");
    for port in out_ports.iter() {
        println!("{}", midi_out.port_name(port).unwrap());
    }
    std::process::exit(0x0000);
}

async fn daemon_mode() {
    println!("We are running in daemon mode!");
    let config = get_config_from_toml();
    println!("Config file found and loaded");
    let Ok(midi_out) = MidiOutput::new("mqtt2midi") else {
        println!("ERROR: could not query MIDI devices");
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
        println!("ERROR: could not open MIDI port {}", config.midi.port);
        std::process::exit(0x0100);
    };
    println!("MIDI port connected");
    let mut mqttoptions = MqttOptions::new("mqtt2midi", config.mqtt.host, config.mqtt.port as u16);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(&config.mqtt.topic, QoS::AtMostOnce).await.unwrap();
    println!("MQTT connected and listening on topic '{}'", config.mqtt.topic);

    let mut play_midi = |channel: u8, control: u8, value: u8| {
        let _ = conn_out.send(&[channel, control, value]);
    };

    loop {
        let notification = eventloop.poll().await;
        if let Event::Incoming(Packet::Publish(event)) = notification.unwrap() {
            let payload: MidiMessage = serde_json::from_slice(&event.payload).unwrap();
            play_midi(payload.channel as u8, payload.control as u8, payload.value as u8);
            println!("CH {} | CC {} | VAL {}", payload.channel as u8, payload.control as u8, payload.value as u8);
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if cli.daemon && cli.list {
        println!("Cannot run in daemon and list mode simultaneously")
    } else if cli.daemon {
        daemon_mode().await;
    } else if cli.list {
        list_midi_ports();
    } else {
        println!("You should specify list (-l) or daemon (-d) mode")
    }
}
