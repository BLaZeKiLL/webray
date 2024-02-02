import { writable, derived, get, type Writable } from 'svelte/store';
import { get_prop, set_prop } from '../utils/object.extensions';
import type { WebrayScene } from '../editor/webray.scene';
import _demo_json from '../../data/demo_01.scene.json';
import { get_index_prop, set_index_prop } from '../utils/array.extensions';


export class SceneStore {
	private store;

	constructor() {
		this.store = writable<WebrayScene>(_demo_json);
	}

	public get current() {
		return get(this.store);
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
				console.error(`bind subscribe failed!, bind path: ${bind_path}, property: ${property}`);
				console.error(data);
			}

			return prop; // ts sorcery
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
			const prop = get_index_prop(data, bind_index, property);

			if (prop === undefined) {
				console.error(`bind subscribe failed!, bind path: ${bind_path}, index: ${bind_index} property: ${property}`);
				console.error(data);
			}

			return prop; // ts sorcery
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
}

export default new SceneStore();
