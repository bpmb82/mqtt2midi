import RPi.GPIO as GPIO


class Encoder:
    def __init__(self, clk_pin, dt_pin):
        self.clk_pin = clk_pin
        self.dt_pin = dt_pin
        self.counter = 0

        # Set up GPIO mode and pins
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(clk_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)
        GPIO.setup(dt_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)
        self.clk_last_state = GPIO.input(clk_pin)
        # Set up event detection for CLK (clock) signal
        GPIO.add_event_detect(clk_pin, GPIO.RISING, callback=self.handle_CLK_rising)

    def handle_CLK_rising(self, channel):
        dt_state = GPIO.input(self.dt_pin)
        if dt_state != self.clk_last_state:
            self.counter += 1
        else:
            self.counter -= 1
        print(f"Encoder count: {self.counter}")
