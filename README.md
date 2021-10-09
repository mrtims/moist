moist
===============

_moist_ is a soil moisture sensor built on the [STM32F429I-DISC][] discovery board.
Written in Rust in an attempt to get a feel for the language and ecosystem,
while attempting to keep my chillies healthy at the same time.

Status
-------
Often getting things up and running, with up-to-date dependencies is the hardest part. So if you're reading this looking for a recently updated starting point for the STM32F429I Disco, this should hopefully be a good start. I'll endeavour to periodically keep it up-to-date with the latest combination of interoperable dependencies.

DIY
-------
To run this yourself, you will need to install a few things, which you can get by following the instructions
in the [Embedded Rust Book][]

Then, running it should be as simple as plugging it in, and then running in two windows, both in the root directory:
```bash
openocd
```
in one window, and
```bash
cargo run --release
```
in the other.

[cortex-debug][] is also set up for VSCode, so it can be debugged out of the box. I haven't hooked up ITM so there will be no stdout, and panics will just freeze the screen.


[STM32F429I-DISC]: https://www.st.com/en/evaluation-tools/32f429idiscovery.html
[Embedded Rust Book]: https://docs.rust-embedded.org/discovery/03-setup/index.html
[cortex-debug]: https://github.com/Marus/cortex-debug

License
-------

[MIT license](LICENSE-MIT.txt).