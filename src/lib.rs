use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x53;
const _FIFO_ADDR: u8 = 0x38;
const _REG_POWER_CTL: u8 = 0x2D;
const REG_DATAX0: u8 = 0x32;

pub fn get_ids(bus: &mut impl I2c) -> Result<[u8; 2], i8> {
    let mut buf = [0; 2]; // Expecting 2 bytes from the device

    // Try to read from register 0x00
    if let Err(_e) = bus.write_read(ADDR, &[0x00], &mut buf) {
        return Err(-1); // Return the error if communication fails
    }

    Ok(buf) // Successfully read 2 bytes, return them
}


pub fn setup (bus: &mut impl I2c) {
    let mut buf = [0; 1];
    let _ = bus.write_read(ADDR, &[0x38], &mut buf).unwrap();
    // reset target bits to 0
    buf[0] &= !0b1100_0000;
    buf[0] |= 0b10 << 6;
    let _ = bus.write(ADDR, &[0x38, buf[0]]);
}

pub fn read_acceleration(bus: &mut impl I2c) -> Result<(f32, f32, f32), i8> {
    let mut buf = [0_u8; 6];
    
    if let Err(_e) = bus.write_read(ADDR, &[REG_DATAX0], &mut buf) {
        return Err(-1);
    }

    let x: f32 = i16::from_le_bytes([buf[0], buf[1]]) as f32 * 48.83;
    let y: f32 = i16::from_le_bytes([buf[2], buf[3]]) as f32 * 48.83;
    let z: f32 = i16::from_le_bytes([buf[4], buf[5]]) as f32 * 48.83;

    Ok((x, y, z))
}
