# tps65185
A WIP `embedded-hal` driver for the TI TPS65185x.

## Power Rails
- CP1: VDDH (+22 V)
- CP2: VEE (-20 V)
- LDO1: VPOS (+15 V)
- LDO2 VNEG (-15 V)

## States
- `SLEEP`: Resets I2C registers and doesn't accept I2C transactions

## References
- Datasheet: http://www.ti.com/lit/ds/symlink/tps65185.pdf
- Product website: http://www.ti.com/product/TPS65185
- Evaluation module: www.ti.com/tool/TPS65185EVM
