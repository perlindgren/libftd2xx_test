use libftd2xx::{DeviceTypeError, Ftdi, FtdiCommon};

fn main() {
    let mut ft = Ftdi::new().unwrap();
    let info = ft.device_info().unwrap();
    println!("Device information: {:?}", info);
}
