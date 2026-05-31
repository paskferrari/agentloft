use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::Duration;
use tauri::Manager;

struct ServerProcess(Mutex<Option<Child>>);

fn find_node() -> Option<String> {
    for candidate in &["node", "/usr/local/bin/node", "/opt/homebrew/bin/node"] {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return Some(candidate.to_string());
        }
    }
    None
}

fn wait_for_server(port: u16, retries: u32) -> bool {
    for _ in 0..retries {
        if ureq::get(&format!("http://127.0.0.1:{}/health", port))
            .timeout(Duration::from_secs(1))
            .call()
            .is_ok()
        {
            return true;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ServerProcess(Mutex::new(None)))
        .setup(|app| {
            let resource_dir = app.path().resource_dir().unwrap_or_default();
            let cli_path = resource_dir.join("cli.js");

            let node = find_node().expect("Node.js non trovato — installa Node.js");

            let child = Command::new(&node)
                .arg(&cli_path)
                .arg("--port")
                .arg("3200")
                .spawn()
                .expect("Impossibile avviare il server AgentLoft");

            *app.state::<ServerProcess>().0.lock().unwrap() = Some(child);

            if !wait_for_server(3200, 20) {
                eprintln!("[AgentLoft] Server non avviato in tempo");
            }

            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::Destroyed = event {}
        })
        .run(tauri::generate_context!())
        .expect("Errore avvio applicazione Tauri");
}
