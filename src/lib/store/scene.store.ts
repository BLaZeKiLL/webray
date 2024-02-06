import { writable, derived, get, type Writable, type Readable } from 'svelte/store';
import { get_prop, set_prop } from '../utils/object.extensions';
import type { WebrayScene } from '../editor/webray.scene';
import { get_id_prop, set_index_prop } from '../utils/array.extensions';

import { tick } from 'svelte';
import { BindDataMap, WebrayEditor } from '../editor';

import _demo_json from '../../data/demo_01.scene.json';

export class SceneStore {
	private store;

	constructor() {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		this.store = writable<WebrayScene>(_demo_json as any);
	}

	public get current() {
		return get(this.store);
	}

	public add_list_item(path: string) {
		const bind_path = SceneStore.get_binding_path(path);
		const data_type = BindDataMap[bind_path as keyof typeof BindDataMap];
		const item = WebrayEditor.getDefaultObj(data_type);

		this.store.update((state) => {
			const change = { ...state };

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const list = change[bind_path as keyof WebrayScene] as any[];

			item.id = list.length + 1;

			list.push(item);

			return change;
		});
	}

	public del_list_item(path: string) {
		const bind = SceneStore.get_binding_path_with_index(path);

		this.store.update((state) => {
			const change = { ...state };

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const list = change[bind.path as keyof WebrayScene] as any[];

			list.splice(
				list.findIndex((val) => val.id === bind.id),
				1
			);

			return change;
		});
	}

	public derived<T>(path: string): Readable<T> {
		const bind_path = SceneStore.get_binding_path(path);

		return derived(this.store, (state) => {
			const data = state[bind_path as keyof WebrayScene] as T;

			return data;
		});
	}

	public bind<T>(path: string, property: string, name = 'store'): Writable<T> {
		const parts = path.split(':');

		if (!(parts[0] === 'webray' && parts[1] === 'scene')) {
			throw new Error(`Bind path ${path} not defined`);
		}

		const bind_path = parts[2].split('[')[0]; // last part

		const initial = get(this.store);

		if (!(bind_path in initial)) {
			throw new Error(`Bind path ${path} not defined`);
		}

		if (parts[2].includes('[')) {
			const bind_index = parseInt(parts[2].split('[')[1].split(']')[0]);

			return {
				name,
				...this.list_bind<T>(bind_path, bind_index, property)
			} as Writable<T>;
		} else {
			return {
				name,
				...this.single_bind<T>(bind_path, property)
			} as Writable<T>;
		}
	}

	private single_bind<T>(bind_path: string, property: string): Writable<T> {
		const { subscribe } = derived(this.store, (state) => {
			const data = state[bind_path as keyof WebrayScene];
			const prop = get_prop(data, property);

			if (prop === undefined) {
				tick().then(() => {
					if (get_prop(data, property) === undefined) {
						console.error(`bind subscribe failed!, bind path: ${bind_path}, property: ${property}`);
						console.error(data);
					}
				});
			}

			return prop;
		});

		const update = (value: T) => {
			this.store.update((state) => {
				// this is a shallow copy
				// can use structuredClone if a deep copy is required
				const change = { ...state };

				const data = change[bind_path as keyof WebrayScene];

				set_prop(data, property, value);

				return change;
			});
		};

		const set = (value: T) => {
			// this is a shallow copy
			// can use structuredClone if a deep copy is required
			const change = { ...get(this.store) };

			const data = change[bind_path as keyof WebrayScene];

			set_prop(data, property, value);

			this.store.set(change);
		};

		return {
			subscribe,
			update,
			set
		} as Writable<T>;
	}

	private list_bind<T>(bind_path: string, bind_index: number, property: string): Writable<T> {
		const { subscribe } = derived(this.store, (state) => {
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const data = state[bind_path as keyof WebrayScene] as any[];
			const prop = get_id_prop(data, bind_index, property);

			if (prop === undefined) {
				// TODO: This is a dirty hacky fix :P
				// re-check next micro tick as it maybe fixed by re-set validator
				tick().then(() => {
					if (data.find((val) => val.id === bind_index) === undefined) {
						// console.warn('Item does not exist');
						// console.warn(data);
						return;
					}

					if (get_id_prop(data, bind_index, property) === undefined) {
						console.error(
							`bind subscribe failed!, bind path: ${bind_path}, id: ${bind_index} property: ${property}`
						);
						console.error(data);
						return;
					}
				});
			}

			return prop;
		});

		const update = (value: T) => {
			this.store.update((state) => {
				// this is a shallow copy
				// can use structuredClone if a deep copy is required
				const change = { ...state };

				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const data = change[bind_path as keyof WebrayScene] as any[];

				set_index_prop(data, bind_index, property, value);

				return change;
			});
		};

		const set = (value: T) => {
			// this is a shallow copy
			// can use structuredClone if a deep copy is required
			const change = { ...get(this.store) };

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const data = change[bind_path as keyof WebrayScene] as any[];

			set_index_prop(data, bind_index, property, value);

			this.store.set(change);
		};

		return {
			subscribe,
			update,
			set
		} as Writable<T>;
	}

	public static get_binding_path(bind_path: string): string {
		const parts = bind_path.split(':');

		return parts[2].split('[')[0];
	}

	public static get_binding_path_with_index(bind_path: string): { path: string; id: number } {
		const parts = bind_path.split(':');

		return {
			path: parts[2].split('[')[0],
			id: parseInt(parts[2].split('[')[1].split(']')[0])
		};
	}
}

export default new SceneStore();
