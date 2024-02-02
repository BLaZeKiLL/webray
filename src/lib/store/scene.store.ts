import { writable, derived, get, type Writable } from 'svelte/store';
import { get_prop, set_prop } from '../utils/object.extensions';
import type { WebrayScene } from '../editor/webray.scene';
import _demo_json from '../../data/demo_01.scene.json';


export class SceneStore {
	private store;

	constructor() {
		this.store = writable<WebrayScene>(_demo_json);
	}

	public get scene() {
		return get(this.store);
	}

	public bind<T>(path: string, property: string) {
		const parts = path.split(':');

		if (!(parts[0] === 'webray' && parts[1] === 'scene')) {
			console.error(`Bind path ${path} not defined`);
			return undefined;
		}

		const bind_path = parts[2]; // last part
		const initial = get(this.store);

		if (!(bind_path in initial)) {
			console.error(`Bind path ${path} not defined`);
			return undefined;
		}

		const { subscribe } = derived(this.store, (state) => {
			const data = state[bind_path as keyof WebrayScene];
			const prop = get_prop(data, property);
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
}

export default new SceneStore();
