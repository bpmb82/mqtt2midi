#!/bin/bash

if ! command -v mosquitto_pub &> /dev/null
then
    echo "ERROR: mosquitto_pub could not be found, please install it."
    exit 1
fi

for i in {0..10}
do
    for value in {0..127}
    do
        for control in {10..29}
        do
            mosquitto_pub -h 192.168.1.250 -p 1883 -t midi/176/$control -m $value && sleep 0.1 &
        done
    done
done