import _editor_json from '../../webray.editor.json';

import type {
	WebrayAction,
	WebrayActions,
	WebrayToolbar,
	WebrayToolbars,
	WebrayEditorConfig,
	WebrayWindow,
	WebrayWindows,
	WebrayDataType,
	WebrayDataTypes
} from './webray.interfaces';

const editor_json = _editor_json as unknown as WebrayEditorConfig;

export class WebrayEditor {
	public static getWindow(id: string): WebrayWindow {
		return editor_json.windows[id as keyof WebrayWindows] as WebrayWindow;
	}

	public static getToolbar(id: string): WebrayToolbar {
		return editor_json.toolbars[id as keyof WebrayToolbars] as WebrayToolbar;
	}

	public static getAction(id: string): WebrayAction {
		return editor_json.actions[id as keyof WebrayActions] as WebrayAction;
	}

	public static getDataType(id: string): WebrayDataType {
		return editor_json.data_types[id as keyof WebrayDataTypes] as WebrayDataType;
	}
}
