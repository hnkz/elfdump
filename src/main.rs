use std::fs::File;
use std::io::Read;
use std::env;

mod elf;

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

    let elf_header = match elf::ElfHeader::new(&buf[0..]) {
        Ok(elf_header) => elf_header,
        Err(error) => {
            println!("Can not convert binary to Elf_Header struct. {}", error);
            return;
        }
    };

    elf::ElfHeader::print_elf_header(&elf_header);

}