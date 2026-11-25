import { page } from 'vitest/browser';
import { describe, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { createRawSnippet } from 'svelte';
import Button from './Button.svelte';

const label = createRawSnippet(() => ({ render: () => '<span>New meeting</span>' }));

describe('Button', () => {
	it('renders its content as a type="button" by default', async () => {
		await render(Button, { children: label });

		const button = page.getByRole('button', { name: 'New meeting' });
		await expect.element(button).toBeVisible();
		await expect.element(button).toHaveAttribute('type', 'button');
	});
});
