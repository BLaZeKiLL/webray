import { writable } from "svelte/store";
import { KernelState } from "../types";
import type { ToastStore } from "@skeletonlabs/skeleton";

export interface WebrayEditorState {
    kernel_state: KernelState;
    toaster?: ToastStore
}

function createWebrayEditorStore() {
    const { subscribe, update } = writable<WebrayEditorState>({kernel_state: KernelState.INITIAL, toaster: undefined});

    const update_kernel_state = (kernel_state: KernelState) => {
        update((state) => {
            return { ...state, kernel_state: kernel_state };
        });
    }
    
    const set_toaster = (toaster: ToastStore) => {
        update((state) => {
            return { ...state, toaster: toaster };
        });
    }

    return {
        subscribe,
        set_toaster,
        update_kernel_state
    };
}

const store = createWebrayEditorStore();

export const editorStore = store;