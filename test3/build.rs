// Copyright 2018 POA Networks Ltd.
//
// This file is part of the POA Networks bridge contracts.
//
// The POA Networks bridge contracts are free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.
#![forbid(warnings)]
use std::{env, fs::File, io::Write, path::PathBuf};
extern crate tiny_keccak;
use tiny_keccak::Keccak;

/// Write the generated code to `f`.
fn generate(f: &mut dyn Write) {
    let mut hash = [0; 32];
    for (i, j) in &[
        ("deployedAtBlock", "DEPLOYED_AT_BLOCK"),
        ("requiredSignatures", "REQUIRED_SIGNATURES"),
        ("validatorCount", "VALIDATOR_COUNT"),
        ("isInitialized", "IS_INITIALIZED"),
        ("owner", "OWNER",)
    ] {
        let mut q = Keccak::new_sha3_256();
        q.update(i.as_bytes());
        q.finalize(&mut hash);
        write!(
            f,
            "const {cname}: [u8;32] = {hash:?};\
            #[allow(non_snake_case)]fn get_{i}()->[u8;32]{{read({typeconv}({cname}))}}\
            #[allow(non_snake_case)]fn set_{i}(s:[u8;32]){{write({typeconv}({cname}),&s)}}\n",
            cname = j,
            hash = hash,
            i = i,
            typeconv = "&pwasm_abi::types::H256::from",
        )
        .expect("I/O error in build script");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_path = env::var("OUT_DIR").expect("cargo should have set this");
    let mut path = PathBuf::from(&manifest_path);
    path.push("hashes.rs");
    let mut f = File::create(path).expect("cannot create constants.rs");
    generate(&mut f);
}
