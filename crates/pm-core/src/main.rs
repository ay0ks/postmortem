use pm_ui::GCanvas;

async unsafe fn async_main() {
    let canvas = GCanvas::new(None).await;
    canvas.show().await;

    println!("Hello, world!");

    loop {}
}

#[tokio::main]
async fn main() {
    unsafe {
        async_main().await;
    }
}
