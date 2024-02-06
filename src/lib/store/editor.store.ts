import { writable } from "svelte/store";
import { KernelState } from "../types";

export interface WebrayEditorState {
    kernel_state: KernelState;
}

function createWebrayEditorStore() {
    const { subscribe, update } = writable<WebrayEditorState>({kernel_state: KernelState.INITIAL});

    const update_kernel_state = (kernel_state: KernelState) => {
        update((state) => {
            return { ...state, kernel_state: kernel_state };
        })
    }  

    return {
        subscribe,
        update_kernel_state
    };
}

const store = createWebrayEditorStore();

export const editorStore = store;