import { ID, WebrayEditor } from '../editor';
import { a_add_list_item, a_del_list_item } from './list';

import { a_render } from './render';

export function register_actions() {
	WebrayEditor.registerActionCallback(ID.a_render, a_render);

	WebrayEditor.registerActionCallback(ID.a_add_list_item, a_add_list_item);
	WebrayEditor.registerActionCallback(ID.a_del_list_item, a_del_list_item);
}
