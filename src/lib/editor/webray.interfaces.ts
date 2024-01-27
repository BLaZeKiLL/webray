export interface WebrayEditorConfig {
	windows: WebrayWindows;
	toolbars: WebrayToolbars;
	actions: WebrayActions;
	data_types: WebrayDataTypes;
}

export interface WebrayWindows {
	w_scene: WebrayWindow;
	w_materials: WebrayWindow;
	w_camera: WebrayWindow;
	w_render_settings: WebrayWindow;
}

export interface WebrayWindow {
	icon: string;
	label?: string;
	tooltip: string;
	data: WebrayData;
}

export interface WebrayData {
    type: string;
    binding: string;
    data_type: string;
}

export interface WebrayToolbars {
	t_app_bar: WebrayToolbar;
	t_image_bar: WebrayToolbar;
}

export interface WebrayToolbar {
	lead: WebrayTool[];
	center: WebrayTool[];
	trail: WebrayTool[];
}

export interface WebrayTool {
	icon: string;
	label?: string;
	tooltip: string;
	action: string;
}

export interface WebrayActions {
	a_download: WebrayAction;
	a_render: WebrayAction;
}

export interface WebrayAction {}

export interface WebrayDataTypes {
    d_sphere: WebrayDataType;
    d_mat_diffuse: WebrayDataType;
    d_mat_metal: WebrayDataType;
    d_mat_dielectric: WebrayDataType;
    d_camera: WebrayDataType;
    d_render_settings: WebrayDataType;
}

export interface WebrayDataType {
    properties: WebrayProperty[];
}

export interface WebrayProperty {
    label: string;
    tooltip: string;
    type: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    default: any;
}

