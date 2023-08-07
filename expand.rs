#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
#[allow(dead_code)]
pub struct Command {
    executable: String,
    #[builder(each = "arg", smth = "smth")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: Vec::new(),
            env: Vec::new(),
            current_dir: None,
        }
    }
}
pub struct CommandBuilder {
    executable: Option<String>,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
impl CommandBuilder {
    pub fn executable(&mut self, value: String) -> &mut Self {
        self.executable = Some(value);
        self
    }
    pub fn arg(&mut self, element: String) -> &mut Self {
        self.args.push(element);
        self
    }
    pub fn env(&mut self, element: String) -> &mut Self {
        self.env.push(element);
        self
    }
    pub fn current_dir(&mut self, value: String) -> &mut Self {
        self.current_dir = Some(value);
        self
    }
    pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
        Ok(Command {
            executable: self.executable.clone().ok_or("executable is required")?,
            args: self.args.clone(),
            env: self.env.clone(),
            current_dir: self.current_dir.clone(),
        })
    }
}
fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();
    match (&command.executable, &"cargo") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (
        &command.args,
        &<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new(["build", "--release"])),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
