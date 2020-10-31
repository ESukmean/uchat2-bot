use log::*;

mod uchat;

fn main() {
    simple_logging::log_to_stderr(log::LevelFilter::Debug);

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
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

}
