use bytes::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct userKey {
	//k [Text("k"), Binary(b"\xba\xf4\xcf\x1bg\xb7C\x11"), Text("탁월한지도자"), Text(""), None, Number(0), Text(""), Text(""), Number(-1413504929), Number(0), Binary(b"$\x87\x06\x83e\xd8%\t")]
	/* var user = {
		'nick':data[1]
		, 'id':data[2]
		, 'level':data[3]
		, 'auth':data[4]
		, 'icons':data[5]
		, 'nickcon':data[6]
		, 'connected':getTimeStamp(data[7])
		, 'time': data[7]
		, 'mute':data[8]&1
		, 'session':(data[9]||'').hexEncode()
		, 'profileimg':data[10]
	}; */
	pub key: Bytes,
	pub nick: String,
	pub id: String,
	pub level: Option<String>,
	pub auth: i32,
	pub icon: String,
	pub nickcon: String,
	pub connected: std::time::SystemTime,
	pub mute: bool,
	pub session: Bytes,
	pub profileimg: Option<String>,
}

impl userKey {
	pub fn empty() -> Self {
		Self {
			key: Bytes::new(),
			nick: String::new(),
			id: String::new(),
			level: None,
			auth: 0,
			icon: String::new(),
			nickcon: String::new(),
			connected: std::time::UNIX_EPOCH,
			mute: false,
			session: Bytes::new(),
			profileimg: None,
		}
	}
	pub fn unpack(mut data: Vec<Message>) -> Result<Self, ()> {
		if data.len() < 11 {
			return Err(());
		} // 11개면 profile img 미포함, 12개면 profile img 포함
  // let mut data = msg.unwrap();
  // Vec구조상 맨 앞의 원소를 제거하면 뒤에거를 다 앞으로 땡겨야 하니까.. 뒤에서 부터 치고 나감.

		let mut profileimg = None;
		if data.len() == 12 {
			// 12번째 필드가 있을수도, 없을수도 있음.
			profileimg = data.remove(11).unwrap_text().ok();
		}
		let session = data.remove(10).unwrap_binary().unwrap_or(Bytes::new());
		let mute = data.remove(9).unwrap_number().unwrap_or(0) != 0;
		let connected = data
			.remove(8)
			.unwrap_number()
			.map(|v| v as u32)
			.map(|v| std::time::UNIX_EPOCH + std::time::Duration::from_secs(v as u64))
			.unwrap();
		let nickcon = data.remove(7).unwrap_text().unwrap_or("".to_string());
		let icon = data.remove(6).unwrap_text().unwrap_or("".to_string());
		let auth = data.remove(5).unwrap_number().unwrap_or(0) as i32;
		let level = data.remove(4).unwrap_text().ok();
		let id = data.remove(3).unwrap_text().unwrap_or("".to_string());
		let nick = data.remove(2).unwrap_text().unwrap_or("".to_string());
		let key = data.remove(1).unwrap_binary().unwrap_or(Bytes::new());
		// 0 -> "k" type

		return Ok(userKey {
			key,
			nick,
			id,
			level,
			auth,
			icon,
			nickcon,
			connected,
			mute,
			session,
			profileimg,
		});
		// return Err(());
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
	//Header(u8),
	Text(String),
	Binary(Bytes),
	Boolean(bool),
	Number(i64),
	Float(f64),
	None,
}
impl Message {
	#[inline]
	pub fn append(&self, buf: &mut BytesMut) {
		let condition = |b: &u8| -> bool {
			return *b < 8;
		};

		match self {
			//Message::Header(c) => buf.put_u8(*c),
			Message::Binary(v) => {
				buf.put_u8(2);
				buf.extend_from_slice(v);
			}
			Message::Text(s) => {
				buf.put_u8(3);
				escape(s.as_bytes(), buf, condition, b'\\');
			}
			Message::Boolean(b) => {
				buf.put_u8(4);
				if *b {
					buf.put_u8(49)
				} else {
					buf.put_u8(48);
				}
			}
			Message::Number(i) => {
				buf.put_u8(5);
				escape(&i.to_be_bytes(), buf, condition, b'\\');
			}
			Message::None => buf.put_u8(6),
			Message::Float(f) => {
				buf.put_u8(7);
				escape(format!("{}", f).as_bytes(), buf, condition, b'\\');
			}
		};
	}
	pub fn wrap(code: u8, data: &mut BytesMut) -> Self {
		match code {
			2 => {
				return Message::Binary(data.clone().freeze());
			}
			3 => unsafe {
				return Message::Text(String::from_utf8_unchecked(data.to_vec()));
			},
			4 => {
				if data.len() > 0 && data[0] == b'1' {
					return Message::Boolean(true);
				}

				return Message::Boolean(false);
			}
			5 => {
				return match data.len() {
					0 => Message::Number(0),
					2 => Message::Number(data.get_i16_le() as i64),
					4 => Message::Number(data.get_i32_le() as i64),
					8 => Message::Number(data.get_i64_le()),
					_ => Message::Number(data.get_i8() as i64),
				};
			}
			7 => {
				if let Ok(f) = std::str::from_utf8(data) {
					return Message::Float(f.parse().unwrap_or(0.0));
				}

				return Message::Float(0.0);
			}
			_ => {
				return Message::None;
			}
		}
	}

