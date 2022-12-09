var DialogMixin = {
  data() {
    return {
      dialog: false
    }
  },
  methods: {
    showDialog() {
      this.dialog = true;
    },
    closeDialog() {
      this.dialog = false;
    }
  }
};
