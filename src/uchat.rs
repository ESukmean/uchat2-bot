use bytes::*;
use log::*;
pub use protocol::*;

mod protocol;
use futures_util::{SinkExt, StreamExt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UChatAuthLevel {
	Admin,    //왕관
	SubAdmin, //은색 왕관
	Member,
	Guest,
	None,
}
impl ToString for UChatAuthLevel {
	fn to_string(&self) -> String {
		match &self {
			UChatAuthLevel::None => "".to_string(),
			UChatAuthLevel::Admin => "admin".to_string(),
			UChatAuthLevel::SubAdmin => "subadmin".to_string(),
			UChatAuthLevel::Member => "member".to_string(),
			UChatAuthLevel::Guest => "guest".to_string(),
		}
	}
}

pub struct JoinConfig {
	pub room: String,
	pub token: Option<String>,
	pub nick: Option<String>,
	pub id: Option<String>,
	pub level: Option<String>,
	pub auth: UChatAuthLevel,
	pub icon: Option<String>,
	pub nickcon: Option<String>,
	pub other: Option<String>,
	pub password: Option<String>,
	pub cache_token: Option<String>,
	pub profile_image: Option<String>,
}
impl JoinConfig {
	pub fn new(room: String) -> Self {
		JoinConfig {
			room,
			token: None,
			nick: None,
			id: None,
			level: None,
			auth: UChatAuthLevel::None,
			icon: None,
			nickcon: None,
			other: None,
			password: None,
			cache_token: None,
			profile_image: None,
		}
	}

	pub fn token(mut self, token: Option<String>) -> Self {
		self.token = token;
		return self;
	}
	pub fn nick(mut self, nick: Option<String>) -> Self {
		self.nick = nick;
		return self;
	}
	pub fn id(mut self, id: Option<String>) -> Self {
		self.id = id;
		return self;
	}
	pub fn level(mut self, level: Option<String>) -> Self {
		self.level = level;
		return self;
	}
	pub fn auth(mut self, auth: UChatAuthLevel) -> Self {
		self.auth = auth;
		return self;
	}
	pub fn icon(mut self, icon: Option<String>) -> Self {
		self.icon = icon;
		return self;
	}
	pub fn nickcon(mut self, nickcon: Option<String>) -> Self {
		self.nickcon = nickcon;
		return self;
	}
	pub fn other(mut self, other: Option<String>) -> Self {
		self.other = other;
		return self;
	}
	pub fn password(mut self, password: Option<String>) -> Self {
		self.password = password;
		return self;
	}
	pub fn client_token(mut self, token: Option<String>) -> Self {
		self.cache_token = token;
		return self;
	}
	pub fn profile_image(mut self, profile_image: Option<String>) -> Self {
		self.profile_image = profile_image;
		return self;
	}

	fn implode(&self, token: &str, time: &String) -> String {
		// ksort => ["auth"]["icons"]["id"]["level"]["nick"]["nickcon"]["other"]["room"]["time"]["token"]
		let mut result = String::with_capacity(128);

		match &self.auth {
			UChatAuthLevel::None => (),
			auth => {
				result.push_str(&auth.to_string());
				result.push_str(token);
			}
		};
		if let Some(v) = self.icon.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}
		if let Some(v) = self.id.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}
		if let Some(v) = self.level.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}
		if let Some(v) = self.nick.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}
		if let Some(v) = self.nickcon.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}
		if let Some(v) = self.other.as_ref() {
			result.push_str(&v.to_string());
			result.push_str(token);
		}

		{
			result.push_str(&self.room);
			result.push_str(token);
		}
		{
			result.push_str(&format!("{}", time));
			result.push_str(token);
		}
		{
			result.push_str(token);
		}

		return result;
	}

	#[inline]
	pub fn build(&self) -> uMessage {
		return self.build_with_time(std::time::Duration::new(0, 0));
	}
	pub fn build_with_time(&self, time_delta: std::time::Duration) -> uMessage {
		use rand::Rng;
		use std::time::SystemTime;

		let session: String = rand::thread_rng()
			.sample_iter(rand::distributions::Alphanumeric)
			.map(|b| std::char::from_u32(b as u32).unwrap())
			.take(32)
			.collect();

		let mut time: Option<String> = None;

		let hash = match self.token.as_ref() {
			None => "".to_string(),
			Some(token) => {
				let t = format!(
					"{}",
					(SystemTime::now()
						.duration_since(SystemTime::UNIX_EPOCH)
						.unwrap() + time_delta)
						.as_secs()
				);
				let hash_base = self.implode(token, &t);

				time = Some(t);
				format!("{:x}", md5::compute(hash_base.as_bytes()))
			}
		};

		//jtest1nicknameidlevelauthiconnickcon6fe01303583a9e71e6ece678a4f268ef8TkDUyBiousTta7Ko7qwyIMU6fqgTl9lutf-81604136208
		//this.socket.send(['j', this.id, data.nick, data.id, data.level, (data.auth||''), data.icons, data.nickcon, data.other, data.hash, session, ua.charset, data.time, this.installData.password, cache['client_token'], data.profileimg]);
		let mut buf: uMessage = uMessage::new();
		buf.push(Message::Text("j".to_string()));
		buf.push(Message::Text(self.room.clone()));
		buf.push(Message::Text(self.nick.clone().unwrap_or(String::new())));
		buf.push(Message::Text(self.id.clone().unwrap_or(String::new())));
		buf.push(Message::Text(self.level.clone().unwrap_or(String::new())));
		buf.push(Message::Text(self.nick.clone().unwrap_or(String::new())));
		buf.push(Message::Text(self.icon.clone().unwrap_or(String::new())));
		buf.push(Message::Text(self.nickcon.clone().unwrap_or(String::new())));
		if let Some(other) = self.other.as_ref() {
			buf.push(Message::Text(other.clone()));
		} else {
			buf.push(Message::None);
		}
		buf.push(Message::Text(hash));
		buf.push(Message::Text(session));
		buf.push(Message::Text("utf-8".to_string()));
		buf.push(Message::Text(time.unwrap_or(String::new())));
		if let Some(password) = self.password.as_ref() {
			buf.push(Message::Text(password.clone()));
		} else {
			buf.push(Message::None);
		}
		if let Some(cache_token) = self.cache_token.as_ref() {
			buf.push(Message::Text(cache_token.clone()));
		} else {
			buf.push(Message::None);
		}
		if let Some(profile_image) = self.profile_image.as_ref() {
			buf.push(Message::Text(profile_image.clone()));
		} else {
			buf.push(Message::None);
		}

		return buf;
	}
}

