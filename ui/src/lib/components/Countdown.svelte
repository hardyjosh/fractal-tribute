<script lang="ts">
  import type { CountdownObject } from "$lib/stores/countdown";

  export let countdown: CountdownObject;
</script>

<div class="flex items-end space-x-2">
  <!-- Days -->
  <div class="text-center">
    <div class="inline-flex space-x-1 text-primary-700 font-bold">
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.days[0]}
      </div>
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.days[1]}
      </div>
    </div>
    <p>days</p>
  </div>

  <!-- Separator -->
  <div class="self-center mb-6 text-2xl font-bold text-primary-700">:</div>

  <!-- Hours -->
  <div class="text-center">
    <div class="inline-flex space-x-1 text-primary-700 font-bold">
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.hours[0]}
      </div>
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.hours[1]}
      </div>
    </div>
    <p>hours</p>
  </div>

  <!-- Separator -->
  <div class="self-center mb-6 text-2xl font-bold text-primary-700">:</div>

  <!-- Minutes -->
  <div class="text-center">
    <div class="inline-flex space-x-1 text-primary-700 font-bold">
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.minutes[0]}
      </div>
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.minutes[1]}
      </div>
    </div>
    <p>minutes</p>
  </div>

  <!-- Separator -->
  <div class="self-center mb-6 text-2xl font-bold text-primary-700">:</div>

  <!-- Seconds -->
  <div class="text-center">
    <div class="inline-flex space-x-1 text-primary-700 font-bold">
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.seconds[0]}
      </div>
      <div
        class="bg-primary-200 rounded-lg w-10 h-12 flex items-center justify-center text-2xl"
      >
        {countdown.seconds[1]}
      </div>
    </div>
    <p>seconds</p>
  </div>
</div>
