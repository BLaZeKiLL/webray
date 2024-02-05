import scene from '$lib/store/scene.store';
import { parse_scene } from '$lib/wasm/webray';

export function a_render() {
	parse_scene(scene.current);
}
