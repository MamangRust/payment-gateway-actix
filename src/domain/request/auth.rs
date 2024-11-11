use serde::{
    Deserialize,
    Serialize
};

use regex::Regex;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), String> {

        if self.firstname.is_empty() {
            return Err("First name is required".to_string());
        }

        if self.lastname.is_empty() {
            return Err("Last name is required".to_string());
        }

        // Validate email format (simple check using regex)
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err("Invalid email format".to_string());
        }

        // Check if passwords match
        if self.password != self.confirm_password {
            return Err("Passwords do not match".to_string());
        }

        // Check if password length is sufficient (e.g., at least 8 characters)
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}


impl LoginRequest {
    pub fn validate(&self) -> Result<(), String> {
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err("Invalid email format".to_string());
        }

        // Check if password is empty
        if self.password.is_empty() {
            return Err("Password is required".to_string());
        }

        Ok(())
    }
}