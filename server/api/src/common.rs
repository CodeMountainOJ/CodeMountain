/*
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::config::get;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Serialize;
use lettre::message::MultiPart;

#[derive(Serialize)]
pub struct StatusPayload {
    pub success: bool,
    pub message: Option<String>,
}

pub fn send_mail(receiver: &str, sender: &str, body: &str, subject: &str) -> bool {
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(body.to_string().clone(), body.to_string()))
        .unwrap();

    let creds = Credentials::new(get::<String>("SMTP_EMAIL"), get::<String>("SMTP_PASSWORD"));

    let mailer = SmtpTransport::relay(&get::<String>("SMTP_SERVER"))
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn send_password_reset_email(
    receiver: &str,
    sender: &str,
    subject: &str,
    reset_token: &str,
    username: &str,
    nickname: &str,
) -> bool {
    let mut email_body = std::fs::read_to_string("email_templates/password_reset.html").unwrap();

    email_body = email_body.replace("{{lastname}}", &nickname);
    email_body = email_body.replace("{{username}}", &username);
    email_body = email_body.replace("{{otp}}", &reset_token);

    send_mail(receiver, sender, &email_body, subject)
}
