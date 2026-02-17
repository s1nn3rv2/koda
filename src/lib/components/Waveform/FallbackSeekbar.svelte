<script lang="ts">
    interface Props {
        progress: number;
    }

    let { progress }: Props = $props();
</script>

<div class="relative h-full w-full">
    <div
        class="absolute inset-0 bg-gradient-to-r from-indigo-500/30 to-purple-500/30 transition-all"
        style="width: {progress}%"
    ></div>
</div>
