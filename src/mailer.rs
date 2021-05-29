use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn mail(subject: String, message: String) {
    let email = Message::builder()
        .from(env::var("EMAIL_FROM").unwrap().parse().unwrap())
        .reply_to(env::var("EMAIL_TO").unwrap().parse().unwrap())
        .to(env::var("EMAIL_TO").unwrap().parse().unwrap())
        .subject(subject)
        .body(message)
        .unwrap();

    let creds = Credentials::new(env::var("SMTP_USER").unwrap(), env::var("SMTP_PASS").unwrap());

// Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&env::var("SMTP_RELAY").unwrap())
        .unwrap()
        .credentials(creds)
        .authentication(vec![Mechanism::Plain])
        .port(587)
        .build();

// Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}