<script lang="ts">
    import { confirmDialogState } from "$lib/state/confirm_dialog.svelte";
    import { fade, scale } from "svelte/transition";

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape" && confirmDialogState.visible) {
            confirmDialogState.cancel();
        }
    }

    function handleBackdropClick() {
        confirmDialogState.cancel();
    }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if confirmDialogState.visible}
    <div
        class="fixed inset-0 z-[9998] bg-black/60 backdrop-blur-sm"
        transition:fade={{ duration: 120 }}
        onclick={handleBackdropClick}
        onkeydown={(e) => {
            if (e.key === "Enter" || e.key === " ") handleBackdropClick();
        }}
        role="presentation"
    ></div>

    <div
        class="fixed inset-0 z-[9999] flex items-center justify-center pointer-events-none"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirm-dialog-title"
    >
        <div
            class="pointer-events-auto w-full max-w-sm bg-[#141419] border border-surface-border rounded-2xl shadow-2xl p-6"
            transition:scale={{ duration: 150, start: 0.95 }}
            onclick={(e) => e.stopPropagation()}
        >
            <h3
                id="confirm-dialog-title"
                class="text-lg font-semibold text-white mb-2"
            >
                {confirmDialogState.title}
            </h3>

            <p class="text-sm text-gray-400 leading-relaxed mb-6">
                {confirmDialogState.message}
            </p>

            <div class="flex justify-end gap-3">
                <button
                    class="px-4 py-2 rounded-lg text-sm font-medium text-gray-300 bg-white/5 border border-surface-border hover:bg-white/10 transition-colors"
                    onclick={() => confirmDialogState.cancel()}
                >
                    {confirmDialogState.cancelLabel}
                </button>

                <button
                    class="px-4 py-2 rounded-lg text-sm font-medium transition-colors
                    {confirmDialogState.variant === 'danger'
                        ? 'bg-red-500/20 text-red-400 border border-red-500/30 hover:bg-red-500/30 hover:text-red-300'
                        : 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/30 hover:text-indigo-300'}"
                    onclick={() => confirmDialogState.confirm()}
                >
                    {confirmDialogState.confirmLabel}
                </button>
            </div>
        </div>
    </div>
{/if}
