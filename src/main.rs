use service::service::{RegisterServiceInput, RunnableService};

mod service;

fn main() {
    let input = RegisterServiceInput {
        name: "AuthService".to_string(),
        path: "/Users/tahmid/projects/emil/EIS/authservicev2".to_string(),
        start_command: "npm".to_string(),
        stop_command: "stop".to_string(),
    };

    let service = RunnableService::new(input);
    service.run_service();

    println!("Hello, world!");
}
