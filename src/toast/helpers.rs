use super::{Status, Toast};


/// Provides a new instance of Toasts with the danger style
pub fn danger(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Danger,
        with_close: false,
    }
}

/// Provides a new instance of Toasts with the primary style
pub fn primary(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Primary,
        with_close: false,
    }
}

/// Provides a new instance of Toasts with the secondary style
pub fn secondary(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Secondary,
        with_close: false,
    }
}

/// Provides a new instance of Toasts with the success style
pub fn success(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Success,
        with_close: false,
    }
}
