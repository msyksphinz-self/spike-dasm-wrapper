#[link(name = "spike-dasm", kind="static")]
use std::ffi::CStr;
use std::os::raw::c_char;

pub enum DisasmImpl {}

extern {
    pub fn Disasm_Disasm() -> *mut DisasmImpl;
    pub fn Disasm_disassemble(diasm: *mut DisasmImpl, insn:u32) -> *const c_char;
}

pub struct Disasm {
    raw: *mut DisasmImpl
}

impl Disasm {
    #[inline]
    pub fn new() -> Self {
        unsafe { Disasm { raw: Disasm_Disasm()}}
    }
    #[inline]
    pub fn disassemble(&mut self, insn: u32) ->String {
        let st = unsafe {
            let st_raw = Disasm_disassemble(self.raw, insn);
            CStr::from_ptr(st_raw).to_string_lossy().into_owned()
        };
        return st;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut disasm = Disasm::new();
        
        let dis_str = disasm.disassemble(0x0000_0000);
        assert_eq!(dis_str, "c.addi4spn s0, sp, 0");

        let dis_str = disasm.disassemble(0x01c28293);
        assert_eq!(dis_str, "addi    t0, t0, 28");

        let dis_str = disasm.disassemble(0x4201);
        assert_eq!(dis_str, "c.li    tp, 0");
    }
}

