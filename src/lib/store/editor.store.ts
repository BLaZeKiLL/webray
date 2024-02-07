import { writable } from 'svelte/store';
import { KernelState, OutputView } from '../types';
import type { ToastStore } from '@skeletonlabs/skeleton';

export interface WebrayEditorState {
	kernel_state: KernelState;
	output_view: OutputView;
	toaster?: ToastStore;
}

function createWebrayEditorStore() {
	const { subscribe, update } = writable<WebrayEditorState>({
		kernel_state: KernelState.INITIAL,
		output_view: OutputView.DEFAULT,
		toaster: undefined
	});

	const update_kernel_state = (kernel_state: KernelState) => {
		update((state) => {
			return { ...state, kernel_state };
		});
	};

	const update_output_view = (output_view: OutputView) => {
		update((state) => {
			return { ...state, output_view };
		});
	};

	const set_toaster = (toaster: ToastStore) => {
		update((state) => {
			return { ...state, toaster };
		});
	};

	return {
		subscribe,
		set_toaster,
		update_output_view,
		update_kernel_state
	};
}

const store = createWebrayEditorStore();

export const editorStore = store;
