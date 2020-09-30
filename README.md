# spike-dasm-wrapper

spike-dasm-wrapper is wrapper of spike-dasm in [riscv-isa-sim](https://github.com/riscv/riscv-isa-sim).

## Usage

```toml
[dependencies]
spike-dasm-wrapper = "0.0.2"
```

## Example

```rust
use spike_dasm_wrapper::{Disasm};

fn main() {
	let mut disasm = Disasm::new();

	let dis_str = disasm.disassemble(0x0000_0000);
	assert_eq!(dis_str, "c.addi4spn s0, sp, 0");

	let dis_str = disasm.disassemble(0x01c28293);
	assert_eq!(dis_str, "addi    t0, t0, 28");

	let dis_str = disasm.disassemble(0x4201);
	assert_eq!(dis_str, "c.li    tp, 0");
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
