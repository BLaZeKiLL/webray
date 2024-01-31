import { writable, derived, get } from 'svelte/store';
import type { WebrayScene } from '../scene/webray.scene';
import {
	TileSize,
	type WMatDielectric,
	type WMatDiffuse,
	type WMatMetal,
	type WSphere
} from '../types';

export class BinderStore {
	private store;

	constructor() {
		this.store = writable<WebrayScene>(this.default_scene());
	}

	public bind_fixed(path: string, property: string) {
		const parts = path.split(':');

		if (!(parts[0] === 'webray' && parts[1] === 'scene')) {
			console.error(`Bind path ${path} not defined`);
			return undefined;
		}

		const bind_path = parts[3];
		const initial = get(this.store);

		if (!(bind_path in initial)) {
			console.error(`Bind path ${path} not defined`);
			return undefined;
		}

		if (!(property in initial[bind_path as keyof WebrayScene])) {
			console.error(`Bind property: ${property} not defined at path: ${path}`);
			return undefined;
		}

		const { subscribe } = derived(this.store, (state) => {
			const data = state[bind_path as keyof WebrayScene];
			const prop = data[property as keyof typeof data];
			return prop; // ts sorcery
		});

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const update = (value: any) => {
			this.store.update((state) => {
				// this is a shallow copy
				// can use structuredClone if a deep copy is required
				const change = { ...state };

				const data = change[bind_path as keyof WebrayScene];

				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				(data[property as keyof typeof data] as any) = value; // ts sorcery

				return change;
			});
		};

		return {
			subscribe,
			update
		};
	}

	private default_scene(): WebrayScene {
		return {
			objects: [
				{
					name: 'ground',
					id: 1,
					material_id: 1,
					type: {
						position: { x: 0.0, y: -100.5, z: -1 },
						radius: 100.0
					} as WSphere
				},
				{
					name: 'left_outer',
					id: 2,
					material_id: 4,
					type: {
						position: { x: -1.0, y: 0.0, z: -1.0 },
						radius: 0.5
					}
				},
				{
					name: 'left_inner',
					id: 3,
					material_id: 4,
					type: {
						position: { x: -1.0, y: 0.0, z: -1.0 },
						radius: -0.4
					}
				},
				{
					name: 'center',
					id: 4,
					material_id: 2,
					type: {
						position: { x: 0.0, y: 0.0, z: -1.0 },
						radius: 0.5
					}
				},
				{
					name: 'right',
					id: 5,
					material_id: 3,
					type: {
						position: { x: 1.0, y: 0.0, z: -1.0 },
						radius: 0.5
					}
				}
			],
			materials: [
				{
					name: 'ground',
					id: 1,
					type: {
						color: ''
					} as WMatDiffuse
				},
				{
					name: 'diffuse',
					id: 2,
					type: {
						color: ''
					} as WMatDiffuse
				},
				{
					name: 'metal',
					id: 3,
					type: {
						color: '',
						roughness: 0.1
					} as WMatMetal
				},
				{
					name: 'dielectric',
					id: 4,
					type: {
						ior: 1.5
					} as WMatDielectric
				}
			],
			camera: {
				look_from: { x: -2.0, y: 2.0, z: 1.0 },
				look_at: { x: 0.0, y: 0.0, z: -1.0 },
				v_up: { x: 0.0, y: 1.0, z: 0.0 },
				v_fov: 20.0,
				dof_angle: 0.6,
				dof_distance: 3.4
			},
			render_settings: {
				width: 1920,
				height: 1080,
				samples: 128,
				bounces: 32,
				tile_size: TileSize.Full
			}
		};
	}
}

export default new BinderStore();