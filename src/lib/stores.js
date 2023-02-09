import { writable, readable } from "svelte/store";

export const main = new writable({
  balance: 0,
  block_count: 0,
});
export const zcash = new writable({
  id: 'zcash',
  name: 'Zcash',
  balance: 0,
  refundable: 0,
  block_count: 0,
});
export const ethereum = new writable({
  id: 'ethereum',
  name: 'Ethereum',
  balance: 0,
  refundable: 0,
  block_count: 0,
});
