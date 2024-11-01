pub type Mnemonic = &'static str;

enum OpSize {
    S8,
    S16,
    S32,
    S64,
}

enum ImmExtendKind {
    SignExtend,
    ZeroExtend,
}

struct ImmOpInfo {
    encoded_size: OpSize,
    extended_size: OpSize,
    extend_kind: ImmExtendKind,
}

enum RegEncoding {
    Modrm,
    Opcode,
}

struct RegOpInfo {
    encoding: RegEncoding,
    size: OpSize,
}

struct RmOpInfo {
    size: OpSize,
}

enum OpInfo {
    Imm(ImmOpInfo),
    Reg(RegOpInfo),
    Rm(RmOpInfo),
}

struct InsnInfo {
    mnemonic: Mnemonic,
    ops: Vec<OpInfo>,
}

fn main() {
    println!("Hello, world!");
}
