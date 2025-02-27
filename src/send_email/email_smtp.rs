use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport, Message,
    Tokio1Executor,
};

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
        content: &str
    ) -> Result<(), String> {
        let creds = Credentials::new(
            self.sender.as_ref().to_string(),
            self.amtp_token.clone() 
            );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.qq.com")
            .unwrap()
            .port(465)
            .credentials(creds)
            .build();

        let from = self.sender.as_ref().parse().unwrap();
        let to = recipient.as_ref().parse().unwrap();

        let email = Message::builder()
            .from(from)
            .to(to)
            .header(ContentType::TEXT_HTML)
            .subject(subject)
            .body(content.to_string())
            .expect("Failed to build a message.");

        mailer.send(email).await
            .map_err(|e| {
                format!("Failed to send email {:?}", e)
            })?;

        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_email_function() {
        let sender_email = SubscriberEmail::parse(
            "tasmira@qq.com".to_string()
        ).expect("Failed to parse email");

        let amtp_token = "rxgyvqgheubxfdje".to_string();

        let sender = EmailSmtp::new(sender_email, amtp_token);

        let recipient_email = SubscriberEmail::parse(
            "2317424838@qq.com".to_string()
        ).expect("Failed to parse email.");

        let subject = "Email test";
        let content = "The message is from Rust.";

        claim::assert_ok!(
            sender.send_email(recipient_email, subject, content).await
        );
    }

}