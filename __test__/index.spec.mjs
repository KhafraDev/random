import test from 'ava';

import { getRandom } from '../index.js';

const isUint8 = (value) => Number.isInteger(value) && value >= 0 && value <= 255;

test('getRandom', (t) => {
  const random = getRandom();

  t.is(random.length, 2);
  t.is(isUint8(random[0]), true);
  t.is(isUint8(random[1]), true);
})
