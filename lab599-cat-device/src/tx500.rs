use lab599_cat_core::{
    CatError, Command, IfStatus, MemoryData, MeterType, Mode, Protocol, Response, VfoSelect,
};

/// Driver for the Lab599 TX-500 transceiver.
///
/// Generic over any `Read + Write` transport (serial port, mock cursor, etc.).
///
/// Feature flags:
/// - `tx500`   — enables this module and TX500-specific commands (BD, BU, SP, VV, XT).
/// - `tx500mp` — implies `tx500`; adds TX500MP-specific commands (AC, CT).
pub struct Tx500<T>
where
    T: std::io::Read + std::io::Write,
{
    io: T,
}

impl<T: std::io::Read + std::io::Write> Tx500<T> {
    pub fn new(io: T) -> Self {
        Self { io }
    }

    // ── internal helpers ──────────────────────────────────────────────────

    fn send(&mut self, cmd: &Command) -> Result<(), CatError> {
        let raw = Protocol::encode(cmd);
        self.io
            .write_all(raw.as_bytes())
            .map_err(|e| CatError::DeviceError(e.to_string()))
    }

    fn recv(&mut self) -> Result<Response, CatError> {
        // Read byte-by-byte until ';' terminator to handle partial reads.
        let mut buf = Vec::with_capacity(64);
        let mut byte = [0u8; 1];
        loop {
            match self.io.read(&mut byte) {
                Ok(0) => break,
                Ok(_) => {
                    buf.push(byte[0]);
                    if byte[0] == b';' {
                        break;
                    }
                    if buf.len() > 256 {
                        return Err(CatError::DeviceError("response too long".into()));
                    }
                }
                Err(e) => return Err(CatError::DeviceError(e.to_string())),
            }
        }
        let s = std::str::from_utf8(&buf)
            .map_err(|e| CatError::DeviceError(e.to_string()))?;
        Protocol::decode(s)
    }

    fn send_recv(&mut self, cmd: &Command) -> Result<Response, CatError> {
        self.send(cmd)?;
        self.recv()
    }

    // ── VFO / Frequency ───────────────────────────────────────────────────

    pub fn set_frequency_a(&mut self, freq: u64) -> Result<(), CatError> {
        self.send(&Command::FaSet(freq))
    }

    pub fn get_frequency_a(&mut self) -> Result<u64, CatError> {
        match self.send_recv(&Command::FaRead)? {
            Response::Fa(f) => Ok(f),
            _ => Err(CatError::UnknownResponse("FA".into())),
        }
    }

    pub fn set_frequency_b(&mut self, freq: u64) -> Result<(), CatError> {
        self.send(&Command::FbSet(freq))
    }

    pub fn get_frequency_b(&mut self) -> Result<u64, CatError> {
        match self.send_recv(&Command::FbRead)? {
            Response::Fb(f) => Ok(f),
            _ => Err(CatError::UnknownResponse("FB".into())),
        }
    }

    pub fn set_vfo_rx(&mut self, sel: VfoSelect) -> Result<(), CatError> {
        self.send(&Command::FrSet(sel))
    }

    pub fn get_vfo_rx(&mut self) -> Result<VfoSelect, CatError> {
        match self.send_recv(&Command::FrRead)? {
            Response::Fr(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("FR".into())),
        }
    }

    pub fn set_vfo_tx(&mut self, sel: VfoSelect) -> Result<(), CatError> {
        self.send(&Command::FtSet(sel))
    }

    pub fn get_vfo_tx(&mut self) -> Result<VfoSelect, CatError> {
        match self.send_recv(&Command::FtRead)? {
            Response::Ft(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("FT".into())),
        }
    }

