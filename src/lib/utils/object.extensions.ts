// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function get_prop(obj: any, prop: string, separator = '.'): any {
	return prop.split(separator).reduce((value, el) => value[el], obj);
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function set_prop(obj: any, prop: string, val: any, separator = '.'): void {
	const path = prop.split(separator);

	path.reduce((value, el, level) => {
		if (level === path.length - 1) {
			value[el] = val;
			return value;
		} else {
			return value[el];
		}
	}, obj);
}
