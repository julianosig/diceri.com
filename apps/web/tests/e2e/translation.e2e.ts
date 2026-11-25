import { chromium, expect, test, type Browser, type Page } from '@playwright/test';

/**
 * Three people, three languages, real providers end to end: each browser's fake
 * microphone loops a spoken phrase (tests/e2e/fixtures, `just fixtures`), and each
 * listener must see the others translated into their own language.
 */

const fixture = (language: string) =>
	new URL(`./fixtures/${language}.wav`, import.meta.url).pathname;

interface Person {
	name: string;
	language: string;
	browser: Browser;
	page: Page;
}

async function launch(
	name: string,
	language: string,
	baseURL: string,
	spokenFixture = language
): Promise<Person> {
	const browser = await chromium.launch({
		args: [
			'--use-fake-ui-for-media-stream',
			'--use-fake-device-for-media-stream',
			`--use-file-for-fake-audio-capture=${fixture(spokenFixture)}`,
			'--autoplay-policy=no-user-gesture-required'
		]
	});
	const context = await browser.newContext({ baseURL, permissions: ['microphone'] });
	return { name, language, browser, page: await context.newPage() };
}

async function fillProfile(person: Person): Promise<void> {
	await person.page.getByLabel('Your name').fill(person.name);
	await person.page.getByLabel('I speak and want to hear').selectOption(person.language);
}

async function admit(host: Page, guest: Person): Promise<void> {
	await guest.page.goto(host.url());
	await fillProfile(guest);
	await guest.page.getByRole('button', { name: 'Ask to join' }).click();
	await expect(host.getByText(`${guest.name} wants to join this call`)).toBeVisible({
		timeout: 15_000
	});
	await host.getByRole('button', { name: 'Admit' }).first().click();
	await expect(guest.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
		timeout: 20_000
	});
}

/** Waits until `listener` shows a caption from `speakerName` matching `text`. */
async function expectCaption(listener: Person, speakerName: string, text: RegExp): Promise<void> {
	const captions = listener.page.getByRole('region', { name: 'Captions' });
	await expect(
		captions.getByText(`${speakerName}:`).locator('..').filter({ hasText: text }).first()
	).toBeVisible({ timeout: 60_000 });
}

test('three people hear each other translated into their own language', async ({ baseURL }) => {
	test.setTimeout(180_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const ana = await launch('Ana', 'pt-BR', url);
	const bob = await launch('Bob', 'en', url);
	const carla = await launch('Carla', 'es', url);
	try {
		const probe = await ana.page.request.post(`${url}/api/v1/rooms`);
		test.skip(!probe.ok(), 'API not reachable');

		await ana.page.goto('/');
		await ana.page.getByRole('button', { name: 'New meeting' }).click();
		await fillProfile(ana);
		await ana.page.getByRole('button', { name: 'Join now' }).click();
		await expect(ana.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await admit(ana.page, bob);
		await admit(ana.page, carla);

		await expectCaption(bob, 'Ana', /contract/i);
		await expectCaption(carla, 'Ana', /contrato/i);
		await expectCaption(ana, 'Bob', /parque/i);
		await expectCaption(carla, 'Bob', /parque/i);
		await expectCaption(ana, 'Carla', /bicicleta/i);
		await expectCaption(bob, 'Carla', /bike|bicycle/i);
	} finally {
		await Promise.all([ana.browser.close(), bob.browser.close(), carla.browser.close()]);
	}
});

test('a person alone can hear their own voice translated', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const bob = await launch('Bob', 'en', url);
	try {
		await bob.page.goto('/');
		await bob.page.getByRole('button', { name: 'New meeting' }).click();
		await fillProfile(bob);
		await bob.page.getByRole('button', { name: 'Join now' }).click();
		await expect(bob.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await bob.page.getByRole('button', { name: 'Audio settings' }).click();
		await bob.page.getByRole('radio', { name: 'Translated' }).click();
		await bob.page.getByLabel('Translate me into').selectOption('pt-BR');

		await expectCaption(bob, 'Bob', /parque/i);
		await expect
			.poll(() => bob.page.locator('audio').count(), { timeout: 20_000 })
			.toBeGreaterThan(0);
	} finally {
		await bob.browser.close();
	}
});

test('translates from the language actually spoken, not the one declared', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	// Declares English but speaks Portuguese (the pt-BR fixture).
	const bob = await launch('Bob', 'en', url, 'pt-BR');
	try {
		await bob.page.goto('/');
		await bob.page.getByRole('button', { name: 'New meeting' }).click();
		await fillProfile(bob);
		await bob.page.getByRole('button', { name: 'Join now' }).click();
		await expect(bob.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await bob.page.getByRole('button', { name: 'Audio settings' }).click();
		await bob.page.getByRole('radio', { name: 'Translated' }).click();
		await bob.page.getByLabel('Translate me into').selectOption('es');

		await expectCaption(bob, 'Bob', /contrato/i);
		await expectCaption(bob, 'Bob', /mañana|tarde/i);
	} finally {
		await bob.browser.close();
	}
});
