import { ref } from "vue";

export const compRef = <T extends abstract new (...args: any) => any>(_: T) => ref<InstanceType<T>>()
