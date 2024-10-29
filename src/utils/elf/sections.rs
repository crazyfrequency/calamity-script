pub struct Section {
    pub name: u32,
    pub s_type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub ent_size: u64
}

impl Section {
    pub fn null() -> Self {
        Self {
            name: 0,
            s_type: 0,
            flags: 0,
            addr: 0,
            offset: 0,
            size: 0,
            link: 0,
            info: 0,
            addr_align: 0,
            ent_size: 0
        }
    }

    pub fn data(size: u64) -> Self {
        Self {
            name: 1,
            s_type: 1,
            flags: 3,
            addr: 0,
            offset: 0x80 + 0x40 * 6,
            size,
            link: 0,
            info: 0,
            addr_align: 4,
            ent_size: 0
        }
    }

    pub fn text(offset: u64, size: u64) -> Self {
        Self {
            name: 7,
            s_type: 1,
            flags: 6,
            addr: 0,
            offset,
            size,
            link: 0,
            info: 0,
            addr_align: 16,
            ent_size: 0
        }
    }

    pub fn shstrtab(offset: u64) -> Self {
        Self {
            name: 0x0d,
            s_type: 3,
            flags: 0,
            addr: 0,
            offset,
            size: 0x32,
            link: 0,
            info: 0,
            addr_align: 1,
            ent_size: 0
        }
    }

    pub fn symtab(offset: u64, size: u64, link: u32, info: u32) -> Self {
        Self {
            name: 0x17,
            s_type: 2,
            flags: 0,
            addr: 0,
            offset,
            size,
            link,
            info,
            addr_align: 8,
            ent_size: 0x18
        }
    }

    pub fn strtab(offset: u64, size: u64) -> Self {
        Self {
            name: 0x1f,
            s_type: 3,
            flags: 0,
            addr: 0,
            offset,
            size,
            link: 0,
            info: 0,
            addr_align: 1,
            ent_size: 0
        }
    }

    pub fn rela_text(offset: u64, size: u64, link: u32, info: u32) -> Self {
        Self {
            name: 0x27,
            s_type: 4,
            flags: 0,
            addr: 0,
            offset,
            size,
            link,
            info,
            addr_align: 8,
            ent_size: 0x18
        }
    }
}

impl Section {
    pub fn to_vec(self) -> Vec<u8> {
        let mut section = Vec::new();

        section.append(&mut self.name.to_le_bytes().to_vec());
        section.append(&mut self.s_type.to_le_bytes().to_vec());
        section.append(&mut self.flags.to_le_bytes().to_vec());
        section.append(&mut self.addr.to_le_bytes().to_vec());
        section.append(&mut self.offset.to_le_bytes().to_vec());
        section.append(&mut self.size.to_le_bytes().to_vec());
        section.append(&mut self.link.to_le_bytes().to_vec());
        section.append(&mut self.info.to_le_bytes().to_vec());
        section.append(&mut self.addr_align.to_le_bytes().to_vec());
        section.append(&mut self.ent_size.to_le_bytes().to_vec());

        section
    }
}