use core::sync::atomic::{AtomicBool, Ordering};
use embassy_usb::{
  class::hid::{HidReaderWriter, ReportId, RequestHandler, State},
  control::{InResponse, OutResponse},
  driver::Driver,
  Builder, Config, Handler,
};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

static USB_READY: AtomicBool = AtomicBool::new(false);

use crate::orbit::dbg::*;

pub struct HID<'a, D: Driver<'a>> {
  config_descriptor: [u8; 256],
  bos_descriptor: [u8; 256],
  msos_descriptor: [u8; 256],
  control_buf: [u8; 64],
  driver: &'a D,
}

impl<'a, D: Driver<'a>> HID<'a, D> {
  pub fn new(driver: &'a D) -> Self {
    let mut hid = HID {
      config_descriptor: [0; 256],
      bos_descriptor: [0; 256],
      msos_descriptor: [0; 256],
      control_buf: [0; 64],
      driver,
    };

    hid.configure();

    hid
  }

  pub fn usb_ready() -> bool {
    USB_READY.load(Ordering::SeqCst)
  }

  fn configure(&mut self) {
    let mut config = Config::new(
      0x16c0, // VID 5824 (0x16c0) | For USB Keyboards
      0x27db, // PID 10203 (0x27db) | For USB Keyboards
    );
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    // let mut request_handler = KeyboardRequest {};
    // let mut device_handler = KeyboardDevice::new();

    // // State needs to live long enough for the builder to use it
    // let mut state = State::new();
  }
}
// pub struct HID<'d, D: Driver> {
//   config_descriptor: [u8; 256],
//   bos_descriptor: [u8; 256],
//   msos_descriptor: [u8; 256],
//   control_buf: [u8; 64],
//   driver: &'d dyn D,
// }

// impl<'d, D: Driver<'d>> HID<'d, D> {
//   pub fn new<'d, D: Driver<'d>>(driver: &'d D) -> Self {
//     let mut hid = HID {
//       config_descriptor: [0u8; 256],
//       bos_descriptor: [0u8; 256],
//       msos_descriptor: [0u8; 256],
//       control_buf: [0u8; 64],
//       driver,
//     };

//     // hid.configure(driver);
//     return hid;

//     // // Create embassy-usb DeviceBuilder using the driver and config.
//     // // It needs some buffers for building the descriptors.
//     // let mut config_descriptor = [0u8; 256];
//     // let mut bos_descriptor = [0u8; 256];
//     // let mut msos_descriptor = [0u8; 256];
//     // let mut control_buf = [0u8; 64];

//     // // Define your handlers
//     // let mut request_handler = KeyboardRequest {};
//     // let mut device_handler = KeyboardDevice::new();

//     // // State needs to live long enough for the builder to use it
//     // let mut state = State::new();

//     // // Define the builder with the correct types by converting arrays to slices.
//     // let mut builder: Builder<'_, D> = Builder::new(
//     //   driver, config, &mut config_descriptor, &mut bos_descriptor, &mut msos_descriptor,
//     //   &mut control_buf,
//     // );

//     // // Uncomment and modify these lines as needed to set up the HID class.
//     // // builder.handler(&mut device_handler);

//     // // Create the HID Reader/Writer if needed.
//     // // let hid_config = embassy_usb::class::hid::Config {
//     // //     report_descriptor: KeyboardReport::desc(),
//     // //     request_handler: Some(&mut request_handler),
//     // //     poll_ms: 60,
//     // //     max_packet_size: 8,
//     // // };
//     // // let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, hid_config);

//     // // Build the USB device.
//     // // let usb = builder.build();
//     // // let usb_fut = usb.run();

//     // // let (reader, mut writer) = hid.split();
//   }

//   fn configure<'b, D: Driver<'b>>(&mut self, driver: D) {
//     let mut config = Config::new(
//       0x16c0, // VID 5824 (0x16c0) | For USB Keyboards
//       0x27db, // PID 10203 (0x27db) | For USB Keyboards
//     );
//     config.manufacturer = Some("Embassy");
//     config.product = Some("HID keyboard example");
//     config.serial_number = Some("12345678");
//     config.max_power = 500;
//     config.max_packet_size_0 = 64;

//     // Required for windows compatibility.
//     // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
//     config.device_class = 0xEF;
//     config.device_sub_class = 0x02;
//     config.device_protocol = 0x01;
//     config.composite_with_iads = true;

//     let mut request_handler = KeyboardRequest {};
//     let mut device_handler = KeyboardDevice::new();

//     // State needs to live long enough for the builder to use it
//     let mut state = State::new();

//     // // Define the builder with the correct types by converting arrays to slices.
//     // let mut builder: Builder<'_, D> = Builder::new(
//     //   driver, config, &mut self.config_descriptor, &mut self.bos_descriptor,
//     //   &mut self.msos_descriptor, &mut self.control_buf,
//     // );
//   }

//   pub fn usb_ready() -> bool {
//     USB_READY.load(Ordering::SeqCst)
//   }
// }

// struct KeyboardRequest {}

// impl RequestHandler for KeyboardRequest {
//   fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
//     info!("Get report for {:?}", id);
//     None
//   }

//   fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
//     info!("Set report for {:?}: {=[u8]}", id, data);
//     OutResponse::Accepted
//   }

//   fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
//     info!("Set idle rate for {:?} to {:?}", id, dur);
//   }

//   fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
//     info!("Get idle rate for {:?}", id);
//     None
//   }
// }

// struct KeyboardDevice {
//   configured: AtomicBool,
// }

// impl KeyboardDevice {
//   fn new() -> Self {
//     KeyboardDevice {
//       configured: AtomicBool::new(false),
//     }
//   }
// }

// impl Handler for KeyboardDevice {
//   fn enabled(&mut self, enabled: bool) {
//     self.configured.store(false, Ordering::Relaxed);
//     if enabled {
//       info!("Device enabled");
//     } else {
//       info!("Device disabled");
//     }
//   }

//   fn reset(&mut self) {
//     self.configured.store(false, Ordering::Relaxed);
//     info!("Bus reset, the Vbus current limit is 100mA");
//   }

//   fn addressed(&mut self, addr: u8) {
//     self.configured.store(false, Ordering::Relaxed);
//     info!("USB address set to: {}", addr);
//   }

//   fn configured(&mut self, configured: bool) {
//     self.configured.store(configured, Ordering::Relaxed);
//     if configured {
//       if !USB_READY.load(Ordering::SeqCst) {
//         USB_READY.store(true, Ordering::SeqCst);
//       }
//       info!("Device configured, it may now draw up to the configured current limit from Vbus.")
//     } else {
//       if USB_READY.load(Ordering::SeqCst) {
//         USB_READY.store(false, Ordering::SeqCst);
//       }
//       info!("Device is no longer configured, the Vbus current limit is 100mA.");
//     }
//   }
// }
