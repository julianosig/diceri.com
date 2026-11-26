/** Where the visitor's choice is kept (also read by the inline script in app.html). */
export const THEME_STORAGE_KEY = 'diceri-theme';

export type Theme = 'light' | 'dark';

/**
 * The colour theme: the `dark` class on <html> switches every design token in
 * layout.css. app.html applies it before first paint; this keeps it in sync after.
 * A global client preference, so module-level state is the right place.
 */
class ThemeState {
	current = $state<Theme>(
		typeof document !== 'undefined' && document.documentElement.classList.contains('dark')
			? 'dark'
			: 'light'
	);

	set(theme: Theme): void {
		this.current = theme;
		document.documentElement.classList.toggle('dark', theme === 'dark');
		try {
			localStorage.setItem(THEME_STORAGE_KEY, theme);
		} catch {
			// Storage can be blocked (private mode); the choice then lasts for this page.
		}
	}

	toggle(): void {
		this.set(this.current === 'dark' ? 'light' : 'dark');
	}
}

export const theme = new ThemeState();
