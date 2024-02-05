import { derived, get, type Writable } from 'svelte/store';
import { get_prop, set_prop } from '../utils/object.extensions';
import { tick } from 'svelte';

// TODO: instead of property use selector functions
export function writable_derived<S, D>(store: Writable<S>, property: string, name = 'store') {
	const { subscribe } = derived(store, (state) => {
		if (state === undefined) {
			return; // item no longer exists
		}

		const prop = get_prop(state, property);

		if (prop === undefined) {
			tick().then(() => {
				if (get_prop(state, property) === undefined) {
					console.error(`${name}: bind subscribe failed!, property: ${property}`);
					console.error(state);
				}
			});
		}

		return prop; // ts sorcery
	});

	const update = (value: D) => {
		store.update((state) => {
			// this is a shallow copy
			// can use structuredClone if a deep copy is required
			const change = { ...state };

			set_prop(change, property, value);

			return change;
		});
	};

	const set = (value: D) => {
		// this is a shallow copy
		// can use structuredClone if a deep copy is required
		const change = { ...get(store) };

		set_prop(change, property, value);

		store.set(change);
	};

	return {
		name,
		subscribe,
		update,
		set
	} as Writable<D>;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function writable_indexed_derived<D>(store: Writable<any[]>, index: number, name = 'store') {
	const { subscribe } = derived(store, (state) => {
		if (state === undefined) {
			return; // item no longer exists
		}

		const prop = state[index];

		if (prop === undefined) {
			tick().then(() => {
				if (state[index] === undefined) {
					console.error(`${name}: bind subscribe failed!, index: ${index}`);
					console.error(state);
				}
			});
		}

		return prop; // ts sorcery
	});

	const update = (value: D) => {
		store.update((state) => {
			// this is a shallow copy
			// can use structuredClone if a deep copy is required
			const change = [...state];

			change[index] = value;

			return change;
		});
	};

	const set = (value: D) => {
		// this is a shallow copy
		// can use structuredClone if a deep copy is required
		const change = [...get(store)];

		change[index] = value;

		store.set(change);
	};

	return {
		name,
		subscribe,
		update,
		set
	} as Writable<D>;
}
