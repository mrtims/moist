moist
===============

_moist_ is a soil moisture sensor built on the [STM32F429I-DISC][] discovery board.
Written in Rust in an attempt to get a feel for the language and ecosystem,
while attempting to keep my chillies healthy at the same time.

Status
-------
Often getting things up and running, with up-to-date dependencies is the hardest part. So if you're reading this looking for a recently updated starting point for the STM32F429I Disco, this should hopefully be a good start. I'll endeavour to periodically keep it up-to-date with the latest combination of interoperable dependencies.

Note that the standard 
```cargo build``` and ```cargo run``` will fail as cargo will attempt to build the STM32F4 target for Windows.
Instead, there are aliases provided in the .cargo/config.toml to build, run, and test parts of this package with the correct target.
This could be addressed but only by depending on unstable cargo features, which I have chosen not to do.

DIY - Simulation
-------
You can build and run a simulated version by running
```bash
cargo run-simulation
```
This requires SDL2 for the [embedded-graphics-simulator][] which can be installed following the instructions at that link, according to your operating system.

DIY - On Target
-------
To run this yourself, you will need to install a few things, which you can get by following the instructions
in the [Embedded Rust Book][]

Then, running it should be as simple as plugging it in, and then running in two command line windows, both in the root directory:
```bash
openocd
```
in one window, and
```bash
cargo run-stm32f429
```
in the other. ```--release``` may also be added to load the release binary instead, which results in a major speedup for drawing to the LCD.

[cortex-debug][] is also set up for VSCode, so it can be debugged out of the box. I haven't hooked up ITM so there will be no stdout, and panics will just freeze the screen.


[STM32F429I-DISC]: https://www.st.com/en/evaluation-tools/32f429idiscovery.html
[embedded-graphics-simulator]: https://crates.io/crates/embedded-graphics-simulator
[Embedded Rust Book]: https://docs.rust-embedded.org/discovery/03-setup/index.html
[cortex-debug]: https://github.com/Marus/cortex-debug

License
-------

[MIT license](LICENSE-MIT.txt).