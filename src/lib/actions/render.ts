import scene from '$lib/store/scene.store';
import { render } from '$lib/wasm/webray';
import { editorStore } from '../store/editor.store';
import { KernelState } from '../types';

export function a_render() {
	editorStore.update_kernel_state(KernelState.RENDERING)
	render(scene.current).then(() => editorStore.update_kernel_state(KernelState.DONE));
}
