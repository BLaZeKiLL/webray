import fs from 'fs';

function random_color() {
    return [
        Math.random() * Math.random(), 
        Math.random() * Math.random(), 
        Math.random() * Math.random()
    ];
}

function random_color_satu() {
    return [
        (Math.random() / 2) + 0.5,
        (Math.random() / 2) + 0.5,
        (Math.random() / 2) + 0.5
    ];
}

function rgbToHex(color) {
    return "#" + (1 << 24 | color[0] << 16 | color[1] << 8 | color[2]).toString(16).slice(1);
  }

function make_sphere(name, id, mid, center, radius) {
    return {
        name: name,
        id: id,
        material_id: mid,
        type: {
            type: 'd_sphere',
            position: center,
            radius: radius
        }
    };
}

function make_mat_diffuse(name, mid, color) {
    return {
        name: name,
        id: mid,
        type: {
            type: 'd_mat_diffuse',
            color: color
        }
    };
}

function make_mat_metal(name, mid, color, roughness) {
    return {
        name: name,
        id: mid,
        type: {
            type: 'd_mat_metal',
            color: color,
            roughness: roughness
        }
    };
}

function make_mat_dielectric(name, mid, ior) {
    return {
        name: name,
        id: mid,
        type: {
            type: 'd_mat_dielectric',
            ior: ior
        }
    };
}

function vec3f_subtract(a, b) {
    return [a[0] - b[0], a[1] - b[1], a[2] - b [2]];
}

function vec3f_length(a) {
    return Math.sqrt((a[0] * a[0]) + (a[1] * a[1]) + (a[2] * a[2]));
}

const scene = {
    objects: [],
    materials: [],
    camera: {
		look_from: [13, 2, 3],
		look_at: [0, 0, 0],
		v_up: [0, 1, 0],
		v_fov: 20,
		dof_angle: 0.6,
		dof_distance: 10
    },
    render_settings: {
		width: 1920,
		height: 1080,
		samples: 64,
		bounces: 12,
		tile_size: {
			type: 'd_tile_size',
            size: 256
		}
    }
};

let mat_id = 0;
let obj_id = 0;

mat_id++;
obj_id++;

scene.materials.push(make_mat_diffuse(`Mat ${mat_id}`, mat_id, rgbToHex([0.5, 0.5, 0.5].map(x => x * 255))));
scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, [0, -1000, 0], 1000));

for (let x = -11; x < 11; x++) {
    for (let y = -11; y < 11; y++) {
        const choose_mat = Math.random();

        const position = [
            x + (0.9 * Math.random()),
            0.2,
            y + (0.9 * Math.random())
        ];

        if (vec3f_length(vec3f_subtract(position, [4, 0.2, 0.0])) > 0.9) {
            mat_id++;
            obj_id++;

            if (choose_mat < 0.8) {
                const color = rgbToHex(random_color().map(x => x * 255));
                scene.materials.push(make_mat_diffuse(`Mat ${mat_id}`, mat_id, color));
                scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, position, 0.2));
            } else if (choose_mat < 0.95) {
                const color = rgbToHex(random_color_satu().map(x => x * 255));
                scene.materials.push(make_mat_metal(`Mat ${mat_id}`, mat_id, color, Math.random() * 0.5));
                scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, position, 0.2));
            } else {
                scene.materials.push(make_mat_dielectric(`Mat ${mat_id}`, mat_id, 1.5));
                scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, position, 0.2));
            }
        }
    }   
}

mat_id++;
obj_id++;

scene.materials.push(make_mat_dielectric(`Mat ${mat_id}`, mat_id, 1.5));
scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, [0, 1, 0], 1.0));

mat_id++;
obj_id++;

scene.materials.push(make_mat_diffuse(`Mat ${mat_id}`, mat_id, rgbToHex([0.4, 0.2, 0.1].map(x => x * 255))));
scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, [-4, 1, 0], 1.0));

mat_id++;
obj_id++;

scene.materials.push(make_mat_metal(`Mat ${mat_id}`, mat_id, rgbToHex([0.7, 0.6, 0.5].map(x => x * 255)), 0));
scene.objects.push(make_sphere(`Obj ${obj_id}`, obj_id, mat_id, [4, 1, 0], 1.0));

const path = './src/data/demo_02.scene.json';

fs.writeFile(path, JSON.stringify(scene, null, 4), err => {
    if (err) {
        console.error('Error writing JSON to file:', err);
    } else {
        console.log('JSON data has been written to', path);
    }
});