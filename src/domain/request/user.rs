use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub noc_transfer: Option<String>,
    pub confirm_password: String,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), String> {
        // Check if the first name is not empty
        if self.firstname.trim().is_empty() {
            return Err("First name is required".to_string());
        }

        // Check if the last name is not empty
        if self.lastname.trim().is_empty() {
            return Err("Last name is required".to_string());
        }

        // Validate email format (simple check using regex)
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            return Err("Invalid email format".to_string());
        }

        // Ensure passwords match
        if self.password != self.confirm_password {
            return Err("Passwords do not match".to_string());
        }

        // Ensure password is at least 8 characters long
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
}

impl UpdateUserRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_none()
            && self.password.is_none()
            && self.firstname.is_none()
            && self.lastname.is_none()
        {
            return Err("At least one field must be provided for update".to_string());
        }

        // Validate firstname if provided
        if let Some(ref firstname) = self.firstname {
            if firstname.trim().is_empty() {
                return Err("First name cannot be empty".to_string());
            }
        }

        // Validate lastname if provided
        if let Some(ref lastname) = self.lastname {
            if lastname.trim().is_empty() {
                return Err("Last name cannot be empty".to_string());
            }
        }

        // Validate email if provided
        if let Some(ref email) = self.email {
            let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
            if !email_regex.is_match(email) {
                return Err("Invalid email format".to_string());
            }
        }

        // Validate password if provided
        if let Some(ref password) = self.password {
            if password.len() < 8 {
                return Err("Password must be at least 8 characters long".to_string());
            }
        }

       
        if let (Some(ref password), Some(ref confirm_password)) =
            (self.password.as_ref(), self.confirm_password.as_ref())
        {
            if password != confirm_password {
                return Err("Passwords do not match".to_string());
            }
        }

        Ok(())
    }
}
