@group(0) @binding(0) var<storage, read_write> data: array<i32>;

@compute @workgroup_size(1) fn main(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    var i = id.x;
    data[i] = data[i] * 2;
}