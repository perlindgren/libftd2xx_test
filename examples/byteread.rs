// use ftdi_mpsse::MpsseSettings;
use libftd2xx::{Ft2232h, Ftdi, FtdiCommon, FtdiMpsse, MpsseSettings};
use std::time::Duration;
use std::{thread, time};

fn main() {
    let mut settings = MpsseSettings::default();

    println!("mpsse {:?}", settings);
    settings.read_timeout = Duration::from_secs(5);

    let mut ft = Ft2232h::with_description("Digilent USB Device A").unwrap();

    ft.initialize_mpsse(&settings);

    let bad_command = [0xAB];
    ft.write_all(&bad_command);

    let mut buf = [0u8; 1024];
    let r = ft.read(&mut buf[..2]).unwrap();

    println!("r {}, buf {:x?}", r, &buf[..r]);
    ft.set_clock(15_000_000).unwrap();

    // pin mode 88 8b (output, direction)
    let (output, direction) = (0x88, 0x8b);
    ft.write_all(&[0x80, output as u8, direction as u8])
        .unwrap();
    ft.write_all(&[0x82, (output >> 8) as u8, (direction >> 8) as u8])
        .unwrap();

    println!("-- reset --");
    reset_to_rti(&mut ft);
    rti_to_shift_dr(&mut ft);
    let idcode = shift_dr(&mut ft);
    println!("idcode {:#010x}", idcode);
    assert_eq!(idcode, 0x0362d093);
    dr_to_rti(&mut ft);

    // println!("read user3 through setting ir");
    // rti_to_shift_ir(&mut ft);
    // shift_ir(&mut ft, 0b10_0010);

    // rti_to_shift_dr(&mut ft);
    // let data = shift_dr(&mut ft);
    // println!("data {:#010x}", data);
    // dr_to_rti(&mut ft);

    let secs = time::Duration::from_secs(1);
    // thread::sleep(secs);

    println!("write user3 through setting ir");
    rti_to_shift_ir(&mut ft);
    shift_ir(&mut ft, 0b10_0010);

    rti_to_shift_dr(&mut ft);
    ft.write(&[
        0x39, 7, 0, 0x1, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x87,
    ])
    .unwrap();
    let mut b8 = [0u8; 8];
    ft.read_all(&mut b8);
    println!("first read {:x?}", b8);
    dr_to_rti(&mut ft);

    rti_to_shift_dr(&mut ft);
    ft.write(&[
        0x39, 7, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x87,
    ])
    .unwrap();
    let mut b8 = [0u8; 8];
    ft.read_all(&mut b8);
    println!("second read {:x?}", b8);
    dr_to_rti(&mut ft);

    rti_to_shift_dr(&mut ft);
    ft.write(&[
        0x39, 7, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x87,
    ])
    .unwrap();
    let mut b8 = [0u8; 8];
    ft.read_all(&mut b8);
    println!("3rd read {:x?}", b8);
    dr_to_rti(&mut ft);

    // rti_to_shift_dr(&mut ft);
    // ft.write(&[0x39, 3, 0, 0xf0, 0xf0, 0xf0, 0xf0, 0x87])
    //     .unwrap();
    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("second read {:x?}", b4);
    // dr_to_rti(&mut ft);

    // rti_to_shift_dr(&mut ft);
    // ft.write(&[0x39, 3, 0, 1, 1, 1, 1, 0x87]).unwrap();
    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("third read {:x?}", b4);
    // dr_to_rti(&mut ft);

    // rti_to_shift_dr(&mut ft);
    // ft.write(&[0x39, 3, 0, 1, 1, 1, 1, 0x87]).unwrap();
    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("fourth read {:x?}", b4);
    // dr_to_rti(&mut ft);

    // thread::sleep(secs);

    // let data = [0x1, 0x2, 0x3, 0x04];
    // ft.write(&[0x39, 2, 0, data[0], data[1], data[2], 0x87])
    //     .unwrap();
    // // ft.write(&[0x1b, 2, 0b01, 0x87]).unwrap();

    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("b4 {:x?}", b4);
    // // thread::sleep(secs);
    // ft.write(&[0x4b, 2, 0b011, 0x87]).unwrap(); // dr_to_rti

    // read_idcode(&mut ft);

    // println!("read/write user3 through setting ir");
    // rti_to_shift_ir(&mut ft);
    // shift_ir(&mut ft, 0b10_0010);

    // rti_to_shift_dr(&mut ft);
    // ft.write(&[0x39, 3, 0, 0, 0, 0, 0, 0x87]).unwrap();
    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("b4 {:x?}", b4);
    // // thread::sleep(secs);
    // dr_to_rti(&mut ft);

    // println!("read/write user3 through setting ir");
    // rti_to_shift_dr(&mut ft);
    // ft.write(&[0x39, 3, 0, 0x11, 0x22, 0x33, 0x44, 0x87])
    //     .unwrap();
    // let mut b4 = [0u8; 4];
    // ft.read_all(&mut b4);
    // println!("b4 {:x?}", b4);
    // dr_to_rti(&mut ft);

    // let data = shift_dr_read_write(&mut ft, 0x11112222);
    // println!("data {:#010x}", data);

    // rti_to_shift_dr(&mut ft);

    // let data = shift_dr_read_write(&mut ft, 0x11112222);
    // println!("data {:#010x}", data);

    // read_idcode(&mut ft);

    // println!("read user3 through setting ir");
    // rti_to_shift_ir(&mut ft);
    // shift_ir(&mut ft, 0b10_0010);

    // rti_to_shift_dr(&mut ft);
    // let data = shift_dr(&mut ft);
    // println!("data {:#010x}", data);
    // dr_to_rti(&mut ft);
}

