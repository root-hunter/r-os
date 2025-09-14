use crate::{core::shell::command::ShellCommand, kernel::Kernel};

pub struct MkDirCommand;

impl ShellCommand for MkDirCommand {
    fn execute(&self, k: &mut Kernel, args: Vec<&str>) -> String {
        if args.is_empty() {
            return "mkdir: missing operand".into();
        }
        let dir_name = args[0];

        k.fs.create_folder(dir_name);

        format!("Directory '{}' created.", dir_name)
    }
}