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

fn main() -> amiquip::Result<()> {
    let sandbox_executable_path = std::env::var("JUDGESERVER_SANDBOX_EXECUTABLE").expect("You need to set JUDGESERVER_SANDBOX_EXECUTABLE env var properly");
    let judgeserver_production = match std::env::var("JUDGESERVER_PRODUCTION") {
        Ok(_) => true,
        Err(_) => false
    };

    match std::fs::read(&sandbox_executable_path) {
        Ok(_) => println!("Sandbox executable: {}", sandbox_executable_path.clone()),
        Err(_) => panic!("You need to set JUDGESERVER_SANDBOX_EXECUTABLE env var properly")
    };

    let mut connection: amiquip::Connection;

    if judgeserver_production {
        let production_amqp_url = std::env::var("JUDGESERVER_PRODUCTION_AMQP_URL").expect("You need to set JUDGESERVER_PRODUCTION_AMQP_URL env var properly");
        connection = amiquip::Connection::open(&production_amqp_url)?;
    }
    else {
        connection = amiquip::Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    }

    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare("judgeserver_job", amiquip::QueueDeclareOptions::default())?;

    let consumer = queue.consume(amiquip::ConsumerOptions::default())?;

    println!("Waiting for jobs. Press [Ctrl-C] to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            amiquip::ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received new message", i);
                println!("{}", body);
                consumer.ack(delivery)?;
            }

            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    Ok(())
}
