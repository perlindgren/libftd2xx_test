# libftd2xx_test

Test of the libftd2xx library for communicating with a Digilent ARTY board.

The example reads and writes a custom USER TAP.

- `examples/open.rs`, opens the first found `ftdi` device.
- `examples/mpsse.rs`, low-level messaging to control the JTAG fsm:

  - reset
  - read IDCODE
  - read IDCODE register using IR selected register 0x09
  - read USER3 register using IR selected register 0x22
  - writes 32 bit data to USER3
  - reads back written value
