#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    pub fn next(self) -> Self {
        match self {
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
            Light::Red => Light::Green,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
}

pub fn all_statuses() -> Vec<OrderStatus> {
    vec![
        OrderStatus::Pending,
        OrderStatus::Paid,
        OrderStatus::Shipped,
        OrderStatus::Delivered,
    ]
}

pub fn label(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Pending => "Pending",
        OrderStatus::Paid => "Paid",
        OrderStatus::Shipped => "Shipped",
        OrderStatus::Delivered => "Delivered",
    }
}