    // ── Mode ─────────────────────────────────────────────────────────────

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), CatError> {
        self.send(&Command::MdSet(mode))
    }

    pub fn get_mode(&mut self) -> Result<Mode, CatError> {
        match self.send_recv(&Command::MdRead)? {
            Response::Md(m) => Ok(m),
            _ => Err(CatError::UnknownResponse("MD".into())),
        }
    }

    // ── PTT / RX / TX ─────────────────────────────────────────────────────

    pub fn set_rx(&mut self) -> Result<(), CatError> {
        self.send(&Command::RxMode)
    }

    pub fn set_tx(&mut self) -> Result<(), CatError> {
        self.send(&Command::TxMode)
    }

    pub fn get_ptt(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::PtRead)? {
            Response::Pt(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("PT".into())),
        }
    }

    // ── Full status ───────────────────────────────────────────────────────

    pub fn get_status(&mut self) -> Result<IfStatus, CatError> {
        match self.send_recv(&Command::IfRead)? {
            Response::If(s) => Ok(s),
            _ => Err(CatError::UnknownResponse("IF".into())),
        }
    }

    // ── AF / RF ───────────────────────────────────────────────────────────

    pub fn set_af_gain(&mut self, value: u16) -> Result<(), CatError> {
        self.send(&Command::AgSet(value))
    }

    pub fn get_af_gain(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::AgRead)? {
            Response::Ag(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("AG".into())),
        }
    }

    pub fn set_rf_gain(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::RgSet(value))
    }

    pub fn get_rf_gain(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::RgRead)? {
            Response::Rg(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("RG".into())),
        }
    }

    pub fn set_preamp(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::PaSet(on))
    }

    pub fn get_preamp(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::PaRead)? {
            Response::Pa(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("PA".into())),
        }
    }

    pub fn set_attenuator(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::RaSet(on))
    }

    pub fn get_attenuator(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::RaRead)? {
            Response::Ra(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("RA".into())),
        }
    }

    // ── S-meter / Power ───────────────────────────────────────────────────

    pub fn get_smeter(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::SmRead)? {
            Response::Sm(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("SM".into())),
        }
    }

    pub fn set_power(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::PcSet(value))
    }

    pub fn get_power(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::PcRead)? {
            Response::Pc(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("PC".into())),
        }
    }

    // ── Noise / Filter ────────────────────────────────────────────────────

    pub fn set_noise_blanker(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::NbSet(on))
    }

    pub fn get_noise_blanker(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::NbRead)? {
            Response::Nb(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("NB".into())),
        }
    }

    pub fn set_noise_blanker_level(&mut self, level: u8) -> Result<(), CatError> {
        self.send(&Command::NlSet(level))
    }

    pub fn get_noise_blanker_level(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::NlRead)? {
            Response::Nl(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("NL".into())),
        }
    }

    pub fn set_noise_reduction(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::NrSet(on))
    }

    pub fn get_noise_reduction(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::NrRead)? {
            Response::Nr(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("NR".into())),
        }
    }

    pub fn set_noise_reduction_level(&mut self, level: u8) -> Result<(), CatError> {
        self.send(&Command::RlSet(level))
    }

    pub fn get_noise_reduction_level(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::RlRead)? {
            Response::Rl(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("RL".into())),
        }
    }

    pub fn set_notch(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::NtSet(on))
    }

    pub fn get_notch(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::NtRead)? {
            Response::Nt(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("NT".into())),
        }
    }

    pub fn set_filter(&mut self, rx: u8, tx: u8) -> Result<(), CatError> {
        self.send(&Command::FlSet { rx, tx })
    }

    pub fn get_filter(&mut self) -> Result<(u8, u8), CatError> {
        match self.send_recv(&Command::FlRead)? {
            Response::Fl { rx, tx } => Ok((rx, tx)),
            _ => Err(CatError::UnknownResponse("FL".into())),
        }
    }

    // ── RIT / XIT ─────────────────────────────────────────────────────────

    pub fn set_rit(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::RtSet(on))
    }

    pub fn get_rit(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::RtRead)? {
            Response::Rt(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("RT".into())),
        }
    }

    // ── AGC ───────────────────────────────────────────────────────────────

    pub fn set_agc(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::GtSet(value))
    }

    pub fn get_agc(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::GtRead)? {
            Response::Gt(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("GT".into())),
        }
    }

    // ── VOX ───────────────────────────────────────────────────────────────

    pub fn set_vox(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::VxSet(on))
    }

    pub fn get_vox(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::VxRead)? {
            Response::Vx(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("VX".into())),
        }
    }

    pub fn set_vox_gain(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::VgSet(value))
    }

    pub fn get_vox_gain(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::VgRead)? {
            Response::Vg(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("VG".into())),
        }
    }

    pub fn set_vox_delay(&mut self, ms: u16) -> Result<(), CatError> {
        self.send(&Command::VdSet(ms))
    }

    pub fn get_vox_delay(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::VdRead)? {
            Response::Vd(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("VD".into())),
        }
    }

    // ── Memory ────────────────────────────────────────────────────────────

    pub fn set_memory_channel(&mut self, channel: u8) -> Result<(), CatError> {
        self.send(&Command::McSet(channel))
    }

    pub fn get_memory_channel(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::McRead)? {
            Response::Mc(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("MC".into())),
        }
    }

    pub fn read_memory(&mut self, channel: u8) -> Result<MemoryData, CatError> {
        match self.send_recv(&Command::MrRead(channel))? {
            Response::Mr(d) => Ok(d),
            _ => Err(CatError::UnknownResponse("MR".into())),
        }
    }

    pub fn write_memory(&mut self, data: MemoryData) -> Result<(), CatError> {
        self.send(&Command::MwWrite(data))
    }

    // ── Meter ─────────────────────────────────────────────────────────────

    pub fn get_meter(&mut self, meter: MeterType) -> Result<u16, CatError> {
        self.send(&Command::RmSet(meter))?;
        match self.recv()? {
            Response::Rm { value, .. } => Ok(value),
            _ => Err(CatError::UnknownResponse("RM".into())),
        }
    }

    // ── Lock ─────────────────────────────────────────────────────────────

    pub fn set_lock(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::LkSet(on))
    }

    pub fn get_lock(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::LkRead)? {
            Response::Lk(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("LK".into())),
        }
    }

    // ── Squelch ───────────────────────────────────────────────────────────

    pub fn set_squelch(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::SqSet(value))
    }

    pub fn get_squelch(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::SqRead)? {
            Response::Sq(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("SQ".into())),
        }
    }

    // ── Misc ──────────────────────────────────────────────────────────────

    pub fn get_id(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::IdRead)? {
            Response::Id(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("ID".into())),
        }
    }

    pub fn get_busy(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::ByRead)? {
            Response::By { busy } => Ok(busy),
            _ => Err(CatError::UnknownResponse("BY".into())),
        }
    }

    pub fn get_voltage(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::VlRead)? {
            Response::Vl(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("VL".into())),
        }
    }

    pub fn set_power_on(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::PsSet(on))
    }

    pub fn set_mic_gain(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::MgSet(value))
    }

    pub fn get_mic_gain(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::MgRead)? {
            Response::Mg(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("MG".into())),
        }
    }

    pub fn set_keying_speed(&mut self, wpm: u8) -> Result<(), CatError> {
        self.send(&Command::KsSet(wpm))
    }

    pub fn get_keying_speed(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::KsRead)? {
            Response::Ks(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("KS".into())),
        }
    }

    pub fn set_carrier_level(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::CgSet(value))
    }

    pub fn get_carrier_level(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::CgRead)? {
            Response::Cg(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("CG".into())),
        }
    }

    pub fn set_monitor_level(&mut self, value: u16) -> Result<(), CatError> {
        self.send(&Command::MlSet(value))
    }

    pub fn get_monitor_level(&mut self) -> Result<u16, CatError> {
        match self.send_recv(&Command::MlRead)? {
            Response::Ml(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("ML".into())),
        }
    }

    pub fn set_monitor_mute(&mut self, mute: bool) -> Result<(), CatError> {
        self.send(&Command::MoSet(mute))
    }

    pub fn set_speech_compressor(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::PrSet(on))
    }

    pub fn set_compressor_level(&mut self, level: u8) -> Result<(), CatError> {
        self.send(&Command::PlSet(level))
    }

    pub fn set_dsp_if(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::IsSet(on))
    }

    pub fn set_dig_gain(&mut self, value: u8) -> Result<(), CatError> {
        self.send(&Command::MaSet(value))
    }

    pub fn get_dig_gain(&mut self) -> Result<u8, CatError> {
        match self.send_recv(&Command::MaRead)? {
            Response::Ma(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("MA".into())),
        }
    }

    // ── TX500-specific ────────────────────────────────────────────────────

    /// Move to the next band down.
    pub fn band_down(&mut self) -> Result<(), CatError> {
        self.send(&Command::BandDown)
    }

    /// Move to the next band up.
    pub fn band_up(&mut self) -> Result<(), CatError> {
        self.send(&Command::BandUp)
    }

    /// Set split operation (VFO B for TX).
    pub fn set_split(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::SpSet(on))
    }

    pub fn get_split(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::SpRead)? {
            Response::Sp(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("SP".into())),
        }
    }

    /// Copy VFO A frequency to VFO B.
    pub fn vfo_copy_ab(&mut self) -> Result<(), CatError> {
        self.send(&Command::VfoCopyAb)
    }

    pub fn set_xit(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::XtSet(on))
    }

    pub fn get_xit(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::XtRead)? {
            Response::Xt(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("XT".into())),
        }
    }

    // ── TX500MP-specific ──────────────────────────────────────────────────

    /// Set antenna tuner state (TX500MP only).
    #[cfg(feature = "tx500mp")]
    pub fn set_antenna_tuner(&mut self, at_on: bool, start_tuning: bool) -> Result<(), CatError> {
        self.send(&Command::AcSet { at_on, start_tuning })
    }

    /// Read antenna tuner state (TX500MP only).
    #[cfg(feature = "tx500mp")]
    pub fn get_antenna_tuner(&mut self) -> Result<(bool, bool), CatError> {
        match self.send_recv(&Command::AcRead)? {
            Response::Ac { at_on, tuning } => Ok((at_on, tuning)),
            _ => Err(CatError::UnknownResponse("AC".into())),
        }
    }

    /// Set CTCSS function on/off (TX500MP only).
    #[cfg(feature = "tx500mp")]
    pub fn set_ctcss(&mut self, on: bool) -> Result<(), CatError> {
        self.send(&Command::CtSet(on))
    }

    /// Read CTCSS function status (TX500MP only).
    #[cfg(feature = "tx500mp")]
    pub fn get_ctcss(&mut self) -> Result<bool, CatError> {
        match self.send_recv(&Command::CtRead)? {
            Response::Ct(v) => Ok(v),
            _ => Err(CatError::UnknownResponse("CT".into())),
        }
    }
}

