use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};

type Cmd = (&'static str, &'static str);
type CmdArray = &'static [Cmd];

// === LINUX ===
#[cfg(target_os = "linux")]
static SHELL: &str = "sh";

#[cfg(target_os = "linux")]
static BUILD_SCRIPTS: CmdArray = &[("forge/forge.sh", "build")("build.sh", "")("build", "")];

#[cfg(target_os = "linux")]
static RUN_SCRIPTS: CmdArray = &[("forge/forge.sh", "run")("run.sh", "")("run", "")];

// === WINDOWS ===
#[cfg(target_os = "windows")]
static SHELL: &str = "powershell";

#[cfg(target_os = "windows")]
static BUILD_SCRIPTS: CmdArray = &[
    ("./forge/forge.bat", "build"),
    ("./build.bat", ""),
    ("./build.ps1", ""),
];

#[cfg(target_os = "windows")]
static RUN_SCRIPTS: CmdArray = &[("./forge/forge.bat", "run"), ("./run.bat", ""), ("./run.ps1", "")];

fn run_exec(cmd: &Cmd) -> Result<String, String> {
    println!(">> {}", cmd.0);

    let process = Command::new(SHELL)
        .args([cmd.0, cmd.1])
        .output()
        .expect("Failed to run command");

    let info = String::from_utf8_lossy(&process.stdout).to_string();
    if process.status.success() {
        return Ok(info);
    } else {
        let msg = format!("{}:{}",info, String::from_utf8_lossy(&process.stderr).to_string());
        return Err(msg);
    }
}

fn main() -> ExitCode {
    let mut code = ExitCode::FAILURE; 

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No option specified, pass 'build' or 'run'.");
    } else if let Some(option) = args.last() {
        if let Some(set) = match option.as_str() {
            "build" => Some(&BUILD_SCRIPTS),
            "run" => Some(&RUN_SCRIPTS),
            _ => None,
        } {
            if let Some(cmd) = set.iter().find(|c| Path::new(c.0).exists()) {
                match run_exec(cmd) {
                    Ok(m) => {
                        println!("> {}", m);
                        code = ExitCode::SUCCESS;
                    }
                    Err(e) => {
                        println!("FAIL > {}", e);
                    }
                }
            } else {
                println!("Didn't find any usable script.");
                println!("Possible scripts are:");
                for ele in set.iter() {
                    println!("- {}",ele.0);
                }
            }
        } else {
            println!("Unknown option {} passed", option);
        }
    } else {
        println!("Could not read last passed option");
    }

    return code;
}
