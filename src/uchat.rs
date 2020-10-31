pub enum UChatAuthLevel {
	Admin, //왕관
	SubAdmin, //은색 왕관
	Member,
	Guest,
	None
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

pub struct AccessConfig {
	room: String,
	token: Option<String>,
	nick: Option<String>,
	id: Option<String>,
	level: Option<String>,
	auth: UChatAuthLevel,
	icon: Option<String>,
	nickcon: Option<String>,
	other: Option<String>,
	password: Option<String>,
	cache_token: Option<String>,
	profile_image: Option<String>,
}
impl AccessConfig {
	pub fn new(room: String) -> Self {
		AccessConfig {
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
			profile_image: None
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
	pub fn icons(mut self, icon: Option<String>) -> Self {
		self.icon = icon;
		return self;
	}
	pub fn nickcon(mut self, nickcon: Option<String>) -> Self {
		self.nickcon = nickcon;
		return self;
	}
	pub fn password(mut self, password: Option<String>) -> Self {
		self.password = password;
		return self;
	}
	pub fn clienToken(mut self, token: Option<String>) -> Self {
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
	pub fn build(&mut self) -> String {
		return self.build_with_time(std::time::Duration::new(0, 0));
	}
	pub fn build_with_time(&mut self, time_delta: std::time::Duration) -> String {
		use std::time::SystemTime;
		use rand::Rng;

		let session: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).collect();
		let mut time: Option<String> = None;
		
		let hash = match self.token.as_ref() {
			None => "".to_string(),
			Some(token) => {
				let t = format!("{}", (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() + time_delta).as_secs());
				let hash_base = self.implode(token, &t);
				
				time = Some(t);
				format!("{:x}", md5::compute(hash_base.as_bytes()))
			}
		};

		//jtest1nicknameidlevelauthiconnickcon6fe01303583a9e71e6ece678a4f268ef8TkDUyBiousTta7Ko7qwyIMU6fqgTl9lutf-81604136208
		//this.socket.send(['j', this.id, data.nick, data.id, data.level, (data.auth||''), data.icons, data.nickcon, data.other, data.hash, session, ua.charset, data.time, this.installData.password, cache['client_token'], data.profileimg]);
		format!(
			"j{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
			self.room,
			self.nick.as_ref().unwrap_or(&"".to_string()),
			self.id.as_ref().unwrap_or(&"".to_string()),
			self.level.as_ref().unwrap_or(&"".to_string()),
			self.auth.to_string(),
			self.icon.as_ref().unwrap_or(&"".to_string()),
			self.nickcon.as_ref().unwrap_or(&"".to_string()),
			self.other.as_ref().unwrap_or(&"".to_string()), // other
			hash,
			session,
			"utf-8",
			time.unwrap_or("".to_string()),
			self.password.as_ref().unwrap_or(&"".to_string()),
			self.cache_token.as_ref().unwrap_or(&"".to_string()),
			self.profile_image.as_ref().unwrap_or(&"".to_string()),
		)
	}
}

struct UChatRoom {
	access_info: AccessConfig
}

impl UChatRoom {
	fn new(access_info: AccessConfig) -> Self {
		UChatRoom {
			access_info
		}
	}

	async fn connect(&mut self) {
		self.connect_with_uri(url::Url::parse("wss://kr-a-worker1.uchat.io:5001/").unwrap()).await;
	}
	async fn connect_with_uri(&mut self, uri: url::Url) -> tokio_tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>> {
		let tls_connector = native_tls::TlsConnector::new().unwrap();
		let tls_conn = tokio_native_tls::TlsConnector::from(tls_connector);

		let tcp = tokio::net::TcpStream::connect((uri.host_str().unwrap(), uri.port().unwrap())).await.unwrap();
		let tls = tls_conn.connect(uri.host_str().unwrap(), tcp).await.unwrap();

		return tokio_tungstenite::client_async_tls(uri.as_str(), tls).await.unwrap().0;
	}
}