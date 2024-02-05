// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function get_id_prop(arr: any[], id: number, prop: string, separator = '.'): any {
	const element = arr.find(item => item.id === id);
	return element === undefined ? undefined : prop.split(separator).reduce((value, el) => value[el], element);
}

export function set_index_prop(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	arr: any[],
	id: number,
	prop: string,
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	val: any,
	separator = '.'
): void {
	const path = prop.split(separator);
	const element = arr.find(item => item.id === id);

	if (element === undefined) return;

	path.reduce((value, el, level) => {
		if (level === path.length - 1) {
			value[el] = val;
			return value;
		} else {
			return value[el];
		}
	}, element);
}
