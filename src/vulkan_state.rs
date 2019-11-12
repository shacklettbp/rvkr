use ash;

pub struct VulkanState {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub device: ash::Device
}

impl VulkanState {
    pub fn new(window_sz: (u32, u32)) -> Self {

        VulkanState {
            entry: entry,
            instance: inst,
            device: dev
        }
    }
}
