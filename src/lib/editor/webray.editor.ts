import _editor_json from '../../webray.editor.json';

import type {
	Action,
	Actions,
	Toolbar,
	Toolbars,
	WebrayEditorConfig,
	Window,
	Windows
} from './webray.interfaces';

const editor_json = _editor_json as unknown as WebrayEditorConfig;

export class WebrayEditor {
	public static getWindow(id: string): Window {
		return editor_json.windows[id as keyof Windows] as Window;
	}

	public static getToolbar(id: string): Toolbar {
		return editor_json.toolbars[id as keyof Toolbars] as Toolbar;
	}

	public static getAction(id: string): Action {
		return editor_json.actions[id as keyof Actions] as Action;
	}
}
