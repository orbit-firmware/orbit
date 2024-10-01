// use crate::hid::builder as HID;
// use crate::hid::builder::{HIDDeviceHandler, HIDRequestHandler};
// use core::cell::UnsafeCell;
// use core::sync::atomic::{AtomicBool, Ordering};
// use defmt::info;
// use embassy_futures::join::join;
// use embassy_time::Timer;
// use embassy_usb::class::hid::Config as HidConfig;
// use embassy_usb::class::hid::{HidReader, HidReaderWriter, HidWriter, State};
// use embassy_usb::class::hid::{ReportId, RequestHandler};
// use embassy_usb::control::OutResponse;
// use embassy_usb::driver::Driver;
// use embassy_usb::Builder;
// use embassy_usb::Config;
// use embassy_usb::Handler;
// use embassy_usb::UsbDevice;
// use static_cell::StaticCell;

// static INITIIALIZED: AtomicBool = AtomicBool::new(false);
// static mut DEVICE: UnsafeCell<Option<HIDDeviceHandler>> = UnsafeCell::new(None);
// static mut REQUEST: UnsafeCell<Option<HIDRequestHandler>> = UnsafeCell::new(None);

// pub const READ_N: usize = 1;
// pub const WRITE_N: usize = 8;
// const USB_RETRY_TIME: u64 = 100; // ms

// pub async fn ready() {
//   unsafe {
//     let ready = (*DEVICE.get()).as_ref().unwrap().ready;
//     while !ready {
//       Timer::after_millis(USB_RETRY_TIME).await;
//     }
//   }
// }

// pub async fn init<D: Driver<'static>>(
//   driver: D,
// ) -> (
//   UsbDevice<'static, D>,
//   HidReader<'static, D, READ_N>,
//   HidWriter<'static, D, WRITE_N>,
// ) {
//   unsafe {
//     if !INITIIALIZED.load(Ordering::SeqCst) {
//       INITIIALIZED.store(true, Ordering::SeqCst);
//       (*DEVICE.get()) = Some(HIDDeviceHandler::new());
//       (*REQUEST.get()) = Some(HIDRequestHandler {});
//     }

//     static STATE: StaticCell<State> = StaticCell::new();
//     let state = STATE.init(State::new());

//     // 10203 (0x27db) | 5824 (0x16c0) | For USB Keyboards
//     // https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt
//     let mut builder = HID::build::<D>(driver, 0x27db, 0x16c0);
//     builder.handler((*DEVICE.get()).as_mut().unwrap());

//     let config = HidConfig {
//       report_descriptor: &[0; 0],
//       request_handler: Some((*REQUEST.get()).as_mut().unwrap()),
//       poll_ms: 60,
//       max_packet_size: 8,
//     };

//     let hid = HidReaderWriter::<D, READ_N, WRITE_N>::new(&mut builder, state, config);

//     let mut usb = builder.build();
//     let (reader, writer) = hid.split();

//     return (usb, reader, writer);
//   }
// }
