use log::*;
use super::uchat::*;
use async_trait::async_trait;

pub struct UBasicRoom {
	my_key: userKey,
	user_key: std::collections::HashMap<bytes::Bytes, userKey>,
}

#[async_trait]
impl UChatRoom for UBasicRoom {
	async fn on_receive(&mut self, ws: &mut wssStream, data: Vec<Message>) {
		debug!("rcv {:?}", data);
	}
	async fn on_key(&mut self, ws: &mut wssStream, data: userKey) {
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
			my_key: userKey::empty()
		}
	}

}