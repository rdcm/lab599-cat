use lab599_cat::{CatDriver, MeterType, Mode};

/// Mock I/O with separate read (response) and write (command) buffers.
struct MockIo {
    rx: std::io::Cursor<Vec<u8>>,
    pub tx: Vec<u8>,
}

impl MockIo {
    fn with_response(data: &[u8]) -> Self {
        MockIo {
            rx: std::io::Cursor::new(data.to_vec()),
            tx: Vec::new(),
        }
    }

    fn write_only() -> Self {
        MockIo {
            rx: std::io::Cursor::new(vec![]),
            tx: Vec::new(),
        }
    }
}

impl std::io::Read for MockIo {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rx.read(buf)
    }
}

impl std::io::Write for MockIo {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.tx.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn get_frequency_a() {
    let mut dev = CatDriver::new(MockIo::with_response(b"FA00007000000;"));
    assert_eq!(dev.get_frequency_a().unwrap(), 7_000_000);
}

#[test]
fn get_frequency_b() {
    let mut dev = CatDriver::new(MockIo::with_response(b"FB00014195000;"));
    assert_eq!(dev.get_frequency_b().unwrap(), 14_195_000);
}

#[test]
fn get_mode_usb() {
    let mut dev = CatDriver::new(MockIo::with_response(b"MD2;"));
    assert_eq!(dev.get_mode().unwrap(), Mode::Usb);
}

#[test]
fn set_frequency_encodes_correctly() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.set_frequency_a(14_225_000).unwrap();
    assert_eq!(&dev.into_inner().tx, b"FA00014225000;");
}

#[test]
fn set_mode_encodes_correctly() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.set_mode(Mode::Lsb).unwrap();
    assert_eq!(&dev.into_inner().tx, b"MD1;");
}

#[test]
fn get_smeter() {
    let mut dev = CatDriver::new(MockIo::with_response(b"SM00015;"));
    assert_eq!(dev.get_smeter().unwrap(), 15);
}

#[test]
fn get_id() {
    let mut dev = CatDriver::new(MockIo::with_response(b"ID500;"));
    assert_eq!(dev.get_id().unwrap(), 500);
}

#[test]
fn get_busy_false() {
    let mut dev = CatDriver::new(MockIo::with_response(b"BY00;"));
    assert!(!dev.get_busy().unwrap());
}

#[test]
fn get_voltage() {
    let mut dev = CatDriver::new(MockIo::with_response(b"VL11.7 ;"));
    assert_eq!(dev.get_voltage().unwrap(), 117);
}

#[test]
fn band_down_encodes_correctly() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.band_down().unwrap();
    assert_eq!(&dev.into_inner().tx, b"BD;");
}

#[test]
fn vfo_copy_ab_encodes_correctly() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.vfo_copy_ab().unwrap();
    assert_eq!(&dev.into_inner().tx, b"VV;");
}

#[test]
fn set_split_on() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.set_split(true).unwrap();
    assert_eq!(&dev.into_inner().tx, b"SP1;");
}

#[test]
fn get_meter_swr() {
    // RM1; selects SWR meter (no response), RM; reads value → RM10015;
    let mut dev = CatDriver::new(MockIo::with_response(b"RM10015;"));
    assert_eq!(dev.get_meter(MeterType::Swr).unwrap(), 15);
    assert_eq!(&dev.into_inner().tx, b"RM1;RM;");
}

#[cfg(feature = "tx500mp")]
#[test]
fn set_antenna_tuner_encodes_correctly() {
    let mut dev = CatDriver::new(MockIo::write_only());
    dev.set_antenna_tuner(true, false).unwrap();
    assert_eq!(&dev.into_inner().tx, b"AC010;");
}
