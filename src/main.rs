#[allow(non_snake_case)]
extern crate elf;
extern crate capstone;

use std::env;
use std::path::PathBuf;
use capstone::{Insn, prelude::*};

fn reg_names(cs: &Capstone, regs: &[RegId]) -> String {
    let names: Vec<String> = regs.iter().map(|&x| cs.reg_name(x).unwrap()).collect();
    names.join(", ")
}

/// Print instruction group names
fn group_names(cs: &Capstone, regs: &[InsnGroupId]) -> String {
    let names: Vec<String> = regs.iter().map(|&x| cs.group_name(x).unwrap()).collect();
    names.join(", ")
}

fn print_detail(cs:&Capstone, i:&Insn){
    let detail: InsnDetail = cs.insn_detail(&i).expect("Failed to get insn detail");
    let arch_detail: ArchDetail = detail.arch_detail();
    let ops = arch_detail.operands();

    let output: &[(&str, String)] = &[
        ("insn id:", format!("{:?}", i.id().0)),
        ("bytes:", format!("{:?}", i.bytes())),
        ("read regs:", reg_names(&cs, detail.regs_read())),
        ("write regs:", reg_names(&cs, detail.regs_write())),
        ("insn groups:", group_names(&cs, detail.groups())),
    ];

    for &(ref name, ref message) in output.iter() {
        println!("{:4}{:12} {}", "", name, message);
    }

    println!("{:4}operands: {}", "", ops.len());
    for op in ops {
        println!("{:8}{:?}", "", op);
    }
}
fn main() {
    let argv: Vec<String> = env::args().collect();
    let path = PathBuf::from(&argv[1]);
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_scn = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Att)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    let insns = cs.disasm_all(&text_scn.data, text_scn.shdr.addr)
        .expect("Failed to disassemble");
    println!("Found {} instructions", insns.len());
    for i in insns.as_ref() {
        if i.bytes().len() == 2 && i.bytes()[0] == 0xFF{
            println!("{}", i);
            // i.id().0 == 62 => callq
            // i.id().0 == 172 => jmpq
        }
    }
}
