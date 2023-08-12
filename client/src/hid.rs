use hidapi::DeviceInfo;

const VENDOR_ID: u16 = 0xfeed;
const PRODUCT_ID: u16 = 0;
const USAGE_PAGE: u16 = 0xff60;

pub fn is_my_device(device: &DeviceInfo) -> bool {
    device.vendor_id() == VENDOR_ID
        && device.product_id() == PRODUCT_ID
        && device.usage_page() == USAGE_PAGE
        && device.usage() == 0x61
}
