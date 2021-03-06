// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use back::target_strs;
use driver::config::cfg_os_to_meta_os;
use metadata::loader::meta_section_name;
use syntax::abi;

pub fn get_target_strs(target_triple: String, target_os: abi::Os) -> target_strs::t {
    let cc_args = if target_triple.as_slice().contains("thumb") {
        vec!("-mthumb".to_strbuf())
    } else {
        vec!("-marm".to_strbuf())
    };
    return target_strs::t {
        module_asm: "".to_strbuf(),

        meta_sect_name:
            meta_section_name(cfg_os_to_meta_os(target_os)).to_strbuf(),

        data_layout: match target_os {
          abi::OsMacos => {
            "e-p:32:32:32\
                -i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64\
                -f32:32:32-f64:64:64\
                -v64:64:64-v128:64:128\
                -a0:0:64-n32".to_strbuf()
          }

          abi::OsWin32 => {
            "e-p:32:32:32\
                -i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64\
                -f32:32:32-f64:64:64\
                -v64:64:64-v128:64:128\
                -a0:0:64-n32".to_strbuf()
          }

          abi::OsLinux => {
            "e-p:32:32:32\
                -i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64\
                -f32:32:32-f64:64:64\
                -v64:64:64-v128:64:128\
                -a0:0:64-n32".to_strbuf()
          }

          abi::OsAndroid => {
            "e-p:32:32:32\
                -i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64\
                -f32:32:32-f64:64:64\
                -v64:64:64-v128:64:128\
                -a0:0:64-n32".to_strbuf()
          }

          abi::OsFreebsd => {
            "e-p:32:32:32\
                -i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64\
                -f32:32:32-f64:64:64\
                -v64:64:64-v128:64:128\
                -a0:0:64-n32".to_strbuf()
          }
        },

        target_triple: target_triple,

        cc_args: cc_args,
    };
}
