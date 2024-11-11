pub fn payment_method_validator(payment_method: &str) -> bool {
    let payment_rules = vec![
        "alfamart",
        "indomart",
        "lawson",
        "dana",
        "ovo",
        "gopay",
        "linkaja",
        "jenius",
        "fastpay",
        "kudo",
        "bri",
        "mandiri",
        "bca",
        "bni",
        "bukopin",
        "e-banking",
        "visa",
        "mastercard",
        "discover",
        "american express",
        "paypal",
    ];

    let payment_method_lower = payment_method.to_lowercase();
    payment_rules.iter().any(|&rule| rule == payment_method_lower)
}