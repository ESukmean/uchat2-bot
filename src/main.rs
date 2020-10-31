use log::*;

mod uchat;

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

}
