import { derived, get, type Writable } from "svelte/store";
import { get_prop, set_prop } from "../utils/object.extensions";

// TODO: instead of property use selector functions
export function writable_derived<S, D>(store: Writable<S>, property: string) {
    const { subscribe } = derived(store, (state) => {
        const prop = get_prop(state, property);
        return prop; // ts sorcery
    });

    const update = (value: D) => {
        store.update((state) => {
            // this is a shallow copy
            // can use structuredClone if a deep copy is required
            const change = { ...state };

            set_prop(state, property, value);

            return change;
        });
    };

    const set = (value: D) => {
        // this is a shallow copy
        // can use structuredClone if a deep copy is required
        const change = { ...get(store) };

        set_prop(change, property, value);

        store.set(change);
    }

    return {
        subscribe,
        update,
        set
    } as Writable<D>;
}