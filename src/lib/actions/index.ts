import { ID, WebrayEditor } from '../editor';
import { a_download, a_load_file, a_save_file } from './file';
import { a_add_list_item, a_del_list_item } from './list';

import { a_full_screen_enter, a_full_screen_exit, a_render } from './render';

export function register_actions() {
	WebrayEditor.registerActionCallback(ID.a_render, a_render);

	WebrayEditor.registerActionCallback(ID.a_add_list_item, a_add_list_item);
	WebrayEditor.registerActionCallback(ID.a_del_list_item, a_del_list_item);

	WebrayEditor.registerActionCallback(ID.a_download, a_download);

	WebrayEditor.registerActionCallback(ID.a_save_file, a_save_file);
	WebrayEditor.registerActionCallback(ID.a_load_file, a_load_file);

	WebrayEditor.registerActionCallback(ID.a_full_screen_enter, a_full_screen_enter);
	WebrayEditor.registerActionCallback(ID.a_full_screen_exit, a_full_screen_exit);
}
