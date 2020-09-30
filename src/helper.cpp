#include <iostream>
#include "disasm.h"

extern "C"
{
    typedef struct
    {
        disassembler_t impl;
    } DisasmImpl;

    DisasmImpl *Disasm_Disasm()
    {
        disassembler_t *diasm = new disassembler_t(64);
        return (DisasmImpl *)diasm;
    }

    const char *Disasm_disassemble(DisasmImpl *di, uint32_t insn)
    // std::string Disasm_disassemble(DisasmImpl *di, uint32_t insn)
    {
        auto str = di->impl.disassemble(insn);
        char *str_ptr = new char[100];
        memcpy(str_ptr, str.c_str(), str.length());
        return str_ptr;
    }
}