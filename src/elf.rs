

/*
 * ELF Header
 * size: 52 Byte
 */
struct Elf_Header {
    e_ident:        [u8: 4], /* Magic number and other info */
    e_type:         u16, /* Object file type */
    e_machine:      u16, /* Architecture */
    e_version:      u32, /* Object file version */
    e_entry:        u64, /* Entry point virtual address */
    e_phoff:        u64, /* Program header table file offset */
    e_shoff:        u64, /* Section header table file offset */
    e_flags:        u32, /* Processor-specific flags */
    e_ehsize:       u16, /* ELF header size in bytes */
    e_phentsize:    u16, /* Program header table entry size */
    e_phnum:        u16, /* Program header table entry count */
    e_shentsize:    u16, /* Section header table entry size */
    e_shnum:        u16, /* Section header table entry count */
    e_shstrndx:     u16, /* Section header string table index */
}


