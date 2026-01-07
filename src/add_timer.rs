use std::process::Command;
use std::fs;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use crate::timers::Timer;


pub fn add_timer(timer: Timer) -> Result<String, std::io::Error> {
    let mut log_output = String::new();
    
    // Defaulting basic required strings
    let timer_name = timer.name.as_deref().unwrap_or("default_timer");
    let schedule = timer.schedule.as_deref().unwrap_or("*:*:*");
    
    // Handling Option<bool> with unwrap_or
    let recurring = timer.recurring.unwrap_or(false);
    let remain_line = if recurring { "RemainAfterElapse=yes" } else { "RemainAfterElapse=no" };

    let description_line = timer.description.as_ref()
        .map(|d| format!("Description={}", d))
        .unwrap_or_default();

    let (service_contents_opt, service_unit_name) = if timer.already_made_service.unwrap_or(false) {
        (None, timer.service.clone().unwrap_or_else(|| timer_name.to_string()))
    } else {
        let exe = timer.executable.as_ref().expect("executable required to create service");
        let esc = exe.replace('\'', "'\\''");
        let exec_start_line = format!("ExecStart=/bin/sh -c '{}'", esc);

        let is_normal = timer.normal_service.unwrap_or(false);
        let service_type = if is_normal { "Type=simple" } else { "Type=oneshot" };
        let restart = if is_normal { "Restart=on-failure" } else { "Restart=no" };

        let mut svc = String::new();
        svc.push_str("[Unit]\n");
        if !description_line.is_empty() { svc.push_str(&format!("{}\n", description_line)); }
        svc.push_str("\n[Service]\n");
        svc.push_str(&format!("{}\n{}\n{}\n", service_type, exec_start_line, restart));
        svc.push_str("\n[Install]\nWantedBy=default.target\n");

        let unit_name = timer.service.clone().unwrap_or_else(|| timer_name.to_string());
        (Some(svc), unit_name)
    };

    let persistent = if timer.exec_if_missed.unwrap_or(false) { "Persistent=yes" } else { "Persistent=no" };
    
    let trigger = if timer.on_calendar.unwrap_or(false) {
        format!("OnCalendar={}", schedule)
    } else if timer.from_boot.unwrap_or(false) {
        format!("OnBootSec={}", schedule)
    } else if recurring {
        format!("OnUnitActiveSec={}", schedule)
    } else {
        format!("OnActiveSec={}", schedule)
    };

    let timer_contents = format!(
        "[Unit]\nDescription=Timer for {}\n\n[Timer]\nUnit={}.service\n{}\n{}\n{}\n\n[Install]\nWantedBy=timers.target\n",
        timer_name, service_unit_name, trigger, persistent, remain_line
    );

    // XDG directory resolution
    let unit_dir = match env::var("XDG_CONFIG_HOME").ok().filter(|s| !s.is_empty()) {
        Some(xdg) => format!("{}/systemd/user", xdg),
        None => match env::var("HOME").ok().filter(|s| !s.is_empty()) {
            Some(home) => format!("{}/.config/systemd/user", home),
            None => "/tmp/systemd/user".to_string(), // Safer fallback than raw "~"
        },
    };

    fs::create_dir_all(&unit_dir)?;

    let timer_path = format!("{}/{}.timer", unit_dir, timer_name);
    if let Some(svc) = service_contents_opt {
        let svc_path = format!("{}/{}.service", unit_dir, service_unit_name);
        fs::write(svc_path, svc)?;
    }
    fs::write(&timer_path, timer_contents)?;

    // Handle single-use promise
    if timer.single_use.unwrap_or(false) {
        let su_path = format!("{}/.single_use.txt", unit_dir);
        let mut f = OpenOptions::new().create(true).append(true).open(su_path)?;
        writeln!(f, "{}", timer_name)?;
    }

    Command::new("systemctl").args(&["--user", "daemon-reload"]).status()?;

    // Enable/Start logic
    let enable = timer.enable_at_login.unwrap_or(false);
    let start = timer.start_after_create.unwrap_or(false);

    let mut args = vec!["--user"];
    if enable && start { args.extend(&["enable", "--now"]); }
    else if enable { args.push("enable"); }
    else if start { args.push("start"); }
    else {
        log_output.push_str("Timer created (manual start required).");
        return Ok(log_output);
    }

    let unit_file = format!("{}.timer", timer_name);
    args.push(&unit_file);

    if Command::new("systemctl").args(&args).status()?.success() {
        log_output.push_str(&format!("Successfully processed {}", unit_file));
    }

    Ok(log_output)
}
