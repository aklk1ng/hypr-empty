use anyhow::Result;
use hyprland::shared::WorkspaceType;
use serde::Deserialize;
use std::thread;
use std::time::Duration;
use std::{fs::File, io::Read, process::Command};

#[derive(Deserialize)]
struct Cmd {
    workspace: String,
    command: String,
    args: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Cmds {
    components: Vec<Cmd>,
}

use hyprland::{
    data::Workspace, event_listener::EventListenerMutable as EventListener, shared::HyprDataActive,
};

fn main() -> Result<()> {
    loop {
        let mut event_listener = EventListener::new()?;

        let home_dir = std::env::var("HOME")?;
        let mut config_file = File::open(format!("{home_dir}/.config/hypr-empty/config.toml"))?;

        let mut config = String::new();
        config_file.read_to_string(&mut config)?;

        let cmds: Cmds = toml::from_str(&config)?;

        event_listener.add_workspace_added_handler(move |id, state| {
            if let WorkspaceType::Regular(_ws) = &state.active_workspace {
                let mut cmds = cmds
                    .components
                    .iter()
                    .filter(|&cmd| *cmd.workspace == id.to_string());
                if let Some(cmd) = &cmds.next() {
                    let ws = Workspace::get_active().unwrap();
                    if ws.windows == 0 {
                        Command::new(&cmd.command)
                            .args(cmd.args.clone().as_deref().unwrap_or_default())
                            .spawn()
                            .unwrap();
                    }
                }
            }
        });

        let mut cnt = 0;
        match event_listener.start_listener() {
            Err(_) => {
                cnt += 1;
                thread::sleep(Duration::from_secs(1));
            }
            Ok(_) => break,
        }
        if cnt >= 9 {
            break;
        }
    }
    Ok(())
}
