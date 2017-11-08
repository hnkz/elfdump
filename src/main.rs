use std::fs::File;
use std::io::Read;
use std::env;

extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian, BigEndian};

struct Elf_Header {
    e_ident:        [u8; 16],    /* Magic number and other info */
    e_type:         u16,        /* Object file type */
    e_machine:      u16,        /* Architecture */
    e_version:      u32,        /* Object file version */
    e_entry:        u64,        /* Entry point virtual address */
    e_phoff:        u64,        /* Program header table file offset */
    e_shoff:        u64,        /* Section header table file offset */
    e_flags:        u32,        /* Processor-specific flags */
    e_ehsize:       u16,        /* ELF header size in bytes */
    e_phentsize:    u16,        /* Program header table entry size */
    e_phnum:        u16,        /* Program header table entry count */
    e_shentsize:    u16,        /* Section header table entry size */
    e_shnum:        u16,        /* Section header table entry count */
    e_shstrndx:     u16,        /* Section header string table index */
}

impl Elf_Header {
    fn new(elf_binary: &[u8]) -> Result<Elf_Header, String> {
        let mut elf_header = Elf_Header {
            e_ident: [
                elf_binary[0], elf_binary[1], elf_binary[2], elf_binary[3],
                elf_binary[4], elf_binary[5], elf_binary[6], elf_binary[7],
                elf_binary[8], elf_binary[9], elf_binary[10], elf_binary[11],
                elf_binary[12], elf_binary[13], elf_binary[14], elf_binary[15],
            ],
            e_type:         LittleEndian::read_u16(&elf_binary[16..18]),
            e_machine:      LittleEndian::read_u16(&elf_binary[18..20]),
            e_version:      LittleEndian::read_u32(&elf_binary[20..24]),
            e_entry:        LittleEndian::read_u64(&elf_binary[24..32]),
            e_phoff:        LittleEndian::read_u64(&elf_binary[32..40]),
            e_shoff:        LittleEndian::read_u64(&elf_binary[40..48]),
            e_flags:        LittleEndian::read_u32(&elf_binary[48..52]),
            e_ehsize:       LittleEndian::read_u16(&elf_binary[52..54]),
            e_phentsize:    LittleEndian::read_u16(&elf_binary[54..56]),
            e_phnum:        LittleEndian::read_u16(&elf_binary[56..58]),
            e_shentsize:    LittleEndian::read_u16(&elf_binary[58..60]),
            e_shnum:        LittleEndian::read_u16(&elf_binary[60..62]),
            e_shstrndx:     LittleEndian::read_u16(&elf_binary[62..64]),
        };

        Ok(elf_header)
    }

    fn print_Elf_Header(elf_header: &Elf_Header){
        println!("e_ident[EI_MAG0]      : {:x}", elf_header.e_ident[0]);
        println!("e_ident[EI_MAG1]      : {:x}", elf_header.e_ident[1]);
        println!("e_ident[EI_MAG2]      : {:x}", elf_header.e_ident[2]);
        println!("e_ident[EI_MAG3]      : {:x}", elf_header.e_ident[3]);
        println!("e_ident[EI_CLASS]     : {:x}", elf_header.e_ident[4]);
        println!("e_ident[EI_DATA]      : {:x}", elf_header.e_ident[5]);
        println!("e_ident[EI_VERSION]   : {:x}", elf_header.e_ident[6]);
        println!("e_ident[EI_OSABI]     : {:x}", elf_header.e_ident[7]);
        println!("e_ident[EI_ABIVERSION]: {:x}", elf_header.e_ident[8]);
        print!("e_ident[EI_PAD]       : {:x}", elf_header.e_ident[9]);
        print!(" {:x}", elf_header.e_ident[10]);
        print!(" {:x}", elf_header.e_ident[11]);
        print!(" {:x}", elf_header.e_ident[12]);
        print!(" {:x}", elf_header.e_ident[13]);
        print!(" {:x}", elf_header.e_ident[14]);
        println!(" {:x}", elf_header.e_ident[15]);
        println!("e_machine             : {:x}", elf_header.e_machine);
        println!("e_version             : {:x}", elf_header.e_version);
        println!("e_entry               : {:x}", elf_header.e_entry);
        println!("e_phoff               : {:x}", elf_header.e_phoff);
        println!("e_shoff               : {:x}", elf_header.e_shoff);
        println!("e_flags               : {:x}", elf_header.e_flags);
        println!("e_ehsize              : {:x}", elf_header.e_ehsize);
        println!("e_phentsize           : {:x}", elf_header.e_phentsize);
        println!("e_phnum               : {:x}", elf_header.e_phnum);
        println!("e_shentsize           : {:x}", elf_header.e_shentsize);
        println!("e_shnum               : {:x}", elf_header.e_shnum);
        println!("e_shstrndx            : {:x}", elf_header.e_shstrndx);

    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    let mut file = match File::open(&args[1]){
        Ok(file) => file,
        Err(e) => {
            println!("could not read file {}", e);
            return;
        }
    };

    let mut buf = [0u8; 1024];
    let _ = match file.read(&mut buf){
        Ok(buf) => buf,
        Err(e) => {
            println!("Could not read file description. {}", e);
            return;
        }
    };

    /* print elf_header_binary
    for b in buf.iter(){
        print!("{:x} ", b);
    }
    println!();
    */

    let elf_header = match Elf_Header::new(&buf[0..]) {
        Ok(elf_header) => elf_header,
        Err(error) => {
            println!("Can not convert binary to Elf_Header struct. {}", error);
            return;
        }
    };

    print_Elf_Header(&elf_header);

}