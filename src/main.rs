use std::collections::HashSet;

use either::Either;
use first_opcode_byte_table::gen_first_opcode_byte_table;
use table_types::*;

mod first_opcode_byte_table;
mod table_gen_utils;
mod table_types;

const CODEGEN_TYPES: &str = include_str!("codegen_types.h");

fn table_all_mnemonics<'a>(table: &'a [InsnInfo]) -> impl Iterator<Item = Mnemonic> + 'a {
    table
        .iter()
        .map(|insn_info| match insn_info {
            InsnInfo::Regular(info) => Either::Left(std::iter::once(info.mnemonic)),
            InsnInfo::ModrmRegOpcodeExt(modrm_reg_opcode_ext_info) => Either::Right(
                modrm_reg_opcode_ext_info
                    .by_reg_value
                    .iter()
                    .map(|info| info.mnemonic),
            ),
        })
        .flatten()
}

fn main() {
    let t = gen_first_opcode_byte_table();
    let mnemonics: HashSet<_> = table_all_mnemonics(&t).collect();
    println!("{:?}", mnemonics);
}
