#[macro_export]
macro_rules! info {
    ($fmt:expr) => {{
        let timestamp = chrono::Utc::now();
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[32m[INFO]\x1b[0m: {}",
            timestamp,
            $fmt
        );
        let plain_msg = format!("{}:[INFO]: {}", timestamp, $fmt);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_info_sender().send(plain_msg);
     } };
    ($fmt:expr, $($args:expr),*) => {{
        let timestamp =  chrono::Utc::now();
        let message = format!($fmt, $($args),*);
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[32m[INFO]\x1b[0m: {}",
            timestamp,
            message
        );
        let plain_msg = format!("{}:[INFO]: {}", timestamp, message);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_info_sender().send(plain_msg);
    }};
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {{
        let timestamp =  chrono::Utc::now();
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[36m[DEBUG]\x1b[0m: {}",
            timestamp,
            $fmt
        );
        let plain_msg = format!("{}:[DEBUG]: {}", timestamp, $fmt);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_debug_sender().send(plain_msg);
    }};
    ($fmt:expr, $($args:expr),*) => {{
        let timestamp = chrono::Utc::now();
        let message = format!($fmt, $($args),*);
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[36m[DEBUG]\x1b[0m: {}",
            timestamp,
            message
        );
        let plain_msg = format!("{}:[DEBUG]: {}", timestamp, message);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_debug_sender().send(plain_msg);
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {{
        let timestamp =  chrono::Utc::now();
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[33m[WARNING]\x1b[0m: {}",
            timestamp,
            $fmt
        );
        let plain_msg = format!("{}:[WARNING]: {}", timestamp, $fmt);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_warn_sender().send(plain_msg);
    }};
    ($fmt:expr, $($args:expr),*) => {{
        let timestamp = chrono::Utc::now();
        let message = format!($fmt, $($args),*);
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[33m[WARNING]\x1b[0m: {}",
            timestamp,
            message
        );
        let plain_msg = format!("{}:[WARNING]: {}", timestamp, message);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_warn_sender().send(plain_msg);
    }};
}

#[macro_export]
macro_rules! err {
    ($fmt:expr) => {{
        let timestamp = chrono::Utc::now();
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[31m[ERROR]\x1b[0m: {}",
            timestamp,
            $fmt
        );
        let plain_msg = format!("{}:[ERROR]: {}", timestamp, $fmt);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_err_sender().send(plain_msg);
    }};
    ($fmt:expr, $($args:expr),*) => {{
        let timestamp = chrono::Utc::now();
        let message = format!($fmt, $($args),*);
        let colored_msg = format!(
            "\x1b[90m{}\x1b[0m:\x1b[31m[ERROR]\x1b[0m: {}",
            timestamp,
            message
        );
        let plain_msg = format!("{}:[ERROR]: {}", timestamp, message);
        println!("{}", colored_msg);
        let _ = $crate::logger::channels::get_err_sender().send(plain_msg);
    }};
}
