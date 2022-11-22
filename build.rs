use std::{env, fs, path::Path, vec::Vec};

use codegen::{Field, Function, Impl, Module, Scope, Struct, Trait};

struct RegisterData {
    addr: u8,
    data: Vec<(String, u32)>,
}

impl RegisterData {
    fn new(addr: u8, data: Vec<(String, u32)>) -> Self {
        RegisterData { addr, data }
    }

    fn from_string(addr: u8, s: String) -> Result<Self, ()> {
        let mut names = Vec::<String>::new();
        let mut lengths = Vec::<u32>::new();
        for (index, field) in s.split_whitespace().enumerate() {
            if index == 0 && field == "skip" {
                return Err(());
            }

            if index % 2 == 0 {
                names.push(field.parse().expect("Cannot parse register name."));
            } else {
                lengths.push(field.parse().expect("Cannot parse register length."));
            }
        }

        let mut zipped = names
            .iter()
            .zip(lengths.iter())
            .map(|item| (item.0.clone(), *item.1))
            .collect::<Vec<(String, u32)>>();
        zipped.reverse(); // Fields are saved in reversed order due to bitfield endianness.

        Ok(RegisterData::new(addr, zipped))
    }
}

fn read_from_file(file_name: &str) -> Vec<RegisterData> {
    let file_data =
        fs::read_to_string(file_name).unwrap_or_else(|_| panic!("Cannot read {}.", file_name));
    let mut register_array = Vec::<RegisterData>::new();
    for (i, line) in file_data.lines().enumerate() {
        if let Ok(reg) = RegisterData::from_string(i as u8, line.to_string()) {
            register_array.push(reg)
        }
    }
    register_array
}

fn generate_register_structs(register_array: &Vec<RegisterData>) -> Scope {
    let mut scope = Scope::new();

    // Trait.
    // TODO: Implement debug for all the structs.
    let mut registers_trait = Trait::new("RegisterWritable");
    registers_trait
        .new_fn("into_reg_bytes")
        .arg_self()
        .ret("[u8; 3]");
    registers_trait
        .new_fn("from_reg_bytes")
        .arg("bytes", "[u8; 3]")
        .ret("Self");
    scope.push_trait(registers_trait);

    // Mod.
    let mut register_structs_module = Module::new("register_structs")
        .import("modular_bitfield::prelude", "*")
        .import("super", "RegisterWritable")
        .attr("allow(clippy::too_many_arguments)")
        .attr("allow(clippy::fn_params_excessive_bools)")
        .attr("allow(dead_code)")
        .attr("allow(unreachable_pub)")
        .vis("pub(crate)")
        .to_owned();

    for register in register_array {
        // Struct.
        let mut current_struct = Struct::new(format!("R{:02X}h", register.addr).as_str());

        current_struct
            .attr("bitfield")
            .vis("pub(crate)")
            .derive("Copy, Clone");

        let mut skips: u8 = 0;

        for (name, length) in register.data.iter() {
            if name == "0" {
                let field = Field::new(&*format!("__{}", skips), format!("B{}", length))
                    .annotation("#[skip]")
                    .to_owned();
                skips += 1;
                current_struct.push_field(field);
            } else {
                let mut field = match length {
                    1 => Field::new(name.as_str(), "bool"),
                    8 | 16 | 32 | 64 => Field::new(name.as_str(), format!("u{}", length)),
                    _ => Field::new(name.as_str(), format!("B{}", length)),
                };

                current_struct.push_field(field.vis("pub(crate)").to_owned());
            }
        }
        register_structs_module.push_struct(current_struct);

        // Trait impl.
        let mut current_trait_impl = Impl::new(format!("R{:02X}h", register.addr));
        current_trait_impl.impl_trait("RegisterWritable");
        current_trait_impl
            .new_fn("into_reg_bytes")
            .arg_self()
            .ret("[u8; 3]")
            .line("let mut reversed = self.into_bytes();")
            .line("reversed.reverse();")
            .line("reversed");
        current_trait_impl
            .new_fn("from_reg_bytes")
            .arg("bytes", "[u8; 3]")
            .ret("Self")
            .line("let mut reversed = bytes;")
            .line("reversed.reverse();")
            .line("Self::from_bytes(reversed)");
        register_structs_module.push_impl(current_trait_impl);
    }

    scope.push_module(register_structs_module);

    scope
}

fn generate_register_block(register_array: &Vec<RegisterData>) -> Scope {
    let mut scope = Scope::new();

    // Import.
    scope.raw("include!(concat!(env!(\"OUT_DIR\"), \"/register_structs.rs\"));");

    // Mod.
    let mut register_block_module = Module::new("register_block")
        .import("std::cell", "RefCell")
        .import("std::rc", "Rc")
        .import("embedded_hal::i2c", "I2c")
        .import("embedded_hal::i2c", "SevenBitAddress")
        .import("crate::register", "Register")
        .import("super::register_structs", "*")
        .vis("pub(crate)")
        .to_owned();

    // Struct.
    let mut register_block_struct = Struct::new("RegisterBlock")
        .generic("I2C")
        .allow("dead_code")
        .allow("non_snake_case")
        .vis("pub(crate)")
        .to_owned();

    for register in register_array {
        let field = Field::new(
            format!("r{:02X}h", register.addr).as_str(),
            format!("Register<I2C, R{:02X}h>", register.addr),
        )
        .vis("pub(crate)")
        .to_owned();

        register_block_struct.push_field(field);
    }
    register_block_module.push_struct(register_block_struct);

    // Impl.
    let mut new_function = Function::new("new");
    new_function
        .vis("pub(crate)")
        .arg("phy_addr", "SevenBitAddress")
        .arg("i2c", "&Rc<RefCell<I2C>>")
        .ret("Self")
        .line("Self {");
    for register in register_array {
        new_function.line(format!(
            "r{:02X}h: Register::new({:#04X}, phy_addr, Rc::clone(i2c)),",
            register.addr, register.addr
        ));
    }
    new_function.line("}");
    let mut register_block_implementation = Impl::new("RegisterBlock<I2C>");
    register_block_implementation
        .generic("I2C")
        .bound("I2C", "I2c")
        .push_fn(new_function);
    register_block_module.push_impl(register_block_implementation);

    scope.push_module(register_block_module);

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
