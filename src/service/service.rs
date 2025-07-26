use std::{
    fs::File,
    process::{Command, Stdio},
    slice::RChunksMut,
};

pub struct RegisterServiceInput {
    pub name: String,
    pub path: String,
    pub start_command: String,
    pub stop_command: String,
}

pub struct RunnableService {
    pub name: String,
    pub path: String,
    pub start_command: String,
    pub stop_command: String,
}

impl RunnableService {
    pub fn new(input: RegisterServiceInput) -> RunnableService {
        return RunnableService {
            name: input.name,
            path: input.path,
            start_command: input.start_command,
            stop_command: input.stop_command,
        };
    }

    pub fn run_service(&self) {
        let output_log_file = File::create("output.log").expect("failed to create file");
        let error_log_file = File::create("error.log").expect("failed to create file");
        let child = Command::new(&self.start_command)
            .arg("run")
            .arg("start:dev")
            .current_dir(&self.path)
            .stdout(Stdio::from(output_log_file))
            .stderr(Stdio::from(error_log_file))
            .spawn()
            .expect("Failed to spawn process");

        println!("Spawned process with id {}", child.id());
    }
}
