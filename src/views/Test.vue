<script>
import ProgressDialog from "@/components/dialog/ProgressDialog.vue";

export default {
  components: {
    ProgressDialog,
  },

  data() {
    return {
      showProgress: false,
    };
  },
};
</script>

<template>
  <h4>Test</h4>

  <div>
    <q-btn label="Progress" @click="showProgress=true" color="primary" />
  </div>

  <br />

  <ProgressDialog v-model="showProgress" />
</template>
