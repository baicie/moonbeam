use crate::Error;
use core_graphics::display::{CGDirectDisplayID, CGDisplay};
use io_kit_sys::{
    display::kIODisplayBrightnessKey, ret::kIOReturnSuccess, types::io_service_t,
    IODisplayCreateWithCGDisplay, IODisplayGetFloatParameter, IODisplaySetFloatParameter,
};
use std::io;

type IOReturn = i32;
type IOOptionBits = u32;
type IoServiceT = u32;

const K_IORETURN_SUCCESS: IOReturn = 0;
const K_IO_DISPLAY_BRIGHTNESS_KEY: &[u8] = b"brightness\0";

#[derive(Debug)]
pub(crate) struct BlockingDeviceImpl {
    display_id: CGDirectDisplayID,
    description: String,
}

impl crate::blocking::Brightness for BlockingDeviceImpl {
    fn device_name(&self) -> Result<String, Error> {
        Ok(self.description.clone())
    }

    fn get(&self) -> Result<u32, Error> {
        unsafe {
            let mut display_connect: io_service_t = 0;
            let result = IODisplayCreateWithCGDisplay(self.display_id, &mut display_connect);

            if result != kIOReturnSuccess {
                return Err(SysError::CreateDisplayFailed.into());
            }

            let mut brightness: f32 = 0.0;
            let result = IODisplayGetFloatParameter(
                display_connect,
                0,
                kIODisplayBrightnessKey.as_ptr() as *const i8,
                &mut brightness,
            );

            if result != kIOReturnSuccess {
                return Err(SysError::GetBrightnessFailed {
                    device: self.description.clone(),
                    source: io::Error::last_os_error(),
                }
                .into());
            }

            Ok((brightness * 100.0) as u32)
        }
    }

    fn set(&self, percentage: u32) -> Result<(), Error> {
        unsafe {
            let mut display_connect: io_service_t = 0;
            let result = IODisplayCreateWithCGDisplay(self.display_id, &mut display_connect);

            if result != kIOReturnSuccess {
                return Err(SysError::CreateDisplayFailed.into());
            }

            let brightness = (percentage as f32) / 100.0;
            let result = IODisplaySetFloatParameter(
                display_connect,
                0,
                kIODisplayBrightnessKey.as_ptr() as *const i8,
                brightness,
            );

            if result != kIOReturnSuccess {
                return Err(SysError::SetBrightnessFailed {
                    device: self.description.clone(),
                    source: io::Error::last_os_error(),
                }
                .into());
            }

            Ok(())
        }
    }
}

pub(crate) fn brightness_devices() -> impl Iterator<Item = Result<BlockingDeviceImpl, SysError>> {
    let main_display = CGDisplay::main();
    if main_display.id == 0 {
        return vec![Err(SysError::NoDisplayFound)].into_iter();
    }

    vec![Ok(BlockingDeviceImpl {
        display_id: main_display.id,
        description: "Main Display".to_string(),
    })]
    .into_iter()
}

#[derive(Debug, Error)]
pub(crate) enum SysError {
    #[error("No display found")]
    NoDisplayFound,

    #[error("Failed to create display connection")]
    CreateDisplayFailed,

    #[error("Failed to get brightness for device {device}")]
    GetBrightnessFailed { device: String, source: io::Error },

    #[error("Failed to set brightness for device {device}")]
    SetBrightnessFailed { device: String, source: io::Error },
}

impl From<SysError> for Error {
    fn from(e: SysError) -> Self {
        match &e {
            SysError::NoDisplayFound | SysError::CreateDisplayFailed => {
                Error::ListingDevicesFailed(Box::new(e))
            }
            SysError::GetBrightnessFailed { device, .. } => Error::GettingDeviceInfoFailed {
                device: device.clone(),
                source: Box::new(e),
            },
            SysError::SetBrightnessFailed { device, .. } => Error::SettingBrightnessFailed {
                device: device.clone(),
                source: Box::new(e),
            },
        }
    }
}
