<script lang="ts">
    import { base_endpoint } from '$lib/global';
    import { type Episode, } from '$lib/episode';
    import CustomAudioPlayer from '$lib/components/player/CustomAudioPlayer.svelte';
    import AudioPlayer from '$lib/components/player/AudioPlayer.svelte';
    export let episode: Episode;
</script>

<div class="border rounded-lg shadow m-2 p-2 border-gray-500 dark:border-white">
    <div
        class="flex flex-col items-center p-2 m-2  rounded-lg shadow md:flex-row md:max-w-xl">
        <img
            class="object-cover w-full rounded-lg h-96 md:h-auto md:w-48"
            alt={episode.title}
            src={episode.image}
        />
        <div class="flex flex-col justify-between p-4 leading-normal">
            <h6 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white line-clamp-2">
                {episode.title}
            </h6>
            <p class=" max-w-80 line-clamp-5 dark:text-gray-100">
                {episode.description}
            </p>
        </div>
    </div>
    <AudioPlayer src="{base_endpoint}/media/{episode.channel_id}/{episode.yt_id}.mp3">
        <CustomAudioPlayer />
    </AudioPlayer>
</div>
