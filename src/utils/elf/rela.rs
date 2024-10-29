pub struct Rela {
    addr: u64,
    info: u64,
    addend: i64
}

impl Rela {
    pub fn x4(offset: u64, id: u64) -> Self {
        Self {
            addr: offset,
            info: 0x10000000b,
            addend: 0x12 + id as i64 * 8
        }
    }

    pub fn x8(offset: u64, id: u64) -> Self {
        Self {
            addr: offset,
            info: 0x100000001,
            addend: 0x12 + id as i64 * 8
        }
    }

    pub fn scanf(offset: u64, index: u64) -> Self {
        Self {
            addr: offset,
            info: 2 + (index << 32),
            addend: -4
        }
    }
    
    pub fn printf(offset: u64, index: u64) -> Self {
        Self {
            addr: offset,
            info: 2 + (index << 32),
            addend: -4
        }
    }

    pub fn format(offset: u64, pos: i64, big: bool) -> Self {
        Self {
            addr: offset,
            info: if big {0x100000001} else {0x10000000b},
            addend: pos
        }
    }
}

impl Rela {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut rela = Vec::new();

        rela.extend(self.addr.to_le_bytes());
        rela.extend(self.info.to_le_bytes());
        rela.extend(self.addend.to_le_bytes());

        rela
    }
}