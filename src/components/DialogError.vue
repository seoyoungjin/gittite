<template>
  <q-card-section>
    <q-icon class="q-pr-sm" :name="octAlert16" size="14pt" color="yellow-7" />
    {{ message }}
  </q-card-section>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { octAlert16 } from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "DialogError",

  setup() {
    return {
      octAlert16,
    };
  },

  props: {
    message: {
      type: String,
    },
  },
});
</script>
