use crate::utils::cow_struct;
use std::fmt;
use std::mem::size_of;

/// Identification offset for the elf binary `e_ident` field
pub struct Indent {}
#[warn(unused_must_use)]
impl Indent {
    //const MAG0: usize = 0;
    //const MAG1: usize = 1;
    //const MAG2: usize = 2;
    //const MAG3: usize = 3;
    const CLASS: usize = 4;
    const DATA: usize = 5;
    const VERSION: usize = 6;
    const OSABIT: usize = 7;
    const ABIVERSION: usize = 8;
    // const PAD: usize = 9;
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

/// Define the version number of the elf specification
///
/// Version define possible values for the version number
/// Its store in the seventh byte of the identifaction 16 bits called `e_ident`
pub struct VERSION {}
impl VERSION {
    /// invalid version
    const NONE: u8 = 0;
    /// current elf version
    const CURRENT: u8 = 1;
}

pub struct OSABIT {}

/// Define the target operating system application binary interface
///
/// OSABIT define the possible values for the target operation system
/// Its store in the seventh byte of the identifaction 16 bits called `e_ident`
impl OSABIT {
    /// UNIX System V ABI
    const SYSV: u8 = 0;
    /// HP-UX
    const HPUX: u8 = 1;
    /// NetBSD.
    const NETBSD: u8 = 2;
    /// Object use GNU ELF extensions
    const GNU: u8 = 3;
    /// Sun Solaris
    const SOLARIS: u8 = 6;
    /// IBM AIX
    const AIX: u8 = 7;
    /// SGI Irix
    const IRIX: u8 = 8;
    /// FreeBSD
    const FREEBSD: u8 = 9;
    /// Compaq tru64 unix
    const TRU64: u8 = 10;
    /// Novell Modesto
    const MODESTO: u8 = 11;
    /// OpenBSD
    const OPENBSD: u8 = 12;
    /// ARM EABI
    const ARM_AEABI: u8 = 64;
    /// ARM
    const ARM: u8 = 97;
    /// Standalone embedded application
    const STANDALONE: u8 = 255;
}

/// Define the object file type
pub struct TYPE {}

impl TYPE {
    /// No file type
    const NONE: u16 = 0;
    /// Relocatable file
    const REL: u16 = 1;
    /// Executable file
    const EXEC: u16 = 2;
    /// Share object file
    const DYN: u16 = 3;
    /// Core file
    const CORE: u16 = 4;
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
        write!(f, "ELF Hearder:\n").unwrap();

        // Write indent
        write!(
            f,
            "Magic:\t\t\t\t\t{}\n",
            self.e_ident
                .iter()
                .map(|hex| format!("{:02X?} ", hex))
                .collect::<String>()
        )
        .unwrap();

        // Write class
        let class = match self.e_ident[Indent::CLASS] {
            CLASS::NONE => "Invalid class",
            CLASS::ELF32 => "ELF32",
            CLASS::ELF64 => "ELF64",
            _ => "Warning: unknown class",
        };
        write!(f, "Class:\t\t\t\t\t{}, \n", class).unwrap();

        // write data encoding
        let data_encoding = match self.e_ident[Indent::DATA] {
            DATA::NONE => "Unknown data encoding",
            DATA::BE => "2's complement, big endian",
            DATA::LE => "2's complement, little endian",
            _ => "Warning: unknow data encoding",
        };
        write!(f, "Data:\t\t\t\t\t{}\n", data_encoding).unwrap();

        // write current number version of elf specification
        let current = format!("{} (current)", self.e_ident[Indent::VERSION]);
        let version = match self.e_ident[Indent::VERSION] {
            VERSION::NONE => "Invalid version",
            VERSION::CURRENT => current.as_str(),
            _ => "Warning: unknow version",
        };
        write!(f, "Version:\t\t\t\t{}\n", version).unwrap();

        // write target os application binary interface
        let osabit = match self.e_ident[Indent::OSABIT] {
            OSABIT::SYSV => "UNIX System V ABI",
            OSABIT::HPUX => "HP-UX",
            OSABIT::NETBSD => "NetBSD",
            OSABIT::GNU => "Object use GNU ELF extensions",
            OSABIT::SOLARIS => "Sun Solaris",
            OSABIT::AIX => "IBM AIX",
            OSABIT::IRIX => "SGI Irix",
            OSABIT::FREEBSD => "FreeBSD",
            OSABIT::TRU64 => "Compaq tru64 unix",
            OSABIT::MODESTO => "Novell Modesto",
            OSABIT::OPENBSD => "OpenBSD",
            OSABIT::ARM_AEABI => "ARM AEABI",
            OSABIT::ARM => "ARM",
            OSABIT::STANDALONE => "Standalone embedded application",
            _ => "Warning: unknow operating system target",
        };
        write!(f, "OS/ABI:\t\t\t\t\t{}\n", osabit).unwrap();

        let abi_version_message = match self.e_ident[Indent::ABIVERSION] {
            0 => "0",
            _ => "Warning: Not compatible with the specification",
        };
        write!(f, "ABI Version:\t\t\t\t{}\n", abi_version_message);

        // write object file type
        let obj_type = match self.e_type {
            TYPE::NONE => "NONE (No file type)",
            TYPE::REL => "REL (Relocatable file)",
            TYPE::EXEC => "EXEC (Executable file)",
            TYPE::DYN => "DYN (Share object file)",
            TYPE::CORE => "CORE (Core file)",
            _ => "Warning: unknow object file type",
        };
        write!(f, "Type: \t\t\t\t\t{}\n", obj_type)
    }
}