pub enum RoomControlCommand {}

#[derive(Clone, Debug, PartialEq)]
enum CurrentState {
	None,
	JoinProcess,
	Joined,
}

pub type wss_stream =
	tokio_tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>>;

pub struct UChatRoomProc<T> {
	access_info: JoinConfig,
	my_info: protocol::userKey,

	ws: Option<
		tokio_tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>>,
	>,

	cmd_rx: tokio::sync::mpsc::UnboundedReceiver<RoomControlCommand>,

	state: CurrentState,
	room: T,
}

impl<T> UChatRoomProc<T>
where
	T: UChatRoom + Send,
{
	pub fn new(access_info: JoinConfig, mut room: T) -> Self {
		let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
		room.on_create(cmd_tx);

		Self {
			access_info,
			my_info: protocol::userKey::empty(),
			ws: None,

			cmd_rx,

			state: CurrentState::None,
			room,
		}
	}

	pub async fn connect(&mut self) -> Result<(), String> {
		return self
			.connect_with_uri(url::Url::parse("ws://sp-worker.uchat.io:80/").unwrap())
			.await;
	}
	pub async fn connect_with_uri(&mut self, uri: url::Url) -> Result<(), String> {
		let tls_connector = native_tls::TlsConnector::new().map_err(|e| e.to_string())?;
		let tls_conn = tokio_native_tls::TlsConnector::from(tls_connector);

		let tcp = tokio::net::TcpStream::connect((uri.host_str().unwrap(), uri.port().unwrap_or(443)))
			.await
			.map_err(|e| e.to_string())?;
		let tls = tls_conn
			.connect(uri.host_str().unwrap(), tcp)
			.await
			.map_err(|e| e.to_string())?;

		debug!("tls: {:?}", tls);
		self.ws.replace(
			tokio_tungstenite::client_async(uri.as_str(), tls)
				.await
				.map_err(|e| e.to_string())?
				.0,
		);

		return Ok(());
	}

	pub async fn process(&mut self) -> Result<(), String> {
		if self.ws.is_none() {
			return Err("웹소켓이 연결되지 않았습니다.".to_string());
		}
		let mut ws: wss_stream = self.ws.take().unwrap();
		self.join(&mut ws).await?;

		let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(30));

		loop {
			tokio::select! {
				cmd = self.cmd_rx.recv() => {

				}
				rcv = ws.next() => {
					if rcv.is_none() {
						self.room.on_closed().await;
						return Ok(());
					}

					let rcv = rcv.unwrap();
					match rcv {
						Err(_) => {

						},
						Ok(item) => {
							let mut item: BytesMut = match item {
								tokio_tungstenite::tungstenite::Message::Binary(b) => {
									if b.len() == 0 { continue; }
									BytesMut::from(&b[..])
								},
								tokio_tungstenite::tungstenite::Message::Text(t) => {
									BytesMut::from(t.as_bytes())
								},
								tokio_tungstenite::tungstenite::Message::Ping(_) => {
									ws.send(tokio_tungstenite::tungstenite::Message::Pong(Vec::new())).await.map_err(|e| e.to_string())?;
									continue;
								},
								_ => { continue; }
							};

							for line in delimit_line(&mut item) {
								let line = uMessage::unpack(line).unwrap();
								if line.len() == 0 { continue; }

								self.process_line(&mut ws, line).await;
							}
						}
					}
				}
				_ = ping_interval.tick() => {
					debug!("핑 전송 시도");

					if let Err(_) = ws.send(tokio_tungstenite::tungstenite::Message::Binary(b"p\n".to_vec())).await {
						self.room.on_raw_err("ping 전송 실패".to_string()).await;
					}
				}
			}
		}

		return Ok(());
	}

	async fn process_line(&mut self, ws: &mut wss_stream, mut data: Vec<Message>) {
		if let Some(key) = data.get(0) {
			match key {
				Message::Text(v) if v == "k" => {
					if let Ok(k) = protocol::userKey::unpack(data) {
						self.room.on_key(ws, k).await;
					}

					return;
				}
				Message::Text(v) if v == "o" => {
					if data.len() < 1 {
						return;
					}
					
					if let Ok(nick) = data.remove(1).unwrap_text() { 
						self.room.on_out(nick);
					} else {
						self.room.on_raw_err("접속 종료한 유저 처리에 실패".to_string());
					}
				}
				Message::Text(v) if v == "i1" => {
					self.state = CurrentState::JoinProcess;
					return;
				}
				Message::Text(v) if v == "k" && self.state == CurrentState::JoinProcess => {
					if let Ok(k) = protocol::userKey::unpack(data) {
						self.my_info = k;

						self.room.on_connected(&self.my_info).await;
					} else {
						self.room.on_raw_err("내 정보 수신 실패".to_string()).await;
					}

					return;
				}
				Message::Text(v) if v == "i2" => {
					self.state = CurrentState::Joined;
					ws.send(tokio_tungstenite::tungstenite::Message::Binary(
						"commanduserList'\n".as_bytes().to_vec(),
					))
					.await;
					ws.send(tokio_tungstenite::tungstenite::Message::Binary(
						"commandio1\n".as_bytes().to_vec(),
					))
					.await;
					ws.send(tokio_tungstenite::tungstenite::Message::Binary(
						"commandchatList\n".as_bytes().to_vec(),
					))
					.await;
					ws.send(tokio_tungstenite::tungstenite::Message::Binary(
						"commandnoticeList\n".as_bytes().to_vec(),
					))
					.await;

					return;
				}
				Message::Text(v) if v == "ERR" => {
					self.room.on_uchat_err(data).await;

					return;
				}
				_ => {}
			}
		}

		self.room.on_receive(ws, data).await;
	}
	async fn join(&self, ws: &mut wss_stream) -> Result<(), String> {
		debug!(
			"{:?}",
			String::from_utf8(self.access_info.build().pack().to_vec())
		);
		return ws
			.send(tokio_tungstenite::tungstenite::Message::Binary(
				(&self.access_info.build().pack()).to_vec(),
			))
			.await
			.map_err(|e| e.to_string());
	}
}

use async_trait::async_trait;
#[async_trait]
pub trait UChatRoom {
	fn on_create(&mut self, cmd_tx: tokio::sync::mpsc::UnboundedSender<RoomControlCommand>);
	async fn on_receive(&mut self, ws: &mut wss_stream, data: Vec<Message>);
	async fn on_key(&mut self, ws: &mut wss_stream, data: userKey);
	async fn on_connected(&mut self, my: &userKey);
	async fn on_out(&mut self, nick: String);
	async fn on_raw_err(&mut self, err: String) {
		error!("{:?}", err);
	}
	async fn on_uchat_err(&mut self, data: Vec<Message>) {}
	async fn on_closed(&mut self) {
		warn!("stream closed");
	}
}
impl<T> Drop for UChatRoomProc<T> {
	fn drop(&mut self) {
		self.cmd_rx.close();
		
		let mut waker = futures::task::noop_waker();
		let mut ctx = std::task::Context::from_waker(&waker);
		while let std::task::Poll::Ready(result) = self.cmd_rx.poll_recv(&mut ctx) {
			if result.is_none() { break }
		}
	}
}
