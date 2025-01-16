use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x53;
const FIFO_ADDR: u8 = 0x38;
const REG_POWER_CTL: u8 = 0x2D;
const REG_DATAX0: u8 = 0x32;

pub fn get_ids(bus: &mut impl I2c) -> Result<[u8; 2], ()> {
    let mut buf = [0; 1];

    let res = bus.write_read(ADDR, &[0x00], &mut buf);

    if res.is_err() {
        log::error!("{:?}", res);
    }
    
    Ok([0,0])
}

pub fn setup (bus: &mut impl I2c) {
    let mut buf = [0; 1];
    let _ = bus.write_read(ADDR, &[0x38], &mut buf);
    // reset target bits to 0
    buf[0] &= !0b1100_0000;
    buf[0] |= 0b10 << 6;
    let _ = bus.write(ADDR, &[0x38, buf[0]]);
}

pub fn read_acceleration(bus: &mut impl I2c) -> Result<(f32, f32, f32), ()> {
    let mut buf = [0_u8; 6];
    let res = bus.write_read(ADDR, &[REG_DATAX0], &mut buf);
    if res.is_err(){
        log::error!("{:?}", res);
    }
    let x:f32 = i16::from_le_bytes([0, 1]) as f32 * 48.83;
    let y:f32 = i16::from_le_bytes([2, 3]) as f32 * 48.83;
    let z:f32 = i16::from_le_bytes([4, 5]) as f32 * 48.83;
    Ok((x, y, z))
}
