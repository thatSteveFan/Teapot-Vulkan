extern crate vulkano;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano_shader_derive;

mod init;

fn main() {
    let (device, queue_family) = init::init_default();
}
