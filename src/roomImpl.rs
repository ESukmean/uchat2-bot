use super::uchat::*;
use async_trait::async_trait;
use log::*;

pub struct UBasicRoom {
	my_key: userKey,
	user_key: std::collections::HashMap<bytes::Bytes, userKey>,

	cmd_tx: Option<tokio::sync::mpsc::UnboundedSender<RoomControlCommand>>,
}

#[async_trait]
impl UChatRoom for UBasicRoom {
	fn on_create(&mut self, cmd_tx: tokio::sync::mpsc::UnboundedSender<RoomControlCommand>) {
		self.cmd_tx = Some(cmd_tx);
	}
	async fn on_receive(&mut self, ws: &mut wss_stream, data: Vec<Message>) {
		debug!("rcv {:?}", data);
	}
	async fn on_key(&mut self, ws: &mut wss_stream, data: userKey) {
		debug!("new key {:?}", data);

		self.user_key.insert(data.key.clone(), data);
	}
	async fn on_join(&mut self, my: &userKey) {
		debug!("joined {:?}", my);

		self.my_key = my.clone();
	}
}
impl UBasicRoom {
	pub fn new() -> Self {
		UBasicRoom {
			user_key: std::collections::HashMap::new(),
			my_key: userKey::empty(),
			cmd_tx: None,
		}
	}
}
