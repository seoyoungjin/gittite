<template>
  <div class="inline flex">
    {{ relativeText }}
    <q-tooltip :class="className">
      {{ absoluteText }}
    </q-tooltip>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { formatDate } from "../lib/format-date";
import { formatRelative } from "../lib/format-relative";

const SECOND = 1000;
const MINUTE = SECOND * 60;
const HOUR = MINUTE * 60;
const DAY = HOUR * 24;

// The maximum value that can be used in setInterval or
// setTimeout without it overflowing (2 ^ 31 - 1). See
//  http://stackoverflow.com/a/16314807
const MAX_INTERVAL = 2147483647;

export default defineComponent({
  name: "RelativeTime",

  props: {
    date: {
      type: Date,
      default: Date(),
    },
    /**
     * For relative durations, use abbreviated units
     * ('m' instead of 'minutes', 'd' instead of 'days')
     *
     * Defaults to `false`
     */
    abbreviate: {
      type: Boolean,
      default: false,
    },
    /**
     * By default the RelativeTime component will start displaying a compact
     * absolute date if the date is more than one week ago. Setting `onlyRelative`
     * to true overrides this behavior and forces relative times for all dates.
     */
    onlyRelative: {
      type: Boolean,
      default: false,
    },
    className: {
      type: String,
      default: "bg-black text-body2",
    },
  },

  data() {
    return {
      absoluteText: "",
      relativeText: "",
      timer_: null as number | null,
    };
  },

  mounted() {
    // this.relativeText = formatRelative(this.date - Date.now());
    this.updateWithDate(this.date);
  },

  unmounted() {
    this.clearTimer();
  },

  methods: {
    clearTimer: function () {
      if (this.timer_) {
        clearTimeout(this.timer_);
        this.timer_ = null;
      }
    },

    updateAndSchedule(
      absoluteText: string,
      relativeText: string,
      timeout: number
    ) {
      this.clearTimer();
      this.timer_ = setTimeout(
        this.updateFromScheduler,
        Math.min(timeout, MAX_INTERVAL)
      );
      this.absoluteText = absoluteText;
      this.relativeText = relativeText;
    },

    updateWithDate(then: Date) {
      const onlyRelative = this.onlyRelative;
      const diff = then.getTime() - Date.now();
      const duration = Math.abs(diff);
      const absoluteText = formatDate(then, {
        dateStyle: "full",
        timeStyle: "short",
      } as Intl.DateTimeFormatOptions);
      const relativeText = formatRelative(diff);

      // Future date, let's just show as absolute and reschedule. If it's less
      // than a minute into the future we'll treat it as 'just now'.
      if (diff > 0 && duration > MINUTE) {
        this.updateAndSchedule(
          absoluteText,
          formatDate(then, {
            dateStyle: "medium",
            timeStyle: "short",
          } as Intl.DateTimeFormatOptions),
          duration
        );
      } else if (duration < MINUTE) {
        this.updateAndSchedule(absoluteText, "just now", MINUTE - duration);
      } else if (duration < HOUR) {
        this.updateAndSchedule(absoluteText, relativeText, MINUTE);
      } else if (duration < DAY) {
        this.updateAndSchedule(absoluteText, relativeText, HOUR);
      } else if (duration < 7 * DAY) {
        this.updateAndSchedule(absoluteText, relativeText, 6 * HOUR);
      } else {
        if (onlyRelative) {
          this.updateAndSchedule(absoluteText, relativeText, 6 * HOUR);
        } else {
          // More than a week ago, just the date will suffice
          this.absoluteText = absoluteText;
          this.relativeText = formatDate(then, {
            dateStyle: "medium",
          } as Intl.DateTimeFormatOptions);
        }
      }
    },

    updateFromScheduler() {
      this.updateWithDate(this.date);
    },
  },
});
</script>
