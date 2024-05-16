import RPi.GPIO as GPIO
from time import sleep


class Encoder:
    def __init__(self, gpio_a, gpio_b):
        self.Enc_A = gpio_a
        self.Enc_B = gpio_b
        self.counter = 0
        GPIO.setup(self.Enc_A, GPIO.IN)
        GPIO.setup(self.Enc_B, GPIO.IN)
        GPIO.add_event_detect(self.Enc_A, GPIO.RISING, callback=self.rotation_decode,
                              )  # bouncetime in mSec
        print("initialized encoder {0} and {1}".format(gpio_a, gpio_b))

    def rotation_decode(self, Enc_A):
        print("rotation")
        sleep(0.002)  # extra 2 mSec de-bounce time

        # read both of the switches
        Switch_A = GPIO.input(self.Enc_A)
        Switch_B = GPIO.input(self.Enc_B)

        if (Switch_A == 1) and (Switch_B == 0):  # A then B ->
            self.counter += 1
            print("direction -> ", self.counter)
            # at this point, B may still need to go high, wait for it
            while Switch_B == 0:
                Switch_B = GPIO.input(self.Enc_B)
            # now wait for B to drop to end the click cycle
            while Switch_B == 1:
                Switch_B = GPIO.input(self.Enc_B)
            return

        elif (Switch_A == 1) and (Switch_B == 1):  # B then A <-
            self.counter -= 1
            print("direction <- ", self.counter)
            # A is already high, wait for A to drop to end the click cycle
            while Switch_A == 1:
                Switch_A = GPIO.input(Enc_A)
            return

        else:  # discard all other combinations
            print("discarded?")
            return
