export interface WebrayEditorConfig {
	windows: Windows;
	toolbars: Toolbars;
	actions: Actions;
}

export interface Windows {
	w_scene: Window;
	w_materials: Window;
	w_camera: Window;
	w_render_settings: Window;
}

export interface Window {
	icon: string;
	label?: string;
	tooltip: string;
}

export interface Toolbars {
	t_app_bar: Toolbar;
	t_image_bar: Toolbar;
}

export interface Toolbar {
	lead: Tool[];
	center: Tool[];
	trail: Tool[];
}

export interface Tool {
	icon: string;
	label?: string;
	tooltip: string;
	action: string;
}

export interface Actions {
	a_download: Action;
	a_render: Action;
}

export interface Action {}
