use std::env;

use dotenv::dotenv;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub struct MailService<'a> {
    pub receiver: &'a str,
    pub subject: &'a str,
    pub html_body: &'a str,
}

impl<'a> MailService<'a> {
    pub fn build_message(&'a mut self) -> MessageBuilder<'a> {
        MessageBuilder::new()
            .from(("FileVerify", "abyerkingaliev@gmail.com"))
            .to(self.receiver)
            .subject(self.subject)
            .html_body(self.html_body)
    }

    pub async fn send(&'a mut self) {
        dotenv().ok();
        let smtp_host = env::var("SMTP_HOST").expect("Failed get SMTP host");
        let smtp_port = env::var("SMTP_PORT")
            .expect("Failed get SMTP port")
            .parse::<u16>()
            .expect("Failed convert smptp_port to u16");
        let smtp_user = env::var("SMTP_USER").expect("Failed get SMTP user");
        let smtp_password = env::var("SMTP_PASSWORD").expect("Failed get SMTP password");
        let mut mail_connection = match SmtpClientBuilder::new(smtp_host, smtp_port)
            .implicit_tls(true)
            .credentials((smtp_user, smtp_password))
            .connect()
            .await
        {
            Ok(connection) => connection,
            Err(_) => panic!("Failed SMTP connection"),
        };
        let message = self.build_message();
        mail_connection
            .send(message)
            .await
            .expect("Error during send message");
    }
}
