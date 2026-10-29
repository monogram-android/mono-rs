# Third-party notices

## Telegram TL schemas

The generated Telegram Type Language declarations come from Telegram schema snapshots. The supported API layers are 223, 229, 230, 231 and 232. These archived snapshots and their SHA-256 digests produced the current generated output:

| Layer | Snapshot                                  | SHA-256                                                            |
|-------|-------------------------------------------|--------------------------------------------------------------------|
| 223   | <https://schema.jppgr.am/layers/223.tl>   | `9ba100ea43cb1963ae69f95f9353a7e8abaa6769929fedf842c39afb7132e719` |
| 229   | <https://schema.jppgr.am/layers/229.tl>   | `c5309939182355548afac6d2f16b123dd8037157c489ee000493ab11163901a5` |
| 230   | <https://schema.jppgr.am/layers/230.tl>   | `c12a6c671796142523a6e5e6a2af1dd83864f53caf732ed98b63ea455ab222ac` |
| 231   | <https://schema.jppgr.am/layers/231.tl>   | `83ccd40f4e4393f32d5cd2a5bb360013a49f86f861ae10ae43085e657778c21e` |
| 232   | <https://schema.jppgr.am/layers/232.tl>   | `7a1f11881d1e0f021af61c97464fec1e5d88edf9f4920950fe68e0c31c0f9947` |

The checked-in generator is mono-rs-owned and lives in the `mono-tl-parser`, `mono-tl-schema` and `mono-tl-codegen` crates. It parses the pinned TL source, validates the schema model, and emits the same bounded `crate::codec` traits used by `mono-proto`. Run `cargo xtask generate-schemas` to regenerate all five modules, or `cargo xtask check-schemas` in CI to verify the downloaded digests and compare generated output byte-for-byte. `cargo xtask test-layers` then compiles and tests every supported generated layer. The generator crates are adapted from an MPL-2.0 implementation; their `LICENSE-MPL-2.0` files and this notice preserve that license. Generated declarations must not be edited by hand. The source URLs and digests below are the provenance record for the checked-in output.

The mono-rs generator crates are covered by MPL-2.0 and retain `LICENSE-MPL-2.0` beside their source. The repository's GPL-3.0-only license covers original project code and contributions. It doesn't remove third-party obligations, and it doesn't relicense third-party material beyond the grants that apply to it.

## Dependencies and vendored code

Rust dependencies keep the licenses their authors declared. Separately vendored components keep their own license files and notices. Review those terms when you redistribute a combined product.
