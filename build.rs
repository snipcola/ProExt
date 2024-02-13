// Copyright (c) 2024 Snipcola
// SPDX-License-Identifier: MIT

extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}