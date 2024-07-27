# mqtt2midi

A small application in Rust to forward/ convert incoming MQTT messages into MIDI messages. This application was made to drive a drumcomputer and synthesizer remotely using MIDI CC and Program Changes, notes have not been tested.

## Configuration

The application expects a config.toml file next to it which contains the MQTT broker information as well as which MIDI port to use.

_Example config.toml_
```
[mqtt]
host = "192.168.1.250"
port = 1883
topic = "midi/#"
username: "username" (optional)
password: "password" (optional)

[midi]
port = "MIDI Out 1"
```

## CLI options

You have to specify one of the below cli options:

```-l``` Lists all the MIDI output devices available on the system*

```-d``` Run in daemon mode 

_*For Windows, install [LoopMIDI](https://www.tobias-erichsen.de/software/loopmidi.html) to get a working virtual port_

## Daemon mode

In daemon mode, the application takes the last 2 parts of the topic name as the MIDI channel and control number to use (e.g. topic 'midi/176/10' would become 'channel' 176 which is CH1 MIDI CC with 'control' number 10). The topic itself must contain a payload in raw that holds the value as integer (value between 0-127):

You can find a list of MIDI channels [here](https://midi.org/expanded-midi-1-0-messages-list).

An example of a topic would be: ```midi/185/10```. We could then send a raw value between 0 and 127 which would send a MIDI CC 10 message for channel 1.

If you send a raw value to just the channel, you can control Program Changes. E.g. ```midi/185```.

## Todo's

* Support authentication on the MQTT broker
* ~~Take topic name as channel so we can have dedicated topics per channel~~