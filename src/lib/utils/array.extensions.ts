// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function get_index_prop(arr: any[], index: number, prop: string, separator = '.'): any {
	return prop.split(separator).reduce((value, el) => value[el], arr[index]);
}

export function set_index_prop(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	arr: any[],
	index: number,
	prop: string,
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	val: any,
	separator = '.'
): void {
	const path = prop.split(separator);

	path.reduce((value, el, level) => {
		if (level === path.length - 1) {
			value[el] = val;
			return value;
		} else {
			return value[el];
		}
	}, arr[index]);
}
