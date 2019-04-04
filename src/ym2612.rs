//const MAX_UPDATE: usize = 0x100;

pub struct YM2612 {
    st_address: u16,
    dac_out: u8,
    dac_en: bool,
    channels: [Channel; 6],
    detune_table: DetuneTable,
    /*
    clock: i16,
    rate: i16,
    timer_base: i16,
    status: i16,
    opn_a_addr: i16,
    opn_n_addr: i16,
    lfo_cnt: i16,
    lfc_int: i16,

    timer_a: i16,
    timer_al: i16,
    timer_a_cnd: i16,
    timer_b: i16,
    timer_bl: i16,
    timer_b_cnt: i16,
    mode: i16,
    dac: i16,
    dac_data: i16,
    dac_highpass: i32,
    frequence: f32,
    inter_cnt: u16,
    inter_step: u16,
    registers: [[i16; 0x100]; 2],

    lfo_env_up: [i16; MAX_UPDATE],
    lfo_freq_up: [i16; MAX_UPDATE],

    in0: i16,
    in1: i16,
    in2: i16,
    in3: i16,
    en0: i16,
    en1: i16,
    en2: i16,
    en3: i16,

    dac_mute: i16, // NOTE: 6 bits used for muting channels
    */
}

struct Channel {
    slots: [Slot; 4],
    /*
    s0_out: [i16; 4],
    old_outd: i16,
    outd: i16,
    left: i16,
    right: i16,
    algo: i16,
    fb: i16,
    fms: i16,
    ams: i16,
    fnum: [i16; 4],
    foct: [i16; 4],
    kc: [i16; 4],
    f_flag: i16,
    mute: i16,
    */
}

struct Slot {
    rate_mul: u32,
    dt: Option<usize>,
    phase: u32,
    incr: Option<u32>,
    tl: u32,
    /*
    mul: i16,
    tll: i16,
    sll: i16,
    ksr_s: i16,
    ksr: i16,
    seg: i16,
    ar: i16,
    dr: i16,
    sr: i16,
    rr: i16,
    fcnt: i16,
    finc: i16,
    ecurp: i16,
    ecnt: i16,
    einc: i16,
    ecmp: i16,
    einca: i16,
    eincd: i16,
    eincs: i16,
    eincr: i16,
    oupt: i16,
    ind: i16,
    chgenm: i16,
    ams: i16,
    amson: i16,
    */
}

struct DetuneTable([[i32; 32]; 8]);

impl YM2612 {
    pub fn new() -> YM2612 {
        YM2612 {
            st_address: 0,
            dac_out: 0,
            dac_en: false,
            channels: [Channel::new(), Channel::new(), Channel::new(), Channel::new(), Channel::new(), Channel::new()],
            detune_table: DetuneTable([[0; 32]; 8]),
        }
    }

    pub fn write(&mut self, address: u8, data: u8) {
        self.st_address = address as u16 & 0x1ff;

        //let addr = self.st_address & 0x1f0;
        match self.st_address {
            0x20..0x2f => match self.st_address {
                0x2a => self.dac_out = ((data - 0x80) | 0) << 6,
                0x2b => self.dac_en = data & 0x80 == 1,
                _ => self.write_mode(data),
            },
            _ => self.write_reg(data),
        }
    }

    fn write_reg(&mut self, data: u8) {
        let channel = self.st_address & 0x3;
        let slot = (self.st_address >> 2) & 0x3;

        match self.st_address & 0xf0 {
            0x30 => {},
            0x40 => {},
            0x50 => {},
            0x60 => {},
            0x70 => {},
            0x80 => {},
            0x90 => {},
            0xa0 => {},
            0xb0 => {},
            _ => panic!("invalid value: 0x{:02x}", data),
        }
    }

    fn write_mode(&mut self, data: u8) {
        match self.st_address {
            0x21 => {},
            0x22 => {},
            0x24 => {},
            0x25 => {},
            0x26 => {},
            0x27 => {},
            0x28 => {},
            _ => panic!("invalid value: 0x{:02x}", data),
        }
    }
}

impl Channel {
    fn new() -> Channel {
        Channel {
            slots: [Slot::new(), Slot::new(), Slot::new(), Slot::new()],
            /*
            s0_out: [0; 4],
            old_outd: 0,
            outd: 0,
            left: 0,
            right: 0,
            algo: 0,
            fb: 0,
            fms: 0,
            ams: 0,
            fnum: [0; 4],
            foct: [0; 4],
            kc: [0; 4],
            f_flag: 0,
            mute: 0,
            */
        }
    }

    fn set_det_mul(&mut self, slot: usize, data: u8) {
        self.slots[slot].set_det_mul(data);
        self.slots[0].incr = None;
    }

    fn set_tl(&mut self, slot: usize, data: u8) {
        self.slots[slot].set_tl(data);
    }
}

impl Slot {
    fn new() -> Slot {
        Slot {
            rate_mul: 1,
            dt: None,
            phase: 0,
            incr: Some(0),
            tl: 0,
            /*
            mul: 0,
            tll: 0,
            sll: 0,
            ksr_s: 0,
            ksr: 0,
            seg: 0,
            ar: 0,
            dr: 0,
            sr: 0,
            rr: 0,
            fcnt: 0,
            finc: 0,
            ecurp: 0,
            ecnt: 0,
            einc: 0,
            ecmp: 0,
            einca: 0,
            eincd: 0,
            eincs: 0,
            eincr: 0,
            oupt: 0,
            ind: 0,
            chgenm: 0,
            ams: 0,
            amson: 0,
            */
        }
    }

    fn set_det_mul(&mut self, data: u8) {
        self.rate_mul = if (data & 0xf) > 0 {
            (data as u32 & 0xf) << 1
        } else {
            1
        };
        self.dt = Some((data as usize >> 4) & 0x7);
    }

    fn set_tl(&mut self, data: u8) {
        self.tl = (data as u32 & 0x7f) << (10 - 7);
    }
}
