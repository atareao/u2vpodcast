<script lang="ts">
	import { GradientButton } from 'flowbite-svelte';
	import { EditSolid, TrashBinSolid } from 'flowbite-svelte-icons';
	import type { Channel } from '$lib/utils/types';

	export let channel: Channel;
	export let onUpdateChannelButtonClicked: any;
	export let onDeleteChannelButtonClicked: any;

	const inactiveClass =
		'bg-red-500 border-red-500 hover:bg-red-700 dark:border-red-700 dark:bg-red-800 dark:hover:bg-red-700';
	const activeClass =
		'bg-white border-gray-200 hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700';
	let firstDate;
	//$: firstDate = channel.first.split('T')[0];
</script>

<div class="border rounded-lg shadow m-2 p-2 border-gray-500 dark:border-white">
	<a href="/app/{channel.id}">
		<div
			class="flex flex-col items-center p-2 m-2 {channel.active
				? activeClass
				: inactiveClass}  rounded-lg shadow md:flex-row md:max-w-xl"
		>
			<img
				class="object-cover w-full rounded-lg h-96 md:h-auto md:w-48"
				alt={channel.title}
				src={channel.image}
			/>
			<div class="flex flex-col justify-between p-4 leading-normal">
				<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
					{channel.title}
				</h5>
				<p class="mb-3 font-normal text-center line-clamp-5 dark:text-gray-100">
					{channel.description}
				</p>
				<p class="text-center dark:text-gray-100 underline">
					<a href={channel.url}>YouTube</a>
				</p>
			</div>
		</div>
	</a>
	<div>
		<GradientButton
			class="mb-2"
			color="cyanToBlue"
			on:click={() => onUpdateChannelButtonClicked(channel)}
			pill
		>
			<EditSolid class="w-6 h-6" />
		</GradientButton>
		<GradientButton
			color="pinkToOrange"
			on:click={() => onDeleteChannelButtonClicked(channel)}
			pill
		>
			<TrashBinSolid class="w-6 h-6" />
		</GradientButton>
	</div>
</div>
