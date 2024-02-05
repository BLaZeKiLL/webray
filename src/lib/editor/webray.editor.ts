import _editor_json from '../../data/webray.editor.json';
import { register_actions } from '../actions';

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const actions: { [id: string]: [(params: any) => void] } = {};

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

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public static registerActionCallback(id: string, cb: (params: any) => void) {
		if (id in actions) {
			actions[id].push(cb);
		} else {
			actions[id] = [cb];
		}
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public static invokeAction(id: string, params: any = {}) {
		actions[id].forEach((cb) => cb(params));
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	public static getDefaultObj(type: string): any {
		return WebrayEditor.getDataType(type).properties.reduce((p, c) => {
			if (c.type === 'data_select') {
				const nested_obj = WebrayEditor.getDefaultObj(c.initial);
				nested_obj['type'] = c.initial;
				return { ...p, [c.name]: nested_obj };
			} else {
				return { ...p, [c.name]: c.initial };
			}
		}, {});
	}
}

export function initialize_editor() {
	register_actions();
}
