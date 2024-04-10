# MIDI mapping for Sylenth1

Example MIDI mapping for the Sylenth1 VST plugin. The example below shows the default mapping of the MIDI CC controls.

## MIDI channel

Channel 176 would correspond with channel 1.

```channel = 176```

## MIDI control

We can change the below controls by default. 

| Parameter name | MIDI CC nr |
|----|----|
Modwheel | 1 |
Portamento | 5 |
Main Volume | 7 |
Mix A | 8 |
Mix B | 9 |
Osc A1 Volume | 10 |
Osc A1 Phase | 11 |
Osc A1 Detune | 12 |
Osc A1 Stereo | 13 |
Osc A1 Pan | 14 |
Osc A2 Volume | 15 |
Osc A2 Phase | 16 |
Osc A2 Detune | 17 |
Osc A2 Stereo | 18 |
Osc A2 Pan | 19 |
Osc B1 Volume | 20 |
Osc B1 Phase | 21 |
Osc B1 Detune | 22 |
Osc B1 Stereo | 23 |
Osc B1 Pan | 24 |
Osc B2 Volume | 25 |
Osc B2 Phase | 26 |
Osc B2 Detune | 27 |
Osc B2 Stereo | 28 |
Osc B2 Pan | 29 |
Hold Pedal | 64 |
Chorus Delay | 65 |
Chorus Dry/Wet | 66 |
Filter A Drive | 67 |
Filter B Drive | 68 |
Filter A Resonance | 69 |
Filter B Resonance | 70 |
Filter Ctrl Resonance | 71 |
Filter A Cutoff | 72 |
Filter B Cutoff | 73 |
Filter Ctrl Cutoff | 74 |
Filter Ctrl Keytrack | 75 |
AmpEnv A Attack | 76 |
AmpEnv A Decay | 77 |
AmpEnv A Sustain | 78 |
AmpEnv A Release | 79 |
AmpEnv B Attack | 80 |
AmpEnv B Decay | 81 |
AmpEnv B Sustain | 82 |
AmpEnv B Release | 83 |
LFO 1 Rate | 84 |
LFO 1 Gain | 85 |
LFO 1 Offset | 86 |
LFO 2 Rate | 87 |
LFO 2 Gain | 88 |
LFO 2 Offset | 89 |
Reverb Dry/Wet | 91 |
Delay Dry/Wet | 92 |
Distortion Amount | 93 |
Distortion Dry/Wet | 94 |
Phaser Dry/Wet | 95 |
Phaser CenterFreq | 96 |
Phaser Spread | 97 |
Reverb Predelay | 102 |
Reverb Damp | 103 |
Reverb Size | 104 |
Reverb Width | 105 |
Delay Time Left | 106 |
Delay Time Right | 107 |
Delay LowCut | 108 |
Delay HighCut | 109 |
Delay Smear | 110 |
Delay Spread | 111 |
Delay Feedback | 112 |
Delay Width | 113 |
Phaser LR Offset | 114 |
Phaser Width | 115 |
Phaser LFO Rate | 116 |
Phaser LFO Gain | 117 |
Phaser Feedback | 118 |
EQ Bass | 119 |
All Notes Off | 123 |
Comp Threshold | 124 |
Comp Attack | 125 |
Comp Release | 127 |

## MQTT messages

With the configuration detailed above, we can now send messages using MQTT and have them translated to the corresponding MIDI CC messages.

| Topic | Value | Effect |
|----|----|----|
| midi/185/10 | 127 | Set OSC A1 volume to 127 |
| midi/185/14 | 127 | Set OSC A1 Pan to 127 |
| midi/185/84 | 66 | Set LFO 1 Rate to 66 |