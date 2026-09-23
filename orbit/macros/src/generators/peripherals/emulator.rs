use proc_macro2::{Ident, TokenStream};
use quote::quote;

// TODO: make this more generic
// to just return a list of pins
#[allow(unused_variables)]
pub fn generate_chip_peripherals(
  inputs: &Vec<Ident>,
  outputs: &Vec<Ident>,
) -> (
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
  TokenStream,
) {
  let structs = create_structs();

  let header = quote! {
    use device_query::{DeviceQuery, DeviceState, Keycode};

    #structs
  };

  let init = quote! {
    let device_state = DeviceState::new();
  };

  let input_definition = quote! {
    DeviceInput
  };

  let input_declaration = quote! {
    [#(DeviceInput::new(device_state.clone(), Keycode::#inputs),)*]
  };

  let output_definition = quote! {
    DeviceOutput
  };

  let output_declaration = quote! {
    [#(DeviceOutput::new(p.#outputs),)*]
  };

  (
    header, init, input_definition, input_declaration, output_definition, output_declaration,
  )
}

fn create_structs() -> TokenStream {
  quote! {
    use embedded_hal::digital::ErrorType;
    use std::marker::PhantomData;

    // host key names the emulator's test runner holds down, on top of the real keyboard
    pub static SIMULATED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

    // off while tests run, so typing elsewhere can't disturb them
    pub static REAL_KEYS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

    fn real(keys: &[Keycode], key: &Keycode) -> bool {
      REAL_KEYS.load(std::sync::atomic::Ordering::Relaxed) && keys.contains(key)
    }

    fn simulated(key: &Keycode) -> bool {
      let name = format!("{:?}", key);
      SIMULATED.lock().unwrap().iter().any(|k| *k == name)
    }

    pub struct DeviceInput {
      device_state: DeviceState,
      key: Keycode,
    }

    impl DeviceInput {
      pub fn new(device_state: DeviceState, key: Keycode) -> Self {
        Self { device_state, key }
      }
    }

    impl ErrorType for DeviceInput {
      type Error = Infallible;
    }

    impl InputPin for DeviceInput {
      fn is_high(&mut self) -> Result<bool, Infallible> {
        let keys: Vec<Keycode> = self.device_state.get_keys();
        Ok(real(&keys, &self.key) || simulated(&self.key))
      }

      fn is_low(&mut self) -> Result<bool, Infallible> {
        let keys: Vec<Keycode> = self.device_state.get_keys();
        Ok(!real(&keys, &self.key) && !simulated(&self.key))
      }
    }


    pub struct DeviceOutput {
      key: Keycode,
    }

    impl DeviceOutput {
      pub fn new(key: Keycode) -> Self {
        Self { key }
      }
    }

    impl ErrorType for DeviceOutput {
      type Error = Infallible;
    }

    impl OutputPin for DeviceOutput {
      fn set_high(&mut self) -> Result<(), Infallible> {
        Ok(())
      }

      fn set_low(&mut self) -> Result<(), Infallible> {
        Ok(())
      }
    }
  }
}
