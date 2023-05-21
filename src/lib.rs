#![feature(c_unwind)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn create_file(name: &str, content: &str) -> std::io::Result<()> {
	let normal_name : String = format!("{}{}", "garrysmod/", name);
	let normal_name_str : &str = &normal_name[..];

	let mut file = File::create(normal_name_str)?;
    file.write_all(content.as_bytes())?;

	Ok(())
}

pub fn remove_file(name: &str) -> std::io::Result<()> {
	let normal_name: String = format!("{}{}", "garrysmod/", name);
	let normal_name_str : &str = &normal_name[..];

	fs::remove_file(normal_name_str)?;
	Ok(())
}