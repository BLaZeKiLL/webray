use wgpu_hello::run;

fn main() {
    pollster::block_on(run()).expect("App error");
}
