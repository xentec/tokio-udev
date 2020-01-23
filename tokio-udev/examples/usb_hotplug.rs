use tokio::stream::StreamExt;
use tokio_udev::{Context, MonitorBuilder};

#[tokio::main]
async fn main() {
    let context = Context::new().unwrap();
    let mut builder = MonitorBuilder::new(&context).unwrap();
    builder.match_subsystem_devtype("usb", "usb_device").unwrap();

    let mut monitor = builder.listen().unwrap();
    while let Some(event) = monitor.next().await {
        println!("Hotplug event: {}: {}", event.event_type(), event.device().syspath().display())
    }
}
