import { useQuasar } from "quasar";
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

export default {
  name: 'ProgressDialog',
  render () {
    const $q = useQuasar()

    function showProgress () {
      const dialog = $q.dialog({
        message: 'Uploading... 0%',
        progress: true, // we enable default settings
        persistent: true, // we want the user to not be able to close it
        ok: false // we want the user to not be able to close it
      })

      // we simulate some progress here...
      let percentage = 0
      const interval = setInterval(() => {
        percentage = Math.min(100, percentage + Math.floor(Math.random() * 22))

        // we update the dialog
        dialog.update({
          message: `Uploading... ${percentage}%`
        })

        // if we are done, we're gonna close it
        if (percentage === 100) {
          clearInterval(interval)
          setTimeout(() => {
            dialog.hide()
          }, 350)
        }
      }, 500)
    }
    return { showProgress }
  },
}
