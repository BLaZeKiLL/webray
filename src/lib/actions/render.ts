import scene from '$lib/store/scene.store';
import { render } from '$lib/wasm/webray';
import { editorStore } from '../store/editor.store';
import { KernelState, OutputView } from '../types';

export function a_render() {
	editorStore.update_kernel_state(KernelState.RENDERING);

	render(scene.current).then(() => editorStore.update_kernel_state(KernelState.DONE));
}

export function a_full_screen_enter() {
	editorStore.update_output_view(OutputView.FULLSCREEN);
}

export function a_full_screen_exit() {
	editorStore.update_output_view(OutputView.DEFAULT);
}
