use std::{
    env,
    fs,
    path::Path,
    vec::Vec,
};

use codegen::{Function, Impl, Module, Scope, Struct, Trait};

struct RegisterData {
    addr: u8,
    data: Vec<(String, u32)>,
}

impl RegisterData {
    fn new(addr: u8, data: Vec<(String, u32)>) -> Self {
        RegisterData {
            addr,
            data,
        }
    }

    fn from_string(addr: u8, s: String) -> Result<Self, ()> {
        let mut names = Vec::<String>::new();
        let mut lenghts = Vec::<u32>::new();
        for (index, field) in s.split_whitespace().enumerate() {
            if index == 0 && field == "skip" {
                return Err(());
            }

            if index % 2 == 0 {
                names.push(field.parse().expect("Cannot parse register name."));
            } else {
                lenghts.push(field.parse().expect("Cannot parse register length."));
            }
        }

        let zipped = names.iter()
            .zip(lenghts.iter())
            .map(|item| {
                (item.0.clone(), *item.1)
            })
            .collect::<Vec<(String, u32)>>();

        Ok(RegisterData::new(addr, zipped))
    }
}

fn read_from_file(file_name: &str) -> Vec<RegisterData> {
    let file_data = fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Cannot read {}.", file_name));
    let mut register_array = Vec::<RegisterData>::new();
    for (i, line) in file_data.lines().enumerate() {
        if let Ok(reg) = RegisterData::from_string(i as u8, line.to_string()) { register_array.push(reg) }
    }
    register_array
}

fn generate_register_structs(register_array: &Vec<RegisterData>) -> Scope {
    let mut scope = Scope::new();

    // Trait.
    let mut my_trait = Trait::new("RegisterWritable");
    my_trait.new_fn("into_reg_bytes")
        .arg_self()
        .ret("[u8; 3]");
    my_trait.new_fn("from_reg_bytes")
        .arg("bytes", "[u8; 3]")
        .ret("Self");
    scope.push_trait(my_trait);

    let mut registers_module = Module::new("registers");
    registers_module.import("modular_bitfield::prelude", "*");
    registers_module.import("super", "RegisterWritable");

    for register in register_array {

        // Struct.
        let mut my_struct = Struct::new(format!("R{:02X}h", register.addr).as_str());

        // A workaround for declaring bitfields inside a module.
        my_struct.vis("#[bitfield]\npub(crate)")
            .derive("Copy, Clone");
        for (name, length) in register.data.iter() {
            if name == "0" {
                my_struct.field("#[skip] __", format!("B{}", length));
            } else {
                match length {
                    1 => my_struct.field(name, "bool"),
                    8 | 16 | 32 | 64 => my_struct.field(name, format!("u{}", length)),
                    _ => my_struct.field(name, format!("B{}", length)),
                };
            }
        }
        registers_module.push_struct(my_struct);

        // Impl.
        let mut my_impl = Impl::new(format!("R{:02X}h", register.addr));
        my_impl.impl_trait("RegisterWritable");
        my_impl.new_fn("into_reg_bytes")
            .arg_self()
            .ret("[u8; 3]")
            .line("self.into_bytes()");
        my_impl.new_fn("from_reg_bytes")
            .arg("bytes", "[u8; 3]")
            .ret("Self")
            .line("Self::from_bytes(bytes)");
        registers_module.push_impl(my_impl);
    }

    scope.push_module(registers_module);

    scope
}

fn generate_register_block(register_array: &Vec<RegisterData>) -> Scope {
    let mut scope = Scope::new();

    // Import.
    scope.import("std::cell", "RefCell");
    scope.import("std::rc", "Rc");
    scope.import("embedded_hal::i2c::blocking", "I2c");
    scope.import("embedded_hal::i2c", "SevenBitAddress");
    scope.import("crate::register", "Register");
    scope.raw("include!(concat!(env!(\"OUT_DIR\"), \"/register_structs.rs\"));");
    scope.raw("use registers::*;");

    // Struct.
    let mut my_struct = Struct::new("RegisterBlock");
    my_struct.generic("I2C");
    my_struct.allow("dead_code").allow("non_snake_case");
    for register in register_array {
        my_struct.field(format!("r{:02X}h", register.addr).as_str(), format!("Register<I2C, R{:02X}h>", register.addr));
    }
    scope.push_struct(my_struct);

    // Impl.
    let mut my_fn = Function::new("new");
    my_fn.vis("pub")
        .arg("phy_addr", "SevenBitAddress")
        .arg("i2c", "&Rc<RefCell<I2C>>")
        .ret("Self")
        .line("Self {");
    for register in register_array {
        my_fn.line(format!("r{:02X}h: Register::new({:#04X}, phy_addr, Rc::clone(i2c)),", register.addr, register.addr));
    }
    my_fn.line("}");
    scope.new_impl("RegisterBlock<I2C>").generic("I2C").bound("I2C", "I2c").push_fn(my_fn);

    scope
}

fn main() {
    let vec = read_from_file("registers.dat");
    let register_structs: Scope = generate_register_structs(&vec);
    let register_block: Scope = generate_register_block(&vec);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let structs_path = Path::new(&out_dir).join("register_structs.rs");
    let block_path = Path::new(&out_dir).join("register_block.rs");

    fs::write(structs_path, register_structs.to_string()).expect("Cannot create structs file.");
    fs::write(block_path, register_block.to_string()).expect("Cannot create block file.");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=registers.dat");
}
