#![allow(unused)]

mod add_timer;
mod disable;
mod enable;
mod list_timers;
mod remove_timer;
mod show_status;
mod start;
mod stop;
mod timers;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timers::Timer;

    #[test]
    fn test_add_timer() {
        let timer = Timer {
            name: "test_timer".to_string(),
            description: Some("A test timer".to_string()),
            schedule: "OnCalendar=*".to_string(),
            executable: Some("/path/to/executable".to_string()),
            exec_if_missed: true,
            single_use: true,
            recurring: false,
            on_calendar: true,
            from_boot: false,
            normal_service: true,
            service: None,
            already_made_service: false,
            enable_at_login: true,
            start_after_create: false,
        };

        let result = add_timer(timer);
        assert!(result.contains("Adding timer:"));
        assert!(result.contains("Success:"));
    }

    #[test]
    fn test_enable() {
        // Assuming the timer "test_timer" exists for this test
        let result = enable("test_timer");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Enabled test_timer.timer (user)"));
    }

    #[test]
    fn test_disable() {
        // Assuming the timer "test_timer" exists for this test
        let result = disable("test_timer");
        assert!(result.is_ok());
        // You can inspect stdout if needed
    }

    #[test]
    fn test_remove_timer() {
        let deletion_info = remove_timer::DeletionInfo {
            name: "test_timer".to_string(),
            remove_service: true,
        };
        
        let result = remove_timer(deletion_info);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Successfully removed timer:"));
    }

    #[test]
    fn test_list_timers() {
        let result = list_timers().unwrap();
        assert!(result.contains(".timer"));
    }

    #[test]
    fn test_show_status() {
        let result = show_status("test_timer").unwrap();
        assert!(result.contains("Active:"));
    }

    #[test]
    fn test_start() {
        let result = start("test_timer").unwrap();
        assert!(result.contains("Starting timer: test_timer"));
    }

    #[test]
    fn test_stop() {
        let result = stop("test_timer").unwrap();
        assert!(result.contains("Stopped timer: test_timer"));
    }
}
