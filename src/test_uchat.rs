use super::uchat;

#[tokio::test]
async fn test_connect() {
	let jc = uchat::JoinConfig::new("#bWFpbg==".to_string());
	let mut uconn = uchat::UChatRoomProc::new(jc);
	
	assert_eq!(true, uconn.connect().await.is_ok());
}
#[tokio::test]
async fn test_connect_different_url() {
	let jc = uchat::JoinConfig::new("#bWFpbg==".to_string());
	let mut uconn = uchat::UChatRoomProc::new(jc);
	let uri = url::Url::parse("ws://kr-a-worker1.uchat.io:5002/").unwrap();

	assert_eq!(true, uconn.connect_with_uri(uri).await.is_ok());
}
#[test]
fn test_auth_level_str() {
	assert_eq!(uchat::UChatAuthLevel::Admin.to_string(), "admin");
	assert_eq!(uchat::UChatAuthLevel::SubAdmin.to_string(), "subadmin");
	assert_eq!(uchat::UChatAuthLevel::Member.to_string(), "member");
	assert_eq!(uchat::UChatAuthLevel::Guest.to_string(), "guest");
	assert_eq!(uchat::UChatAuthLevel::None.to_string(), "");
}
#[test]
fn test_join_config() {
	// pub struct JoinConfig {
	let room: String = "room".to_string();
	let token: Option<String> = Some("token".to_string());
	let nick: Option<String> = Some("nick".to_string());
	let id: Option<String> = Some("id".to_string());
	let level: Option<String> = Some("level".to_string());
	let auth: uchat::UChatAuthLevel = uchat::UChatAuthLevel::SubAdmin;
	let icon: Option<String> = Some("icon".to_string());
	let nickcon: Option<String> = Some("nickcon".to_string());
	let other: Option<String> = Some("other".to_string());
	let password: Option<String> = Some("password".to_string());
	let cache_token: Option<String> = Some("cache_token".to_string());
	let profile_image: Option<String> = Some("profile_image".to_string());
	// }
	let jc = uchat::JoinConfig::new(room.clone());
	let jc = jc.token(token.clone())
		.nick(nick.clone())
		.id(id.clone())
		.level(level.clone())
		.auth(auth.clone())
		.icon(icon.clone())
		.nickcon(nickcon.clone())
		.other(other.clone())
		.password(password.clone())
		.client_token(cache_token.clone())
		.profile_image(profile_image.clone());
	
	assert_eq!(jc.room, room);
	assert_eq!(jc.token, token);
	assert_eq!(jc.nick, nick);
	assert_eq!(jc.id, id);
	assert_eq!(jc.level, level);
	assert_eq!(jc.auth, auth);
	assert_eq!(jc.icon, icon);
	assert_eq!(jc.nickcon, nickcon);
	assert_eq!(jc.other, other);
	assert_eq!(jc.password, password);
	assert_eq!(jc.cache_token, cache_token);
	assert_eq!(jc.profile_image, profile_image);
}
