use log::*;

mod uchat;

#[cfg(test)]
mod test_uchat;

fn main() {
	simple_logging::log_to_stderr(log::LevelFilter::Debug);

	let runtime = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.on_thread_start(|| debug!("new thread start"))
		.on_thread_stop(|| debug!("thread stopped"))
		.build()
		.unwrap(); 

	info!("runtime start");
	runtime.block_on(entry());
	info!("runtime stopped");
}


async fn entry() {
	let ac = uchat::JoinConfig::new("#bWFpbg==".to_string());
	let mut uconn = uchat::UChatRoomProc::new(ac);
	let r = uconn.connect().await;
	debug!("{:?}", r);
	let r = uconn.process().await;
	debug!("{:?}", r);
}
