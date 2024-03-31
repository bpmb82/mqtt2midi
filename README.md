# mqtt2midi

A small application in Rust to forward/ convert incoming MQTT messages into MIDI messages.

## Configuration

The application expects a config.toml file next to it which contains the MQTT broker information as well as which MIDI port to use.

_Example config.toml_
```
[mqtt]
host = "192.168.1.250"
port = 1883
topic = "midi/#"

[midi]
port = "MIDI Out 1"
```

## CLI options

You have to specify one of the below cli options:

```-l``` Lists all the MIDI output devices available on the system*

```-d``` Run in daemon mode 

_*For Windows, install LoopMIDI to get a working virtual port_

## Daemon mode

In daemon mode, the application takes the last part of the topic name as the MIDI channel to use (e.g. midi/176 would become 176 which is CH1 MIDI CC). The topic itself must contain a payload in json that holds the 'control' and the 'value':

```
{
    "control": 10,
    "value": 127
}
```
You can find a list of MIDI channels here: https://midi.org/expanded-midi-1-0-messages-list


## Todo's

* Support authentication on the MQTT broker
* ~~Take topic name as channel so we can have dedicated topics per channel~~