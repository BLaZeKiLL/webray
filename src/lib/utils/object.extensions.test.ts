import { describe, it, expect } from 'vitest';
import { get_prop, set_prop } from './object.extensions';

describe('binder test', () => {
    describe('get_prop', () => {
        const obj = {
            a: 'prop_a',
            b: {
                c: 'prop_c'
            }
        }

        it('should get simple prop', () => {
            expect(get_prop(obj, 'a')).toBe(obj.a);
        });

        it('should get nested prop', () => {
            expect(get_prop(obj, 'b.c')).toBe(obj.b.c);
        });
    });

    describe('set_prop', () => {
        const obj = {
            a: 'prop_a',
            b: {
                c: 'prop_c'
            }
        }

        it('should set simple prop', () => {
            set_prop(obj, 'a', 'set_a')

            expect(obj.a).toBe('set_a');
        });

        it('should set nested prop', () => {
            set_prop(obj, 'b.c', 'set_c')

            expect(obj.b.c).toBe('set_c');
        });
    });
});