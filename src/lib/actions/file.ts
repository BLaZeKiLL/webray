import { get } from 'svelte/store';

import { editorStore } from '../store/editor.store';
import scene from '../store/scene.store';
import { KernelState } from '../types';

export function a_download() {
	const editor = get(editorStore);

	if (editor.kernel_state !== KernelState.DONE) {
		editor.toaster!.trigger({
			message: 'Render a image before saving',
			background: 'variant-ghost-warning'
		});

		return;
	}

	const img = document.getElementById('output-image-target') as HTMLImageElement;

	download_file('render.png', img.src);
}

export function a_save_file() {
	const href = `data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify(scene.current, null, 4))}`;

	download_file('scene.json', href);
}

export function a_load_file() {
	const input = document.createElement('input') as HTMLInputElement;
	input.style.display = 'none';
	input.type = 'file';

	input.onchange = () => {
		const editor = get(editorStore);

		if (input.files === null || input.files.length <= 0) {
			editor.toaster!.trigger({
				message: 'Select a valid json file to upload',
				background: 'variant-ghost-error'
			});
			input.remove();
			return;
		}

		const reader = new FileReader();

		reader.onload = (e) => {
			if (e.target === null) {
				editor.toaster!.trigger({
					message: 'Select a valid json file to upload',
					background: 'variant-ghost-error'
				});
				input.remove();
				return;
			}

			try {
				const scene_json = JSON.parse(e.target.result as string);
				scene.import_scene(scene_json);
				input.remove();
			} catch (e) {
				editor.toaster!.trigger({
					message: 'Select a valid json file to upload',
					background: 'variant-ghost-error'
				});

				console.error(e);
				input.remove();
				return;
			}
		};

		reader.readAsText(input.files[0]);
	};

	input.click();
}

function download_file(name: string, href: string) {
	const a = document.createElement('a');
	a.style.display = 'none';
	a.href = href;
	a.download = name;
	a.click();
	a.remove();
}
