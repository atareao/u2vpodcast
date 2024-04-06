<script lang="ts">
  import VolumeMute from '$lib/components/icons/VolumeMute.svelte';
  import VolumeUp from '$lib/components/icons/VolumeUp.svelte';
  import RangeSlider from '$lib/components/player/RangeSlider.svelte';

  import { getAudioContext } from '$lib';
  import { toggle } from '$lib/utils';

  const { volume, muted } = getAudioContext();

  let volumePercentage = 100;
  $: $volume = volumePercentage / 100;
</script>

<div class="volume flex justify-center relative">
  <button on:click={() => toggle(muted)}>
    {#if $muted}
      <VolumeMute class="w-6 text-red-300" />
    {:else}
      <VolumeUp class="w-6 text-red-300" />
    {/if}
  </button>

  <div class="volume-control w-16 origin-left -rotate-90 absolute -top-1 left-[50%]">
    <RangeSlider bind:value={volumePercentage} max={100} step={1} />
  </div>
</div>

<style lang="postcss">
  .volume-control {
    @apply opacity-0 transition-opacity;
  }

  .volume:hover .volume-control {
    @apply opacity-100;
  }
</style>
