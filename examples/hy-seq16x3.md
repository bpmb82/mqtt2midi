# MIDI mapping for HY-SEQ16x3

Example MIDI mapping for the hy-seq16x3 VST plugin. The example below can be used to trigger a drum plugin but could be expanded upon to control a synthesizer as well.

## MIDI channel

Channel 185 would correspond with channel 10.

```channel = 185```

## MIDI control

### global

The below CC messages control transpose, this allows us to switch all the steps up or down at will to select other sounds
!!! NOTE !!! 
Needs testing to see how to go up or down a single NOTE

```transpose = 0C```

The below controls the 'probability' parameter of this plugin. Set this to 0 or 127 to turn on/ off a step.
```
seq1_prob_1 = 10
seq1_prob_2 = 11
seq1_prob_3 = 12
seq1_prob_4 = 13
seq1_prob_5 = 14
seq1_prob_6 = 15
seq1_prob_7 = 16
seq1_prob_8 = 17
seq1_prob_9 = 18
seq1_prob_10 = 19
seq1_prob_11 = 1A
seq1_prob_12 = 1B
seq1_prob_13 = 1C
seq1_prob_14 = 1D
seq1_prob_15 = 1E
seq1_prob_16 = 1F
```

## MQTT messages

With the configuration detailed above, we can now send messages using MQTT and have them translated to the corresponding MIDI CC messages.

| Topic | Value | Effect |
|----|----|----|
| midi/185/10 | 127 |  turn on the first note in the sequence |
| midi/185/14 | 127 |  turn on the 5th note in the sequence |
| midi/185/0C | 66 | transpose the entire sequence |