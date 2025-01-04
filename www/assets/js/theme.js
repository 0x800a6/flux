class ThemeManager {
    constructor() {
        this.theme = localStorage.getItem('theme') || 'system';
        this.systemDark = window.matchMedia('(prefers-color-scheme: dark)');
        this.init();
    }

    init() {
        this.applyTheme();
        this.setupListeners();
        this.setupButtons();
    }

    applyTheme() {
        const isDark = this.theme === 'dark' || 
            (this.theme === 'system' && this.systemDark.matches);
        
        document.documentElement.classList.toggle('dark', isDark);
        document.documentElement.setAttribute('data-theme', this.theme);
    }

    setupListeners() {
        this.systemDark.addListener(() => {
            if (this.theme === 'system') {
                this.applyTheme();
            }
        });

        document.querySelectorAll('.theme-button').forEach(button => {
            button.addEventListener('click', () => {
                this.theme = button.dataset.theme;
                localStorage.setItem('theme', this.theme);
                this.applyTheme();
                this.updateActiveButton();
            });
        });
    }

    setupButtons() {
        this.updateActiveButton();
    }

    updateActiveButton() {
        document.querySelectorAll('.theme-button').forEach(button => {
            button.classList.toggle('active', button.dataset.theme === this.theme);
        });
    }
}

new ThemeManager(); 