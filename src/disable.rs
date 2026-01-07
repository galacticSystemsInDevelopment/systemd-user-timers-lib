pub fn disable(name: &str) -> Result<String, std::io::Error> {
    let timer_unit = format!("{}.timer", name);
    
    let output = std::process::Command::new("systemctl")
        .args(&["--user", "disable", &timer_unit])
        .output()?;

    if output.status.success() {
        // Concatenate the intent with the systemctl stdout
        Ok(format!(
            "Disabling timer: {} - Success: {}", 
            name, 
            String::from_utf8_lossy(&output.stdout).trim()
        ))
    } else {
        // Capture stderr for the error variant
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Disabling timer: {} - Failed: {}", name, error_message)
        ))
    }
}
