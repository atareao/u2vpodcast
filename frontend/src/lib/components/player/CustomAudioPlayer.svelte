<script lang="ts">
  import PauseIcon from '$lib/components/icons/PauseIcon.svelte';
  import PlayIcon from '$lib/components/icons/PlayIcon.svelte';
  import RepeatIcon from '$lib/components/icons/RepeatIcon.svelte';
  import RewindIcon from '$lib/components/icons/RewindIcon.svelte';
  import SpeedIcon from '$lib/components/icons/SpeedIcon.svelte';

  import { getAudioContext } from '$lib/context';
  import { toggle, toHHMMSS } from '$lib/utils';
  import RangeSlider from './RangeSlider.svelte';
  import VolumeControl from './VolumeControl.svelte';

  const SEEK_SECONDS = 10;
  const PLAYBACK_SPEEDS = [1, 1.25, 1.5, 1.75, 2, 0.25, 0.5, 0.75];

  const { playing, playbackRate, paused, repeat, seekBy, currentTime, duration } =
    getAudioContext();

  let speedIndex = 0;

  const handlePlaybackSpeedClick = () => {
    $playbackRate = PLAYBACK_SPEEDS[++speedIndex % PLAYBACK_SPEEDS.length];
  };
</script>

<div
  class="h-30 w-[320px] p-2 flex flex-col justify-center items-center"
>
  <div class="flex items-center space-x-3">
    <button on:click={() => toggle(repeat)}>
      <RepeatIcon class={`w-4 ${$repeat ? 'text-red-500' : 'text-red-300'}`} />
    </button>

    <button on:click={() => seekBy(-1 * SEEK_SECONDS)}>
      <RewindIcon class="w-4 text-red-300" />
    </button>
    <button
      on:click={() => toggle(paused)}
      class="w-10 h-10 rounded-full flex justify-center items-center bg-gradient-to-br from-red-300 to-red-500"
    >
      {#if $playing}
        <PauseIcon />
      {:else}
        <PlayIcon />
      {/if}
    </button>

    <button on:click={() => seekBy(SEEK_SECONDS)}>
      <SpeedIcon class="w-4 text-red-300" />
    </button>

    <button
      class="w-8 h-6 flex items-center justify-center rounded-full bg-red-400"
      on:click={handlePlaybackSpeedClick}
    >
      <span class="text-[10px] font-semibold text-white">{$playbackRate}x</span>
    </button>
  </div>

  <div class="mt-4 w-full flex items-center space-x-2">
    <span class="text-sm text-800">{toHHMMSS($currentTime)}</span>
    <RangeSlider max={$duration} bind:value={$currentTime} />
    <span class="text-sm text-800">{toHHMMSS($duration)}</span>
    <VolumeControl />
  </div>
</div>
