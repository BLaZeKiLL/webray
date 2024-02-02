import type {
	WMatDielectric,
	WMatDiffuse,
	WMatMetal,
	WSphere,
	WTileSizeFull,
	WTileSize,
	vec3f
} from '../types';

export interface WebrayScene {
	objects: WebrayObject[];
	materials: WebrayMaterial[];
	camera: WebrayCamera;
	render_settings: WebrayRenderSettings;
}

export interface WebrayObject {
	id: number;
	name: string;
	material_id: number;
	type: WSphere;
}

export interface WebrayMaterial {
	id: number;
	name: string;
	type: WMatDiffuse | WMatMetal | WMatDielectric;
}

export interface WebrayCamera {
	look_from: vec3f;
	look_at: vec3f;
	v_up: vec3f;
	v_fov: number;
	dof_angle: number;
	dof_distance: number;
}

export interface WebrayRenderSettings {
	width: number;
	height: number;
	samples: number;
	bounces: number;
	tile_size: WTileSizeFull | WTileSize;
}
