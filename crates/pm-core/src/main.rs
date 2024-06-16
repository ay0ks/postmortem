use pm_ui::GCanvas;

async unsafe fn async_main() {
    let mut canvas = GCanvas::new(None).await.unwrap();
    canvas.set_title("Hello, world!").await;
    canvas.open().await;

    println!("Hello, world!");

    loop {}
}

#[tokio::main]
async fn main() {
    unsafe {
        async_main().await;
    }
}
