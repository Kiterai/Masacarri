use std::env;

use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

use crate::{
    error::AppResult,
    models::{Comment, Page},
};

pub async fn notify_reply(
    page: &Page,
    comment_replyto: &Comment,
    _comment_reply: &Comment,
) -> AppResult<()> {
    let replyto_addr = match &comment_replyto.mail_addr {
        Some(x) => x,
        None => return Ok(()),
    };

    let site_name = env::var("SITE_NAME").unwrap_or("Masacarri".to_string());
    let mailaddr_from = env::var("SMTP_MAILADDR")?;
    let smtp_encryption = env::var("SMTP_ENCRYPTION")?;
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port = env::var("SMTP_PORT")?;
    let smtp_user = env::var("SMTP_USER")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;

    let email = Message::builder()
        .from(mailaddr_from.parse()?)
        .to(Mailbox::new(None, replyto_addr.parse()?))
        .subject(format!("{}: Your comment got a reply", site_name))
        .body(format!("Check reply to your comment: {}", page.page_url))?;

    let cred = Credentials::new(smtp_user, smtp_password);

    let mailer = match smtp_encryption.as_str() {
        "starttls" => SmtpTransport::starttls_relay(&smtp_host),
        _ => SmtpTransport::relay(&smtp_host),
    }
    .unwrap()
    .port(smtp_port.parse()?)
    .credentials(cred)
    .build();

    mailer.send(&email)?;

    Ok(())
}
