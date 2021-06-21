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
use lettre::transport::smtp::{authentication::Credentials, Error};
use lettre::{Message, SmtpTransport, Transport};

pub fn mail(body: String, receiver: String, subject: String) -> Result<(), Error> {
    let smtp_email = std::env::var("SMTP_EMAIL").unwrap();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap();
    let smtp_server = std::env::var("SMTP_SERVER").unwrap();

    let email = Message::builder()
        .from(smtp_email.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(smtp_email, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