	pub fn unwrap_text(self) -> Result<String, ()> {
		match self {
			Message::Text(v) => return Ok(v),
			_ => return Err(()),
		};
	}
	pub fn unwrap_binary(self) -> Result<Bytes, ()> {
		match self {
			Message::Binary(v) => return Ok(v),
			_ => return Err(()),
		};
	}
	pub fn unwrap_boolean(self) -> Result<bool, ()> {
		match self {
			Message::Boolean(v) => return Ok(v),
			_ => return Err(()),
		};
	}
	pub fn unwrap_number(self) -> Result<i64, ()> {
		match self {
			Message::Number(v) => return Ok(v),
			_ => return Err(()),
		};
	}
	pub fn unwrap_float(self) -> Result<f64, ()> {
		match self {
			Message::Float(v) => return Ok(v),
			_ => return Err(()),
		};
	}
	pub fn unwrap_none(self) -> Result<Option<()>, ()> {
		match self {
			Message::None => return Ok(None),
			_ => return Err(()),
		};
	}
}

#[derive(Debug)]
pub struct uMessage {
	inner: Vec<Message>,
}
impl uMessage {
	pub fn new() -> Self {
		uMessage { inner: Vec::new() }
	}
	pub fn unpack(mut buf: BytesMut) -> Self {
		let mut inner = Vec::new();
		// log::debug!("rcv buf {:?}", buf);
		// log::debug!("data {:?}", buf);

		if buf.len() > 0 {
			// inner.push(Message::Header(buf[0]));
			// buf.advance(1);

			let mut tmp = BytesMut::with_capacity(buf.len());
			let mut mode = 3;

			while buf.is_empty() == false {
				match buf[0] {
					b'\\' => {
						if buf.len() > 1 {
							tmp.put_u8(buf[1]);
							buf.advance(1);
						} else {
							tmp.put_u8(b'\\');
						}
					}
					type_code if type_code > 1 && type_code < 8 => {
						inner.push(Message::wrap(mode, &mut tmp));
						tmp.clear();

						mode = type_code;
					}
					other => {
						tmp.put_u8(other);
					}
				}

				buf.advance(1);
			}

			inner.push(Message::wrap(mode, &mut tmp));
			tmp.clear();
		}

		return uMessage { inner };
	}

	pub fn len(&self) -> usize {
		return self.inner.len();
	}
	pub fn push(&mut self, msg: Message) {
		self.inner.push(msg);
	}
	pub fn unwrap(self) -> Vec<Message> {
		return self.inner;
	}
	pub fn pack(mut self) -> Bytes {
		let mut b = BytesMut::new();
		let mut escape_buf = BytesMut::with_capacity(128);

		if self.inner.len() > 0 {
			self.inner.remove(0).append(&mut escape_buf);
			escape_buf.advance(1);

			self.inner.iter().for_each(|item| {
				item.append(&mut escape_buf);

				escape(&escape_buf, &mut b, |b| *b == b'\n', b'\\');
				escape_buf.clear();
			});
		}

		b.put_u8(b'\n');
		return b.freeze();
	}
}

#[inline]
fn escape<F>(b: &[u8], buf: &mut BytesMut, condition: F, escape_char: u8)
where
	F: Fn(&u8) -> bool,
{
	b.iter().for_each(|item| {
		if condition(item) {
			buf.put_u8(escape_char);
		}

		buf.put_u8(*item);
	})
}
pub fn delimit_line(buf: &mut BytesMut) -> Vec<BytesMut> {
	let mut result = Vec::new();
	let mut tmp = BytesMut::with_capacity(buf.len());

	while buf.is_empty() == false {
		match buf[0] {
			b'\\' => {
				if buf.len() > 1 && buf[1] == b'\n' {
					tmp.put_u8(b'\n');
					buf.advance(2);

					continue;
				} else {
					tmp.put_u8(b'\\');
				}
			}
			b'\n' => {
				result.push(tmp.split());
			}
			other => {
				tmp.put_u8(other);
			}
		};

		buf.advance(1);
	}

	if tmp.len() > 0 {
		result.push(tmp.split());
	}
	return result;
}
