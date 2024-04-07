<script lang="ts">
    import { type Channel, } from '$lib/channel';
    export let channel: Channel;
</script>

<div>
    <a href="/app/{channel.id}" class="flex flex-col items-center bg-white border border-gray-200 rounded-lg shadow md:flex-row md:max-w-xl hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
        <img class="object-cover w-full rounded-t-lg h-96 md:h-auto md:w-48 md:rounded-none md:rounded-s-lg"
            alt={channel.title}
            src={channel.image}>
        <div class="flex flex-col justify-between p-4 leading-normal">
            <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{channel.title}</h5>
            <p class="mb-3 font-normal text-center">{channel.description}</p>
            <p class="text-red-500 text-center">
                <a href={channel.url}>YouTube</a>
            </p>
        </div>
    </a>
</div>
