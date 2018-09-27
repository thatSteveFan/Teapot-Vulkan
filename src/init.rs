
use vulkano::device::Device;
use vulkano::instance::Instance;
use std::sync::Arc;
use vulkano::device::Queue;
use vulkano::instance::InstanceExtensions;

use std::collections::HashMap;

extern crate vulkano_win;
extern crate vulkano;
use std::io;


pub fn init_default() -> (Arc<Device>, Arc<Queue>) {
    let instance = {
        // When we create an instance, we have to pass a list of extensions that we want to enable.
        //
        // All the window-drawing functionalities are part of non-core extensions that we need
        // to enable manually. To do so, we ask the `vulkano_win` crate for the list of extensions
        // required to draw to a window.
         let extensions = vulkano_win::required_extensions();//InstanceExtensions::none()

        // Now creating the instance.
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    let physical_devices = vulkano::instance::PhysicalDevice::enumerate(&instance);
    let gpu = get_gpu_choice(physical_devices);
    // print!("{:?}", gpu.queue_families());
    for q in gpu.queue_families() {
        println!("{:?}", q);
    }
    let queue_family = gpu
        .queue_families()
        .find(|q| q.supports_compute() )//&& q.supports_graphics())
        .expect("trouble creating GPU");

    let device_ext = vulkano::device::DeviceExtensions {
        ..vulkano::device::DeviceExtensions::none()
    };
    let (device, mut queueiter) = Device::new(
        gpu,
        gpu.supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned(), // make a queue of 5 elements and medium priority
    ).expect("Trouble making Device");
    (device, queueiter.next().unwrap())
}

pub fn get_gpu_choice<'a>(
    iter: vulkano::instance::PhysicalDevicesIter,
) -> vulkano::instance::PhysicalDevice {
    let len = iter.len();
    let mut physical_device_map = HashMap::with_capacity(len);
    for (i, d) in iter.enumerate() {
        physical_device_map.insert(i, d);
    }
    for (i, d) in physical_device_map.iter() {
        println!("{}: {:?}", i, d.name());
    }
    loop {
        println!("Chose which device to use:");
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("chose a GPU from the list.");
        // print!("{}", user_choice.trim().parse().expect("Error reading number"));

        let user_choice: usize = user_choice
            .trim()
            .parse::<usize>()
            .expect("Error reading number");
        if user_choice >= len {
            continue;
        }
        return physical_device_map.remove(&user_choice).unwrap();
    }
}