fn read_idcode(ft: &mut Ft2232h) {
    rti_to_shift_ir(ft);
    shift_ir(ft, 0b00_1001);
    rti_to_shift_dr(ft);
    let idcode = shift_dr(ft);
    println!("idcode {:#010x}", idcode);
    assert_eq!(idcode, 0x0362d093);
    dr_to_rti(ft);
}

// reset state machine, and go to rti
fn reset_to_rti(ft: &mut Ft2232h) {
    ft.write(&[0x4b, 5, 0b11111, 0x87]).unwrap();
    ft.write(&[0x4b, 0, 0b0, 0x87]).unwrap();
}

fn rti_to_shift_dr(ft: &mut Ft2232h) {
    ft.write(&[0x4b, 2, 0b001, 0x87]).unwrap();
}

fn rti_to_shift_ir(ft: &mut Ft2232h) {
    ft.write(&[0x4b, 3, 0b0011, 0x87]).unwrap();
}

fn dr_to_rti(ft: &mut Ft2232h) {
    ft.write(&[0x4b, 2, 0b011, 0x87]).unwrap();
}

fn ir_to_rti(ft: &mut Ft2232h, bit7: u8) {
    ft.write(&[0x4b, 2, bit7 | 0b011, 0x87]).unwrap();
}

fn shift_ir(ft: &mut Ft2232h, ir: u8) {
    // 5 bits of ir
    ft.write(&[0x1b, 4, ir, 0x87]).unwrap();
    // msb of ir as bit 7 of next transaction
    ir_to_rti(ft, (ir & 0b10_0000) << 2);
}

// shift dr
fn shift_dr(ft: &mut Ft2232h) -> u32 {
    let mut buf = [0u8; 1024];
    let mut shift_dr = vec![];
    for _ in 0..8 {
        // shift_dr
        shift_dr.extend_from_slice(&[0x6b, 3, 0b0000]);
    }
    shift_dr.push(0x87);
    ft.write(&shift_dr).unwrap();

    let r = ft.read(&mut buf[..8]).unwrap();

    println!("r {} {:02x?}", r, &buf[..r]);

    buf[..r]
        .iter()
        .rev()
        .fold(0, |acc, d| acc << 4 | (*d >> 4) as u32)
}

// shift dr read write
fn shift_dr_read_write(ft: &mut Ft2232h, mut input: u32) -> u32 {
    let mut buf = [0u8; 4];

    // // data to be shifted out
    // let shift_dr = [
    //     0x39,
    //     3,
    //     0,
    //     input as u8,
    //     (input >> 8) as u8,
    //     (input >> 16) as u8,
    //     (input >> 24) as u8,
    //     0x87,
    // ];
    let shift_dr = [
        0x2c, // clock data bytes in
        3, 0, 0x87,
    ];
    ft.write(&shift_dr).unwrap();
    let r = ft.read(&mut buf).unwrap();
    dr_to_rti(ft);
    println!("r {} {:02x?}", r, buf);
    u32::from_le_bytes(buf)
}
