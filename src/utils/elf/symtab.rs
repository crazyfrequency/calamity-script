pub struct Symtab {
    name: u32,
    info: u8,
    other: u8,
    shndx: u16,
    value: u64,
    size: u64,
}

impl Symtab {
    pub fn null() -> Self {
        Self {
            name: 0,
            info: 0,
            other: 0,
            shndx: 0,
            value: 0,
            size: 0,
        }
    }

    pub fn data() -> Self {
        Self {
            name: 0,
            info: 3,
            other: 0,
            shndx: 1,
            value: 0,
            size: 0,
        }
    }

    pub fn text() -> Self {
        Self  {
            name: 0,
            info: 3,
            other: 0,
            shndx: 2,
            value: 0,
            size: 0,
        }
    }

    pub fn main() -> Self {
        Self {
            name: 0x16,
            info: 0x10,
            other: 0,
            shndx: 2,
            value: 0,
            size: 0,
        }
    }

    pub fn scanf() -> Self {
        Self {
            name: 0x1B,
            info: 0x10,
            other: 0,
            shndx: 0,
            value: 0,
            size: 0,
        }
    }

    pub fn printf() -> Self {
        Self {
            name: 0xF,
            info: 0x10,
            other: 0,
            shndx: 0,
            value: 0,
            size: 0,
        }
    }

    pub fn ident(name: u32, value: u64) -> Self {
        Self {
            name,
            info: 0,
            other: 0,
            shndx: 1,
            value,
            size: 0,
        }
    }
}

impl Symtab {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut symtab = Vec::new();

        symtab.append(&mut self.name.to_le_bytes().to_vec());
        symtab.append(&mut self.info.to_le_bytes().to_vec());
        symtab.append(&mut self.other.to_le_bytes().to_vec());
        symtab.append(&mut self.shndx.to_le_bytes().to_vec());
        symtab.append(&mut self.value.to_le_bytes().to_vec());
        symtab.append(&mut self.size.to_le_bytes().to_vec());

        symtab
    }
}