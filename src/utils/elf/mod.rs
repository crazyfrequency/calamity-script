use std::{fs::{self, File}, io::{Error, Seek, SeekFrom, Write}, path::Path};

use rela::Rela;
use sections::Section;
use symtab::Symtab;

mod sections;
mod symtab;
mod rela;

pub struct Elf {
    res_file: String,
    position: u64,
    ident_count: u16,
    ident_name_offset: Vec<u32>,
    program: Vec<u8>,
    asm_idents: Vec<(u64, u64, bool)>
}

impl Elf {
    pub fn new(res_file: impl Into<String>, ident_count: u16, program: Vec<u8>, asm_idents: Vec<(u64, u64, bool)>) -> Self {
        Self {
            res_file: res_file.into(),
            position: 0,
            ident_count: ident_count.into(),
            ident_name_offset: Vec::new(),
            program,
            asm_idents
        }
    }

    pub fn process(&mut self) -> Result<(), Error> {
        if Path::new(&self.res_file).exists() {
            fs::remove_file(self.res_file.clone())?;
        }
        let mut file = File::create(self.res_file.clone())?;
        let header = self.get_header();
        let header = header.as_bytes();
        file.write(header)?;
        // смещение на позицию после заголовков секций
        self.position = 0x80 + 0x40 * 6;
        file.seek(SeekFrom::Start(self.position))?;

        let (size, data) = self.get_data();
        let data = data.as_slice();
        file.write(data)?;
        self.position = file.stream_position()?;
        
        file.seek(SeekFrom::Start(0x80))?;
        file.write(&Section::data(size).to_vec())?;

        file.seek(SeekFrom::Start(0x40 + 0x40 * 2))?;
        file.write(&Section::text(self.position, self.program.len() as u64).to_vec())?;

        file.seek(SeekFrom::Start(self.position))?;
        file.write(self.program.as_slice())?;
        self.position = file.stream_position()?;

        for _ in 0..(16 - self.position % 16) {
            file.write(&[0x00])?;
        }
        self.position = file.stream_position()?;

        file.seek(SeekFrom::Start(0x40 + 0x40 * 3))?;
        file.write(&Section::shstrtab(self.position).to_vec())?;
        file.seek(SeekFrom::Start(self.position))?;
        file.write(include_bytes!("./data.bin"))?;
        self.position = file.stream_position()?;

        let strtab = self.get_strtab();

        file.seek(SeekFrom::Start(0x40 + 0x40 * 4))?;
        let (info, symtab) = self.get_symtab();
        file.write(&Section::symtab(self.position, symtab.len() as u64, 5, info).to_vec())?;
        
        file.seek(SeekFrom::Start(self.position))?;
        file.write(symtab.as_slice())?;
        self.position = file.stream_position()?;

        for _ in 0..(16 - self.position % 16) {
            file.write(&[0x00])?;
        }
        self.position = file.stream_position()?;

        file.seek(SeekFrom::Start(0x40 + 0x40 * 5))?;
        file.write(&Section::strtab(self.position, strtab.len() as u64).to_vec())?;

        file.seek(SeekFrom::Start(self.position))?;
        file.write(strtab.as_slice())?;
        self.position = file.stream_position()?;

        for _ in 0..(16 - self.position % 16) {
            file.write(&[0x00])?;
        }
        self.position = file.stream_position()?;

        let rela_text = self.get_rela_text();

        file.seek(SeekFrom::Start(0x40 + 0x40 * 6))?;
        file.write(&Section::rela_text(self.position, rela_text.len() as u64, 4, 2).to_vec())?;

        file.seek(SeekFrom::Start(self.position))?;
        file.write(rela_text.as_slice())?;
        self.position = file.stream_position()?;

        for _ in 0..(16 - self.position % 16) {
            file.write(&[0x00])?;
        }
        
        Ok(())
    }

    fn get_header(&self) -> String {
        let mut header = String::new();
        header += "\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        header += "\x01\x00\x3e\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        header += "\x00\x00\x00\x00\x00\x00\x00\x00\x40\x00\x00\x00\x00\x00\x00\x00";
        header += "\x00\x00\x00\x00\x40\x00\x00\x00\x00\x00\x40\x00\x07\x00\x03\x00";

        header
    }

    fn get_data(&self) -> (u64, Vec<u8>) {
        let mut data = Vec::new();
        data.append(&mut b"\x25\x6c\x64\x00\x25\x6c\x64\x0a\x00\x25\x6c\x66\x00\x25\x6c\x66\x0a\x00".to_vec());
        for _ in 0..self.ident_count+1 {
            data.append(&mut b"\x00\x00\x00\x00\x00\x00\x00\x00".to_vec());
        }

        let size = data.len();
        
        for _ in 0..(16 - size % 16) {
            data.append(&mut vec![0x00]);
        }
        
        (size as u64, data)
    }

    fn get_symtab(&mut self) -> (u32, Vec<u8>) {
        let mut data = Vec::new();

        data.append(&mut Symtab::null().to_vec());
        data.append(&mut Symtab::data().to_vec());
        data.append(&mut Symtab::text().to_vec());
        
        data.append(&mut Symtab::ident(1, 0).to_vec());
        data.append(&mut Symtab::ident(4, 4).to_vec());
        data.append(&mut Symtab::ident(7, 9).to_vec());
        data.append(&mut Symtab::ident(11, 13).to_vec());

        for i in 0..self.ident_count+1 {
            data.append(&mut Symtab::ident(self.ident_name_offset[i as usize], 0x12 + i as u64 * 8).to_vec());
        }

        data.append(&mut Symtab::scanf().to_vec());
        data.append(&mut Symtab::printf().to_vec());

        data.append(&mut Symtab::main().to_vec());

        (8+self.ident_count as u32, data)
    }

    fn get_strtab(&mut self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut b"\0if\0of\0iff\0off\0printf\0main\0scanf\0".to_vec());

        for i in 0..self.ident_count+1 {
            self.ident_name_offset.push(data.len() as u32);
            data.append(&mut format!("i{}\0", i).as_bytes().to_vec());
        }

        data
    }

    fn get_rela_text(&mut self) -> Vec<u8> {
        let mut data = Vec::new();
        let max = self.ident_count as u64;

        for (id, pos, big) in self.asm_idents.clone() {
            match id {
                id if id == max+1 =>
                    data.append(&mut Rela::format(pos, 0, big).to_vec()),
                id if id == max+2 =>
                    data.append(&mut Rela::format(pos, 9, big).to_vec()),
                id if id == max+3 =>
                    data.append(&mut Rela::format(pos, 4, big).to_vec()),
                id if id == max+4 =>
                    data.append(&mut Rela::format(pos, 13, big).to_vec()),
                id if id == max+5 =>
                    data.append(&mut Rela::scanf(pos, 8 + self.ident_count as u64).to_vec()),
                id if id == max+6 =>
                    data.append(&mut Rela::printf(pos, 9 + self.ident_count as u64).to_vec()),
                id if big => data.append(&mut Rela::x8(pos, id).to_vec()),
                id => data.append(&mut Rela::x4(pos, id).to_vec())
            }
        }

        data
    }
}
