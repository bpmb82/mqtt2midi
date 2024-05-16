#from encoder import Encoder
import json

from gpiozero import RotaryEncoder
from threading import Event
import paho.mqtt.client as mqtt


client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION1)

rotors = []

def main():
    config = get_config_file()
    client.username_pw_set(config["mqtt"]['username'], config["mqtt"]['password'])
    client.connect(config["mqtt"]["host"], 1883, 60)
    client.on_connect = on_connect
    client.on_message = on_message
    parse_buttons_from_config(config)
    client.loop_forever()


def get_config_file():
    try:
        with open("config.json", "r") as f:
            config = json.load(f)
            return config
    except FileNotFoundError:
        print("config.json not found")
        exit()


def parse_buttons_from_config(config):
    for x in config["buttons"]:
        print(x)
        if not x.get("gpio", None):
            continue
        rotor = RotaryEncoder(x["gpio"]["a"], x["gpio"]["b"], wrap=False, max_steps=63)
        rotor.when_rotated = callback
        rotors.append(rotor)



def on_connect(client_o, userdata, flags, rc):
    print("Connected with result code " + str(rc))


def on_message(client_o, userdata, msg):
    print(msg.payload.decode())


def callback(btn):
    value = ((btn.steps + 63) / 126) * 126
    print(int(value))
    client.publish("midi/176/11", int(value), qos=1)


if __name__ == '__main__':
    main()
