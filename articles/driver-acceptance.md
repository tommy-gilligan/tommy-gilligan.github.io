+++
title = "Zsh Keybinding Reminders"
description = "A small flagellator to learn more efficient use of Zsh"
published = true
published_at = 2023-01-01T01:01:01Z
+++
Bootsel
Establishing that we are using Pico
Maybe not so much justification is necessary
Explain the project
Pick a single sensor
Existing driver
Running MicroPython example

Explain the usefulness of porting the example (sans testing)
Show how you run the example (via prope)

Moving from MicroPython to Rust's `embedded-hal`, it makes little sense to get
too attached to the existing API.  I don't feel like it makes sense to try too
hard to replicate the API of the official PiicoDev drivers.  How then will I
decide if my drivers are ready to be released?  One potential criterion for
whether my new drivers pass mustard:

> Can these new drivers be used for all of the same things as the existing drivers?

Such a criterion is frustratingly broad.  The set of specific uses for the
drivers is basically endless.  Thankfully the offical drivers come with a large but finite number 

A nice bonus here is that by using this as the acceptance criterion, examples
need to also be ported (and code examples are an indispensible artefact for
library users).

### Laser Distance Sensor VL53L1X (*p7*)
I've picked just a single PiicoDev device to focus on for now: [Laser Distance
Sensor VL53L1X](https://piico.dev/p7).  This device is identified as *p7* in
PiicoDev URLs.  *p7* is a convenient shorthand to refer to this device and is
what I'll be sticking to.  The [official
driver](https://github.com/CoreElectronics/CE-PiicoDev-VL53L1X-MicroPython-Module.git)
includes a single, simple example for *p7*:

```python
from PiicoDev_VL53L1X import PiicoDev_VL53L1X
from time import sleep

distSensor = PiicoDev_VL53L1X()

while True:
    # read the distance in millimetres
    dist = distSensor.read()
    # convert the number to a string and print
    print(str(dist) + " mm")
    sleep(0.1)
```

### Running the MicroPython Example with Thonny
Core Electronics provides instructions for [How to Setup a Raspberry Pi Pico
and Code with
Thonny](https://core-electronics.com.au/guides/how-to-setup-a-raspberry-pi-pico-and-code-with-thonny/).
Thonny eases some of the pain points common to getting started with MicroPython:

- Downloading the right MicroPython interpreter to the Pico.
- Downloading 3rd-party MicroPython libraries to the Pico.
- Initiating a MicroPython session on the Pico for code execution. 

[A bit of further instruction](https://core-electronics.com.au/guides/raspberry-pi-pico/piicodev-distance-sensor-vl53l1x-raspberry-pi-pico-guide/) is all that is needed to get the above *p7* example running in Thonny.  Summarising:

1. Install an appropriate MicroPython interpreter on the Pico.<br>
    <span style="display: flex"><img style="max-width: 60%" src="thonny-1.png"><img style="max-width: 60%" src="thonny-2.png"></span>
2. Copy the [PiicoDev Unified Library](https://raw.githubusercontent.com/CoreElectronics/CE-PiicoDev-Unified/main/PiicoDev_Unified.py) to the Pico filesystem.
3. Copy the [PiicoDev_VL53L1X.py](https://raw.githubusercontent.com/CoreElectronics/CE-PiicoDev-VL53L1X-MicroPython-Module/main/PiicoDev_VL53L1X.py) to the Pico filesystem.
4. Run the above example. <br>
    <span><img style="max-width: 60%" src="thonny-3.png"></span>

### Running the MicroPython Example with `mpremote`
Thonny is great but for automation MicroPython makes available a handy tool
called
[`mpremote`](https://docs.micropython.org/en/latest/reference/mpremote.html).
Steps 2-4 of *Running the MicroPython Example with Thonny* can be done through `mpremote`
In other words: `mpremote` can be used for basically all of what Thonny can be
used for bar installing the interpreter.  Without Thonny to find the right
MicroPython interpreter for your specific development board, you'll need to find the right one at [MicroPython Downloads](https://micropython.org/download/).  You're probably after either
- [MicroPython for Raspberry Pi Pico W](https://micropython.org/resources/firmware/RPI_PICO_W-20230426-v1.20.0.uf2) or
- [MicroPython for Raspberry Pi Pico](https://micropython.org/resources/firmware/RPI_PICO-20230426-v1.20.0.uf2)
Download one of these.

1. Hold BOOTSEL button during connection then `cp DOWNLOADED_INTERPRETER DEVBOARD_MOUNTPOINT`
2. `mpremote cp PiicoDev_Unified.py :`
3. `mpremote cp PiicoDev_VL53L1X.py :`
4. `mpremote run main.py`

Condensing this into a script:
Refer to repos

script.sh
```bash
# connect to Pico with BOOTSEL held
# assumes that it has been mounted to DEVBOARD_MOUNTPOINT
set -e
DOWNLOADED_INTERPRETER=$1
DEVBOARD_MOUNTPOINT=$2
echo Copying MicroPython interpreter to devboard
if cp -X -v $DOWNLOADED_INTERPRETER $DEVBOARD_MOUNTPOINT
then
  :
else
  echo Make sure the Pico is mounted at $DEVBOARD_MOUNTPOINT
  exit 1
fi
echo Spamming intepreter connection until it is ready
until mpremote eval True >/dev/null
do
  :
done
echo Copying libraries to devboard
mpremote cp CE-PiicoDev-Unified/min/PiicoDev_Unified.py \
  CE-PiicoDev-VL53L1X-MicroPython-Module/min/PiicoDev_VL53L1X.py :
echo Running example
mpremote run CE-PiicoDev-VL53L1X-MicroPython-Module/main.py
```

The example runs indefinitely but can be disconnected from with
<kbd>Ctrl</kbd>+<kbd>C</kbd>.  So we've been able to automate basically all of
the steps involved in running this example except

- selecting and downloading the right interpreter
- putting the Pico in the right state for installing the interpreter

Being familiar with running code on the Pico with probe-rs, needing to connect
the Pico in BOOTSEL mode to install the interpreter seems unnecessary.  
[link to official SWD]
I don't have one of these handy but it's not hard to build effectively the same
thing from a spare Pico.

Lowers barrier to entry
`mpremote` does provide `mpremote bootloader` but can this be used if MicroPython is not already installed
MicroPython binaries are distributed as UF2.  UF2 is a pretty handy format, it
is designed expressly for ease-of-use: a device makes itself available as 'mass
storage' (Pico's BOOTSEL) and any UF2 file copied to this storage is treated as
a binary to run.  Unfortunately, `probe-rs` does not accept UF2 files for
programming.  This ends up being a fairly minor problem to overcome.
An official tool from Microsoft makes it trivial to convert any UF2 file to a
format acceptable to `probe-rs`:

```bash
DOWNLOADED_INTERPRETER=$1
git clone --depth 1 https://github.com/microsoft/uf2.git uf2
uf2/utils/uf2conv.py $DOWNLOADED_INTERPRETER
```
<pre><samp>--- UF2 File Header Info ---
Family ID is RP2040, hex value is 0xe48bff56
Target Address is 0x10000000
All block flag values consistent, 0x2000
----------------------------
Converted to bin, output size: 700416, start address: 0x10000000
Wrote 700416 bytes to flash.bin<samp></pre>
The output here is important.  It gives us:

- the output filename: `flash.bin`
- the start address: `0x10000000`

This information feeds into `probe-rs`

```bash
probe-rs download \
    --protocol swd \
    --chip RP2040 \
    --format bin \
    --base-address 0x10000000 \
    flash.bin
```

`probe-rs` does provide a mechanism ([Real-Time
Transfer](https://github.com/probe-rs/rtt-target)) that can share the USB
connection used by the probe for programming the target.  As far as I can tell,
this is not something MicroPython supports.

That is not a problem though because using a USB connection to the probe to install the
MicroPython interpreter does not preclude a separate but concurrent USB connection
to the target.  In other words, `probe-rs` can have its connection for installing MicroPython and
`mpremote` can have its connection for communicating with the interpreter.

Bringing this all together:
```bash
DOWNLOADED_INTERPRETER=$1

git clone --depth 1 https://github.com/microsoft/uf2.git uf2
uf2/utils/uf2conv.py $DOWNLOADED_INTERPRETER
probe-rs download \
    --protocol swd \
    --chip RP2040 \
    --format bin \
    --base-address 0x10000000 \
    flash.bin
```

A Rust version of this example that corresponds fairly directly to the
original:
```rust
loop { // read the distance in millimetres let dist =
dist_sensor.read().unwrap(); // convert the number to a string and print
println!("{} m", dist.to_f64() / 1000.0_f64); delay.delay_us(100_000); }
```

In both cases, readings from *p7* will be printed every decisecond
indefinitely: <pre><samp>&hellip; 1822 mm 1825 mm 1823 mm 1824 mm 1820 mm
&hellip;</samp></pre>

To compare the results from these example programs it is necessary to prevent
them from running indefinitely.

Spurious differences between readings Taking the range over 1000 readings and
doubling, we decide that that is the max acceptable delta

<pre><code>% probe-run target/thumbv6m-none-eabi/debug/examples/p7 \ --chip
RP2040 --no-location 2>/dev/null \ | head -n1000 \ | sort -nk1 \ | uniq -c
<samp>   4 1819 mm 12 1820 mm 71 1821 mm 203 1822 mm 334 1823 mm 255 1824 mm 98
1825 mm 14 1826 mm 9 1827 mm</samp></code></pre> We're making a statement about
the stability of a real-world environment.  Even if this is a controlled
environment, it begs the question: just how controlled is this environment?  If
we intend others to run these tests, how strict do we want to be about their
environment? Sensor noise is a source of error but also subtle movement of
sensor or the service it is targeting, atmospheric conditions, vibrations etc.

probe-run --chip RP2040 target/thumbv6m-none-eabi/debug/examples/p7 1 1819 mm
16 1820 mm 41 1821 mm 173 1822 mm 273 1823 mm 295 1824 mm 132 1825 mm 64 1826
mm 5 1827 mm

  1 1817 mm 12 1818 mm 132 1819 mm 1205 1820 mm 6573 1821 mm 18643 1822 mm
  29304 1823 mm 26601 1824 mm 13341 1825 mm 3580 1826 mm 548 1827 mm 57 1828 mm
  3 1829 mm

Despite the length of time between Python and Rust, maybe it makes sense to
treat them as part of the same series.

Using an unreleased version of `probe-rs` for the sake of a new flag:
`--no-location` ```bash cargo install --features cli --rev 481ffd5 \ --git
https://github.com/probe-rs/probe-rs.git \ probe-rs ```

To get the right format of image from binary distribution of MicroPython
```bash git clone https://github.com/microsoft/uf2.git
cd uf2/utils
# python3 from homebrew. other installations omit 3 from name
python3 uf2conv.py
  # find uf2 image appropriate for target at 
  # https://micropython.org/download/?port=rp2
  <(curl https://micropython.org/resources/firmware/RPI_PICO_W-20230426-v1.20.0.uf2)
  --convert --output mp-pico-w.bin
```
```
--- UF2 File Header Info ---
Family ID is RP2040, hex value is 0xe48bff56
Target Address is 0x10000000
All block flag values consistent, 0x2000
----------------------------
Converted to bin, output size: 700416, start address: 0x10000000
Wrote 700416 bytes to /Users/tom/piicodev-drivers/python/mp-pico-w.bin
```

```bash
RUST_ASSETS_DIR=target/thumbv6m-none-eabi/debug/examples
OUR_EXAMPLE_NAME=p7

MICROPYTHON_ASSETS_DIR=python
MICROPYTHON_BIN=mp-pico-w.bin
MICROPYTHON_LIBS="
CE-PiicoDev-Unified/min/PiicoDev_Unified.py
CE-PiicoDev-VL53L1X-MicroPython-Module/min/PiicoDev_VL53L1X.py
"
THEIR_EXAMPLE_NAME=CE-PiicoDev-VL53L1X-MicroPython-Module/main.py

pushd $RUST_ASSETS_DIR
echo Building ours
cargo build --target thumbv6m-none-eabi --example $OUR_EXAMPLE_NAME
echo Downloading ours to target
probe-rs download --protocol swd --chip RP2040 $OUR_EXAMPLE_NAME
echo Collecting our results
ours="$(probe-rs run --protocol swd --chip RP2040 \
  --disable-progressbars --no-location \
  $OUR_EXAMPLE_NAME 2>/dev/null | head | shuf)"
echo "$ours"
popd

pushd $MICROPYTHON_ASSETS_DIR
echo Downloading MicroPython to target
probe-rs download --protocol swd --chip RP2040 \
  --format bin --base-address 0x10000000 $MICROPYTHON_BIN
echo Resetting target
probe-rs reset --protocol swd --chip RP2040
echo Waiting 1 second for MicroPython to come up
sleep 1
echo Copying MicroPython libraries to target
mpremote cp $MICROPYTHON_LIBS :
echo Collecting their results
theirs="$(mpremote run \
  $THEIR_EXAMPLE_NAME 2>/dev/null | head | shuf)"
echo "$theirs"
popd
```

Because of inherent problems with end-to-end testing of hardware and because I'm interested in conformance to an existing artefact.
Termination
Collecting output
Flashing hardware
Could start with flashing with Python and then flashing with Rust and making a comparison
Could also have a special thing that is running on host that is talking by some other protocol: UART?  To send and receive I2C.
Because both Python and Rust drivers target a HAL, they can be run on so many different combinations of hardware.  We need to be specific which ones we care about.
It'd be nice if we could run the same tests across different targets.

Another issue, how much does it make sense to copy the same interface?  It is necessary for me to make changes. floating point?
Then how can my acceptance tests be useful
Rather than: is this thing the same as the Python thing, is this thing somethign I can use to do the same things as Python thign.  It is fine for my drivers to be diong less and they probably should be doing less.  These acceptance tests are still doing something important (transitively or in an around about way): they give us a loose indication of whether we can get the hardware into the right state.
These tests aren't able to say much without manual intervention: the card sensor, needs something to tap the card.
It probably makes sense to be OK with allowing a human to be involved in these tests.
To be able to run these tests in a likely more reproducible fashion, you could have pieces of hardware testing eachother eg. RGB led and RGB sensor.  This is not without issues.  You could end up with matching issue in RGB led and RGB sensor (what if the channels are swapped).  This kind of machine mistake though is not that different to a human administering the same tests but the human happens to be colour blind.
You could also use a piece of hardware to capture the I2C signals.  Can then look if the right messages are being sent (and that the right timing is happening too)
Nothing stops us from having separate classes of tests, we just need to be clear about them.
Manual vs automatic
Comparison between intended identical implementations vs sane values regardless 
It also means that examples are checked

Let's focus on p7
Describe connections
We get the official source from PiicoDev
How to load it on automatically?
First we got to flash it with uf2
% picotool load mp-pico-w.uf2
% pip3 install mpremote
unified should use package.json for easy installation
mpremote should be fine with pipe being closed
If we're using just OCD we need to use elf
The MicroPython thing for Pico is distributed in uf2.  Need to convert or build from scratch



basically following along from source of bins from the project whos libraries i'm using
decided to switch to latest probe-rs from git (for no_location)
probably try to use bins as much as possible (rather than using the library directly)

we have special kinds of expectations/assumptions for the running of these tests.  for example for the distance sensor, whatever it is sensing in front of it stays at a set distance between running micropython code and running rust code.

minimize cables
simplify code (for demonstration!)

RTT for micropython
getting tests to execute closer together
default i2c addresses should be used
there is more than 1 i2c interface on rp2040 (so collissions should be avoidable)
also more than 1 core (so should be able to run more than 1 test at once)
compare to other kinds of tests already used by community (mention embedded-hal-mock and its usefulness + you have worked on it)
it'd be nice to have rust and micropython installed at the same time (i think it should be possible)

just make it a shell script
