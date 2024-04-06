<script lang="ts">
  export let min = 0;
  export let max = 1;
  export let value;
  export let step = 0.01;
</script>

<div class="relative flex items-center flex-1">
  <div class="h-1 w-full rounded-sm bg-red-300 absolute" />
  <div
    class="h-1 rounded-m bg-gradient-to-r from-red-300 to-red-500 absolute"
    style={`width: ${(value * 100) / max}%;`}
  />
  <input
    bind:value
    {min}
    {max}
    {step}
    type="range"
    class="w-full absolute h-1 appearance-none bg-transparent outline-none"
  />
</div>

<style>
  input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    @apply w-1 h-1 cursor-pointer;
  }

  input[type='range']::-webkit-slider-thumb:hover {
    @apply bg-red-600;
  }
</style>
