import scene from '../store/scene.store';

export function a_add_list_item(params: { bind_path: string }) {
    scene.add_list_item(params.bind_path);
}

export function a_del_list_item(params: { bind_path: string }) {
    scene.del_list_item(params.bind_path);
}

