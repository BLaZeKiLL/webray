// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function get_prop(obj: any, prop: string): any {
    return prop.split('.').reduce((value, el) => value[el], obj);
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function set_prop(obj: any, prop: string, val: any): void {
    const path = prop.split('.');

    path.reduce((value, el, level) => {
        if (level === path.length - 1) {
            value[el] = val;
            return value;
        } else {
            return value[el];
        }
    }, obj);
}