// ── Integration tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock I/O with separate read (response) and write (command) buffers.
    struct MockIo {
        rx: std::io::Cursor<Vec<u8>>,
        pub tx: Vec<u8>,
    }

    impl MockIo {
        fn with_response(data: &[u8]) -> Self {
            MockIo { rx: std::io::Cursor::new(data.to_vec()), tx: Vec::new() }
        }

        fn write_only() -> Self {
            MockIo { rx: std::io::Cursor::new(vec![]), tx: Vec::new() }
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
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }

    #[test]
    fn get_frequency_a() {
        let mut dev = Tx500::new(MockIo::with_response(b"FA00007000000;"));
        assert_eq!(dev.get_frequency_a().unwrap(), 7_000_000);
    }

    #[test]
    fn get_frequency_b() {
        let mut dev = Tx500::new(MockIo::with_response(b"FB00014195000;"));
        assert_eq!(dev.get_frequency_b().unwrap(), 14_195_000);
    }

    #[test]
    fn get_mode_usb() {
        let mut dev = Tx500::new(MockIo::with_response(b"MD2;"));
        assert_eq!(dev.get_mode().unwrap(), Mode::Usb);
    }

    #[test]
    fn set_frequency_encodes_correctly() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.set_frequency_a(14_225_000).unwrap();
        assert_eq!(&dev.io.tx, b"FA00014225000;");
    }

    #[test]
    fn set_mode_encodes_correctly() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.set_mode(Mode::Lsb).unwrap();
        assert_eq!(&dev.io.tx, b"MD1;");
    }

    #[test]
    fn get_smeter() {
        let mut dev = Tx500::new(MockIo::with_response(b"SM00015;"));
        assert_eq!(dev.get_smeter().unwrap(), 15);
    }

    #[test]
    fn get_id() {
        let mut dev = Tx500::new(MockIo::with_response(b"ID500;"));
        assert_eq!(dev.get_id().unwrap(), 500);
    }

    #[test]
    fn get_busy_false() {
        let mut dev = Tx500::new(MockIo::with_response(b"BY00;"));
        assert!(!dev.get_busy().unwrap());
    }

    #[test]
    fn get_voltage() {
        let mut dev = Tx500::new(MockIo::with_response(b"VL1350;"));
        assert_eq!(dev.get_voltage().unwrap(), 1350);
    }

    #[test]
    fn band_down_encodes_correctly() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.band_down().unwrap();
        assert_eq!(&dev.io.tx, b"BD;");
    }

    #[test]
    fn vfo_copy_ab_encodes_correctly() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.vfo_copy_ab().unwrap();
        assert_eq!(&dev.io.tx, b"VV;");
    }

    #[test]
    fn set_split_on() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.set_split(true).unwrap();
        assert_eq!(&dev.io.tx, b"SP1;");
    }

    #[cfg(feature = "tx500mp")]
    #[test]
    fn set_antenna_tuner_encodes_correctly() {
        let mut dev = Tx500::new(MockIo::write_only());
        dev.set_antenna_tuner(true, false).unwrap();
        assert_eq!(&dev.io.tx, b"AC010;");
    }
}
