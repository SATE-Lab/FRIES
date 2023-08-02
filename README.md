# FRIES (Fuzzing Rust Library Interactions via Ecosystem-Guided Target Generation)
This is the prototype tool for FRIES. We implemented FRIES based on the rustc compiler, utilizing the MIR module and librustdoc. Meanwhile, we have implemented automated scripts to handle the corpora and fuzzy testing.

## FRIES_core
Here is the core of FRIES, which includes the corpus parsing module and the target library parsing module. This project is based on Rust Projects and utilizes Rustc and librustdoc.

## FRIES_corpus_script
Here is the script FRIES uses to process the corpus in batches, including downloading, batch parsing, and streamlining deduplication.

## FRIES_test_script
This script is used by FRIES to fuzz ambiguous targets and perform batch processing on the generated files.

# How to run it

## Change the version of the rustup toolchain.
```
rustup toolchain install nightly-2022-11-30  
rustup default nightly-2022-11-30 
```
## install it
```
cd $WORKDIR/FRIES_core
./x.py setup  ./x.py check
# This may fail because the LLVM library from eight months ago may not be available.
./x.py build --stage=2 && rustup toolchain link fuzz build/x86_64-unknown-linux-gnu/stage2 
```

## Analyse target library
```
cd $TL_ROOT_DIR
cargo +fuzz doc --target-dir=tested
```

## Analyse corpus crate

```
cd $CP_DIR
cargo +fuzz doc
```
Alternatively, there are automation scripts available for running. Before using it, you need to modify the code inside. In parse_dependents.rs, locate the last line and modify it to your experiment root directory, for example:
```
const EXPERIMENT_ROOT_PATH: &'static str = "/home/.../workspace/fuzz/experiment_root/";
```
install it
```
cd FRIES_corpus_script
cargo install --path .
```
clone the corpus crates
```
rust_fuzzer parse-dep --clone --name xxx -n 500 
```
corpus analysis
```
rust_fuzzer parse-dep --parse --name xxx -n 500
```
dedup the information
```
rust_fuzzer parse-dep --name url --dedup
```
## Fuzz
install
```
cd $WORK_DIR/FRIES_test_script
cargo install --path afl_scripts/
cargo install --path find_literal/
```
fuzz
```
RUST_LOG=afl_scripts afl_scripts --all xxx
```

