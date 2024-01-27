/* eslint-disable @typescript-eslint/no-explicit-any */
import Scene from '~icons/iconamoon/3d-light';
import Download from '~icons/uil/image-download';
import Render from '~icons/material-symbols/rocket-launch';
import Camera from '~icons/material-symbols/android-camera-outline';
import RenderSettings from '~icons/cil/tv';
import Material from '~icons/icon-park-outline/material';
import FullScreenEnter from '~icons/gridicons/fullscreen';
import FullScreenExit from '~icons/gridicons/fullscreen-exit'

const icons = {
	i_scene: Scene,
	i_materials: Material,
	i_render: Render,
	i_download: Download,
	i_camera: Camera,
	i_render_settings: RenderSettings,
	i_full_screen_enter: FullScreenEnter,
	i_full_screen_exit: FullScreenExit
} as {
	[key: string]: any;
};

export class Icons {
	public static getIcon(id: string): any {
		return icons[id];
	}
}
