<script lang="ts">
    import type { PageData } from "./$types";
    import type { Response, Episode } from '$lib/types';
    import EpisodeCard from '$lib/components/EpisodeCard.svelte';
    export let data: Episode[];

</script>

<div class="grid justify-items-center">
{#each data.episodes as episode}
    <EpisodeCard {episode}>
    </EpisodeCard>
{/each}
</div>



