import _editor_json from '../../webray.editor.json';
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

const actions: {[id: string]: [() => void]} = {};

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

	public static registerActionCallback(id: string, cb: () => void) {
		if (id in actions) {
			actions[id].push(cb);
		} else {
			actions[id] = [cb];
		}
	}

	public static invokeAction(id: string) {
		actions[id].forEach(cb => cb());
	}
}

export function initialize_editor() {
	register_actions();
}