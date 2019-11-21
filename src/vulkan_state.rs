use ash::{self, vk};
use ash::extensions::{ext, khr, nv::RayTracing, nv};
use ash::extensions::khr::XlibSurface as PlatformSurface;
use ash::version::{DeviceV1_1, EntryV1_0, InstanceV1_1};
use std::ffi::{CStr, CString};

pub struct VulkanState {
    pub entry: ash::Entry,
    pub instance: ash::Instance
}

fn get_extensions() -> Vec<*const i8> {
    vec![
        khr::Surface::name().as_ptr(),
        PlatformSurface::name().as_ptr(),
        ext::DebugReport::name().as_ptr()
    ]
}

fn instance_entry_new(name: &str, layers: &[&str]) ->
                      (ash::Entry, ash::Instance) {

    let app_name_cstr = CString::new(name).unwrap();

    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name_cstr)
        .application_version(0)
        .engine_name(&app_name_cstr)
        .engine_version(0)
        .api_version(ash::vk_make_version!(1, 1, 0));

    let layers_cstr : Vec<CString> = layers.iter()
        .map(|layer| CString::new(*layer).unwrap())
        .collect();

    let layers_raw : Vec<*const i8> = layers_cstr.iter()
        .map(|cstr| cstr.as_ptr())
        .collect();

    let exts_raw = get_extensions();

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers_raw)
        .enabled_extension_names(&exts_raw);

    let entry = unsafe {
        ash::Entry::new().unwrap()
    };

    let instance = unsafe {
        entry.create_instance(&create_info, None)
            .expect("Failed to create VK instance")
    };

    (entry, instance)
}

impl VulkanState {
    pub fn new(name: &str, window_sz: (u32, u32)) -> Self {
        let (entry, inst) = instance_entry_new(name,
            &["VK_LAYER_LUNARG_standard_validation"]);

        VulkanState {
            entry: entry,
            instance: inst
        }
    }

}
