<script lang="ts">
    let {
        text,
        position = "top",
        class: className = "",
    }: {
        text: string;
        position?: "top" | "left" | "right" | "bottom";
        class?: string;
    } = $props();

    const positionClasses = $derived.by(() => {
        switch (position) {
            case "top":
                return "bottom-full left-1/2 -translate-x-1/2 mb-2";
            case "bottom":
                return "top-full left-1/2 -translate-x-1/2 mt-2";
            case "left":
                return "right-full top-1/2 -translate-y-1/2 mr-2";
            case "right":
                return "left-full top-1/2 -translate-y-1/2 ml-2";
            default:
                return "bottom-full left-1/2 -translate-x-1/2 mb-2";
        }
    });
</script>

<div
    class="absolute {positionClasses} px-2 py-1 bg-surface-elevated border border-white/10 rounded text-[10px] font-medium text-text-primary opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50 shadow-xl {className}"
>
    {text}
</div>
