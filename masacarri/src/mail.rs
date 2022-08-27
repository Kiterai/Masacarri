use std::env;

use diesel::{prelude::*, r2d2::ConnectionManager};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use r2d2::PooledConnection;

use crate::{
    error::AppResult,
    models::{Comment, Page}, bgtask::BgActor, db::{MainDbConnection, MainDbPooledConnection},
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

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct MailNotifyTask {
    pub id_replyto: uuid::Uuid,
    pub conn: MainDbPooledConnection,
    pub comment_new: Comment,
}

impl actix::Handler<MailNotifyTask> for BgActor {
    type Result = ();

    fn handle(&mut self, task: MailNotifyTask, _: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        const NOTIFY_RETRY_NUMBER: i32 = 5;
        
        let comment_replyto = comments.filter(id.eq(task.id_replyto)).first::<Comment>(&task.conn);
        let comment_replyto = match comment_replyto {
            Ok(x) => x,
            Err(_) => return,
        };
        let page = crate::schema::pages::dsl::pages.filter(crate::schema::pages::id.eq(task.comment_new.page_id)).first::<crate::models::Page>(&task.conn);
        let page = match page {
            Ok(x) => x,
            Err(_) => return,
        };

        for _ in 0..NOTIFY_RETRY_NUMBER {
            let res = actix::System::new().block_on(notify_reply(&page, &comment_replyto, &task.comment_new));
            match res {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                },
            };

            return;
        }
    }
}
