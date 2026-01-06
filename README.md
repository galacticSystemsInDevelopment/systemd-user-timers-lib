# systemd-user-timers-lib
Systemd User Timers Library

This is the library. For the app, see https://github.com/galacticSystemsInDevelopment/systemd-user-timers.  

### Add to your Rust project:
```bash
cargo add systemd-user-timers-lib
```

## Modules
### Add example:
```rust
use systemd_user_timers_lib::timers::Timer;
fn main() {
  let timer = Timer {
        name: "notify".to_string(),
        description: "5 Minute Notify Timer".to_string(),
        schedule: "5min".to_string(),
        executable: "/home/user/notify.sh",
        exec_if_missed: false,
        single_use: false,     
        recurring: false,       
        on_calendar: false,    
        from_boot: false,      
        normal_service: false,   
        service: None,         
        already_made_service: false, 
        enable_at_login: false, 
        start_after_create: true, 
  };
  println!("{}", systemd_user_timers_lib::add_timer:add_timer(timer));

}
```
### Start example:
```rust
fn main() {
  let output = systemd_user_timers_lib::start::start("notify").unwrap_or_else(|e| {
        eprintln!("Error listing timers: {}", e);
        String::new()  // Returning an empty string on error
    });

    if !output.is_empty() {
        println!("{}", output);  // Print the output
    }
}
```

### Stop example:
```rust
fn main() {
      println!("{}", systemd_user_timers_lib::stop::stop("notify").unwrap());
}
```

### List example
```rust
fn main() {
      let output = systemd_user_timers_lib::list_timers::list_timers().unwrap_or_else(|e| {
        eprintln!("Error listing timers: {}", e);
        String::new()  // Returning an empty string on error
    });

    if !output.is_empty() {
        println!("{}", output);  // Print the output
    }
}
```
