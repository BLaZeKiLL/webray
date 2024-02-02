import { describe, it, expect } from 'vitest';
import { get_index_prop, set_index_prop } from './array.extensions';

describe('scene test', () => {
	describe('get_index_prop', () => {
		const arr = [
			{
				a: 'prop_a',
				b: {
					c: 'prop_c'
				}
			}
		];

		it('should get simple prop', () => {
			expect(get_index_prop(arr, 0, 'a')).toBe(arr[0].a);
		});

		it('should get nested prop', () => {
			expect(get_index_prop(arr, 0, 'b.c')).toBe(arr[0].b.c);
		});
	});

	describe('set_index_prop', () => {
		const arr = [
			{
				a: 'prop_a',
				b: {
					c: 'prop_c'
				}
			}
		];

		it('should set simple prop', () => {
			set_index_prop(arr, 0, 'a', 'set_a');

			expect(arr[0].a).toBe('set_a');
		});

		it('should set nested prop', () => {
			set_index_prop(arr, 0, 'b.c', 'set_c');

			expect(arr[0].b.c).toBe('set_c');
		});
	});
});
