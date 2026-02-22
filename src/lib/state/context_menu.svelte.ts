export interface ContextMenuItem {
    label: string;
    icon?: any;
    onclick: () => void;
    variant?: 'default' | 'danger';
}

class ContextMenuState {
    visible = $state(false);
    x = $state(0);
    y = $state(0);
    items = $state<ContextMenuItem[]>([]);

    open(x: number, y: number, items: ContextMenuItem[]) {
        this.x = x;
        this.y = y;
        this.items = items;
        this.visible = true;

        setTimeout(() => {
            const menu = document.getElementById('global-context-menu');
            // prevents from going off screen
            if (menu) {
                const rect = menu.getBoundingClientRect();
                if (this.x + rect.width > window.innerWidth) {
                    this.x = window.innerWidth - rect.width - 8;
                }
                if (this.y + rect.height > window.innerHeight) {
                    this.y = window.innerHeight - rect.height - 8;
                }
            }
        }, 0);
    }

    close() {
        this.visible = false;
    }
}

export const contextMenuState = new ContextMenuState();
