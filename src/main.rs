use std::collections::HashSet;

use c_emitter::CEmitter;
use either::Either;
use first_opcode_byte_table::gen_first_opcode_byte_table;
use table_types::*;

mod c_emitter;
mod first_opcode_byte_table;
mod table_gen_utils;
mod table_types;

const CODEGEN_TYPES: &str = include_str!("codegen_types.h");

fn table_all_regular_insn_infos<'a>(
    table: &'a [InsnInfo],
) -> impl Iterator<Item = &'a RegularInsnInfo> + 'a {
    table
        .iter()
        .map(|insn_info| match insn_info {
            InsnInfo::Regular(info) => Either::Left(std::iter::once(info)),
            InsnInfo::ModrmRegOpcodeExt(modrm_reg_opcode_ext_info) => {
                Either::Right(modrm_reg_opcode_ext_info.by_reg_value.iter())
            }
        })
        .flatten()
}

fn table_all_mnemonics<'a>(table: &'a [InsnInfo]) -> impl Iterator<Item = Mnemonic> + 'a {
    table_all_regular_insn_infos(table).map(|insn_info| insn_info.mnemonic)
}

fn table_all_ops<'a>(table: &'a [InsnInfo]) -> impl Iterator<Item = Ops> + 'a {
    table_all_regular_insn_infos(table).map(|insn_info| insn_info.ops)
}

fn mnemonic_to_c_variant_name(mnemonic: Mnemonic) -> String {
    format!("MNEMONIC_{}", mnemonic.to_uppercase())
}

fn main() {
    let mut emitter = CEmitter::new();
    let first_opcode_byte_table = gen_first_opcode_byte_table();

    let mut mnemonics: HashSet<_> = table_all_mnemonics(&first_opcode_byte_table).collect();
    // a psuedo mnemonic used to represent the fact that this instruction required further identification using the reg field
    // of the modrm field.
    mnemonics.insert("MNEOMNIC_MODRM_REG_OPCODE_EXT");

    let ops: HashSet<_> = table_all_ops(&first_opcode_byte_table).collect();

    emitter.emit_enum(
        "mnemonic_t",
        mnemonics.iter().map(|x| mnemonic_to_c_variant_name(*x)),
    );

    emitter
        .begin_struct("insn_info_t")
        .bit_field_min_size("mnemonic", mnemonics.len())
        .bit_field_min_size("ops_index", ops.len())
        .emit();

    println!("{}", emitter.code());
    println!("{}", mnemonics.len());
    println!("{}", ops.len());
}
