interface ConfirmDialogOptions {
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: 'default' | 'danger';
    onConfirm: () => void;
    onCancel?: () => void;
}

class ConfirmDialogState {
    visible = $state(false);
    title = $state('');
    message = $state('');
    confirmLabel = $state('Confirm');
    cancelLabel = $state('Cancel');
    variant = $state<'default' | 'danger'>('default');
    private _onConfirm: (() => void) | null = null;
    private _onCancel: (() => void) | null = null;

    open(options: ConfirmDialogOptions) {
        this.title = options.title;
        this.message = options.message;
        this.confirmLabel = options.confirmLabel ?? 'Confirm';
        this.cancelLabel = options.cancelLabel ?? 'Cancel';
        this.variant = options.variant ?? 'default';
        this._onConfirm = options.onConfirm;
        this._onCancel = options.onCancel ?? null;
        this.visible = true;
    }

    confirm() {
        this._onConfirm?.();
        this.close();
    }

    cancel() {
        this._onCancel?.();
        this.close();
    }

    close() {
        this.visible = false;
        this._onConfirm = null;
        this._onCancel = null;
    }
}

export const confirmDialogState = new ConfirmDialogState();
