import { writable } from 'svelte/store';

const createStore = <T extends { id: string }>() => {
  const store = writable<T[]>([]);

  return {
    subscribe: store.subscribe,
    add: (value: Exclude<T, 'id'>) => {
      const value_with_id = {
        ...value,
        id: crypto.randomUUID() as string
      };
      store.update(($store) => {
        $store.push(value_with_id);
        return $store;
      });
    }
  };
};

export default createStore;