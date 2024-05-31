use super::{Status, Toast};


pub fn danger(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Danger,
        with_close: false,
    }
}

pub fn primary(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Primary,
        with_close: false,
    }
}

pub fn secondary(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Secondary,
        with_close: false,
    }
}

pub fn success(title: &str) -> Toast {
    Toast {
        title: title.to_string(),
        body: String::new(),
        status: Status::Success,
        with_close: false,
    }
}
