use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler, QueueProperties};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        let _ = (ten_millis, now);

        thread::sleep(ten_millis);

        println!(
            "In Fernando Lawrence's Computer [2406422986]. Message received: {:?}",
            message
        );

        Ok(())
    }

    fn get_handler_action(&self) -> String {
        "user_created".to_owned()
    }
}

fn main() {
    let listener =
        CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_owned()).unwrap();

    _ = listener
        .listen(
            "user_created".to_owned(),
            UserCreatedHandler {},
            QueueProperties {
                auto_delete: false,
                durable: false,
                use_dead_letter: true,
            },
        )
        .unwrap();

    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
