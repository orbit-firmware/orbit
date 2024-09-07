// use stm32f3xx_hal::usb::{Peripheral, UsbBus};
// use usb_device::prelude::*;
// use usbd_serial::{SerialPort, USB_CLASS_CDC};

// pub fn usb() {
//   // Enable USB.
//   let usb_dm = gpioa
//     .pa11
//     .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
//   let usb_dp = usb_dp.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

//   let usb = Peripheral {
//     usb: dp.USB,
//     pin_dm: usb_dm,
//     pin_dp: usb_dp,
//   };

//   let usb_bus = UsbBus::new(usb);
//   let mut serial = SerialPort::new(&usb_bus);

//   let descriptors = StringDescriptors::default()
//     .manufacturer("github.com/sunsided")
//     .product("stm32f3disco-rust")
//     .serial_number(env!("SERIAL"));

//   let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
//     .strings(&[descriptors])
//     .unwrap()
//     .device_class(USB_CLASS_CDC)
//     // .self_powered(false)
//     .build();
// }

// pub fn usb_read() {
//   // Must be called at least every 10 ms, i.e. at 100 Hz.
//   let usb_has_events = usb_dev.poll(&mut [&mut serial]);

//   // Handle reading of data first.
//   if usb_has_events {
//     let mut buf = [0u8; 64];
//     match serial.read(&mut buf[..]) {
//       Ok(_count) => {
//         // count bytes were read to &buf[..count]
//         defmt::trace!("Received USB data");
//       }
//       Err(UsbError::WouldBlock) => {
//         // No data received
//         defmt::trace!("Received no USB data");
//       }
//       Err(err) => {
//         // An error occurred
//         defmt::error!("Failed to receive USB data: {}", err);
//       }
//     };
//   }
// }

// pub fn usb_write() {
//   match serial.write(transmit_buffer) {
//     Ok(count) => {
//       let remaining = sensor_buffer.commit_read(count);
//       if remaining > 0 {
//         defmt::warn!(
//           "Couldn't write completely, range is now {} (length {})",
//           sensor_buffer.buffer_range(),
//           remaining
//         );
//       }
//     }
//     Err(UsbError::WouldBlock) => {
//       // No data could be written (buffers full)
//       defmt::trace!("Buffer full while writing USB data");
//     }
//     Err(err) => {
//       // An error occurred
//       defmt::error!("Failed to send USB data: {}", err);
//     }
//   };
// }
