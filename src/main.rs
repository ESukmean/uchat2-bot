use log::*;

mod uchat;

#[cfg(test)]
mod test_uchat;

fn main() {
	simple_logging::log_to_stderr(log::LevelFilter::Debug);

	let runtime = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.on_thread_start(|| println!("new thread start"))
		.on_thread_stop(|| println!("thread  stopped"))
		.build()
		.unwrap(); 

	debug!("runtime start");
	runtime.block_on(entry());
	debug!("runtime stopped");
}


async fn entry() {
	let ac = uchat::JoinConfig::new("#bWFpbg==".to_string());
	let mut uconn = uchat::UChatRoom::new(ac);
	let r = uconn.connect().await;
	debug!("{:?}", r);
	let r = uconn.process().await;
}
