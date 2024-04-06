<script lang="ts">
    import { type Episode, } from '$lib/episode';
    import CustomAudioPlayer from '$lib/components/player/CustomAudioPlayer.svelte';
    import AudioPlayer from '$lib/components/player/AudioPlayer.svelte';
    export let episode: Episode;
    import { Card } from 'flowbite-svelte';
</script>

<div class="mb-8">
  <Card img={episode.image} horizontal size="md">
    <h5 class="mb-2 text-lg font-bold tracking-tight text-gray-900 dark:text-white">
        {episode.title}
    </h5>
    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
        {episode.description}
    </p>
    <AudioPlayer src="/audios/{episode.channel_id}/{episode.yt_id}.mp3">
        <CustomAudioPlayer />
    </AudioPlayer>
    <p class="text-center text-red-500">
    <a href="https://www.youtube.com/watch?v={episode.yt_id}">YouTube</a>
    </p>
  </Card>
</div>
