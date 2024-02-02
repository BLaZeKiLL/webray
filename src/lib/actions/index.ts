import { ID, WebrayEditor } from '../editor';

import { a_render } from './render';

export function register_actions() {
	WebrayEditor.registerActionCallback(ID.a_render, a_render);
}
