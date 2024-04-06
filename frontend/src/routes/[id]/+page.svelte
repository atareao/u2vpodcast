<script lang="ts">
    import type { PageData } from "./$types";
    import { type Episode, } from '$lib/episode';
    import EpisodeCard from '$lib/components/EpisodeCard.svelte';
    export let data: Episode[];

</script>

<div class="grid justify-items-center">
{#each data.episodes as episode}
    <EpisodeCard {episode}>
    </EpisodeCard>
{/each}
</div>



