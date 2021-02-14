use crate::utils::cow_struct;
use std::fmt;
use std::mem::size_of;

/// Identification offset for the elf binary `e_ident` field
pub struct indent {}
impl indent {
    const MAG0: usize = 0;
    const MAG1: usize = 1;
    const MAG2: usize = 2;
    const MAG3: usize = 3;
    const CLASS: usize = 4;
    const DATA: usize = 5;
    const VERSION: usize = 6;
    const OSABIT: usize = 7;
    const ABIVERSION: usize = 8;
    const PAD: usize = 9;
}

/// Define the architecture for the binary
///
/// CLASS defined possible value for a binary
/// Its store in the fifth byte of the identifaction 16 bits called `e_ident`
pub struct CLASS {}
impl CLASS {
    /// Invalid class
    const NONE: u8 = 0;
    /// 32 bits object
    const ELF32: u8 = 1;
    /// 32 bits object
    const ELF64: u8 = 2;
}

/// Define the data encoding for the processor-specific data
///
/// Data define possible values for the encoding
/// Its store in the sixth byte of the identifaction 16 bits called `e_ident`
pub struct DATA {}
impl DATA {
    /// Unknow data encoding
    const NONE: u8 = 0;
    /// Little endian data encoding
    const LE: u8 = 1;
    /// Big endian data encoding
    const BE: u8 = 2;
}

/// Format of Executable and Linking Format (ELF64) files
///
/// The header file <elf.h> defines the format of ELF executable
/// binary files.  Amongst these files are normal executable files,
/// relocatable object files, core files, and shared objects.
///
/// An executable file using the ELF file format consists of an ELF
/// header, followed by a program header table or a section header
/// table, or both.  The ELF header is always at offset zero of the
/// file.  The program header table and the section header table's
/// offset in the file are defined in the ELF header.  The two tables
/// describe the rest of the particularities of the file.
/// Sources:
/// * https://www.man7.org/linux/man-pages/man5/elf.5.html
/// * https://uclibc.org/docs/elf-64-gen.pdf
#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed)]
pub struct Elf64 {
    /// ELF identifaction
    pub e_ident: [u8; 16],
    /// object file type
    pub e_type: u16,
    /// machine type
    pub e_machine: u16,
    /// object file version
    pub e_version: u32,
    /// Entry point address
    pub e_entry: u64,
    /// program header offset
    pub e_phoff: u64,
    /// section header offset
    pub e_shoff: u64,
    /// processor specific flags
    pub e_flags: u32,
    /// elf header size
    pub e_ehsize: u16,
    /// Size of program header entry
    pub e_phentsize: u16,
    /// numbers of program header entries
    pub e_phnum: u16,
    /// size of section header entry
    pub e_shentsize: u16,
    /// number of section header entries
    pub e_shnum: u16,
    /// section name string table index
    pub e_shstrndx: u16,
}

impl Elf64 {
    pub const SIZE: usize = size_of::<Self>();

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        let (header_bytes, _data) = data.split_at(Elf64::SIZE);
        match cow_struct::<Self>(header_bytes) {
            Some(header) => println!("{}", header),
            None => println!("failed !"),
        }
        return None;
    }
}

impl fmt::Display for Elf64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write class
        write!(f, "CLASS:\t\t\t\t\t").unwrap();
        match self.e_ident[indent::CLASS] {
            CLASS::NONE => write!(f, "Invalid class\n",),
            CLASS::ELF32 => write!(f, "ELF32\n"),
            CLASS::ELF64 => write!(f, "ELF64\n"),
            _ => write!(f, "Warning: unknown class\n"),
        };

        // write data encoding
        write!(f, "DATA:\t\t\t\t\t").unwrap();
        match self.e_ident[indent::DATA] {
            DATA::NONE => write!(f, "Unknown data encodingi\n"),
            DATA::BE => write!(f, "2's complement, big endian\n"),
            DATA::LE => write!(f, "2's complement, little endian\n"),
            _ => write!(f, "Warning: unknow data encoding"),
        }
    }
}
