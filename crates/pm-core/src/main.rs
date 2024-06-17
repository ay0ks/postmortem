use pm_ui::{GCanvas, GCoordinateHelper};

async unsafe fn async_main() {
    let (x, y) = GCoordinateHelper::center(None).await.unwrap();
    let mut canvas = GCanvas::new(None, x, y, 400, 300).await.unwrap();
    canvas.set_title("Hello, world!").await;
    canvas.open().await;
    canvas.run().await;

    println!("Hello, world!");

    loop {}
}

#[tokio::main]
async fn main() {
    unsafe {
        async_main().await;
    }
}
