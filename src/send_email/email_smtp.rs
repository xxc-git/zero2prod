use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::domain::SubscriberEmail;

pub struct EmailSmtp {
    sender: SubscriberEmail,
    amtp_token: String,
}

impl EmailSmtp {
    pub fn new(sender: SubscriberEmail, amtp_token: String) -> Self {
        Self {
            sender,
            amtp_token,
        }
    } 

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        content: String
    ) -> Result<(), String> {
        let creds = Credentials::new(
            self.sender.as_ref().to_string(),
            self.amtp_token 
            );

        let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .port(465)
            .credentials(creds)
            .build();

        let from = self.sender.as_ref().parse().unwrap();
        let to = recipient.as_ref().parse().unwrap();

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(content)
            .expect("Failed to build a message.");

        mailer.send(&email)?;

        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use crate::configuration::get_configuration;

    use super::*;

    #[tokio::test]
    async fn test_send_email_function() {
        let sender_email = SubscriberEmail::parse("tasmira@qq.com".to_string()).expect("Failed to parse email");
        let config = get_configuration().expect("Failed read conifuration files.");
        let sender = EmailSmtp::new(sender_email, config.) 

    }

}