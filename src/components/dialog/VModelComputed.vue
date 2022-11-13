<template>
  <q-dialog v-model="model">
    <q-card style="max-width: 761px; width: 100%; max-height: 820px">
      <q-card-section class="row items-center no-wrap"> ... </q-card-section>
    </q-card>
  </q-dialog>
</template>

<script>
import { computed } from "vue";
export default {
  name: "VModelComputed",
  props: {
    modelValue: { type: Boolean, required: true },
  },
  emits: ["update:modelValue"],
  setup(props, { emit }) {
    const model = computed({
      get() {
        return props.modelValue;
      },
      set(newValue) {
        emit("update:modelValue", newValue);
      },
    });

    return { model };
  },
};

// dialog v-model and forward it down to q-dialog using computed
//
// import VModelComputed from "@/components/dialog/VModelComputed.vue";
// ...
// components: {
//    VModelComputed,
// },
// ...
// data() {
//   return {
//     ...
//     isOpenVModelComputed: false
//   };
// },
// ...
// <VModelComputed v-model='isOpenVModelComputed' />
// ...
// <q-btn @click="isOpenVModelComputed=true">Show Dialog</q-btn>
//
</script>
