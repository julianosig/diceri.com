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
	await person.page.getByLabel('Hear everyone in').selectOption(person.language);
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
		const probe = await ana.page.request.post(`${url}/api/v1/rooms`, {
			data: { title: 'Probe' }
		});
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

test('detects the spoken language regardless of the output language', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	// Hears in English but speaks Portuguese (the pt-BR fixture).
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

test('recording a voice takes seconds and the call keeps translating', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const bob = await launch('Bob', 'en', url);
	try {
		await bob.page.goto('/');
		await bob.page.getByRole('button', { name: 'New meeting' }).click();
		await fillProfile(bob);
		await bob.page.getByRole('button', { name: /Record my voice/ }).click();
		await expect(bob.page.getByText('Recording · read aloud')).toBeVisible();
		await expect(bob.page.getByText(/I agree to Diceri using my voice/)).toBeVisible();
		await expect(bob.page.getByText('Your voice is ready')).toBeVisible({ timeout: 30_000 });

		await bob.page.getByRole('button', { name: 'Join now' }).click();
		await expect(bob.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await bob.page.getByRole('button', { name: 'Audio settings' }).click();
		await bob.page.getByRole('radio', { name: 'Translated' }).click();
		await bob.page.getByLabel('Translate me into').selectOption('pt-BR');
		// The fixture is a synthetic voice, which the provider refuses to clone-speak,
		// so this also proves the stock-voice fallback keeps translations flowing.
		await expectCaption(bob, 'Bob', /parque/i);
	} finally {
		await bob.browser.close();
	}
});

test('previewing in the language you speak explains there is nothing to translate', async ({
	baseURL
}) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	// Hears English and speaks English; the preview starts in the output language.
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
		await expect(bob.page.getByLabel('Translate me into')).toHaveValue('en');
		await expect(bob.page.getByRole('status')).toContainText("You're speaking English", {
			timeout: 30_000
		});
	} finally {
		await bob.browser.close();
	}
});

test('when the host ends the call everyone gets the transcript', async ({ baseURL }) => {
	test.setTimeout(180_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const ana = await launch('Ana', 'pt-BR', url);
	const bob = await launch('Bob', 'en', url);
	try {
		await ana.page.goto('/');
		await ana.page.getByRole('button', { name: 'New meeting' }).click();
		await fillProfile(ana);
		await ana.page.getByRole('button', { name: 'Join now' }).click();
		await expect(ana.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await admit(ana.page, bob);
		await expectCaption(bob, 'Ana', /contract/i);

		await ana.page.getByRole('button', { name: 'Leave call' }).click();
		await ana.page.getByRole('menuitem', { name: 'End call for everyone' }).click();

		await expect(bob.page.getByRole('heading', { name: 'The host ended the call' })).toBeVisible({
			timeout: 20_000
		});
		const bobTranscript = bob.page.getByRole('region', { name: 'Transcript' });
		await expect(bobTranscript).toContainText(/contrato/i);
		await expect(bobTranscript).toContainText(/contract/i);
		await expect(bobTranscript.getByRole('button', { name: 'Download' })).toBeVisible();

		await expect(
			ana.page.getByRole('heading', { name: 'You ended the call for everyone' })
		).toBeVisible();
		await expect(ana.page.getByRole('region', { name: 'Transcript' })).toContainText(
			/weather|tempo/i
		);
	} finally {
		await Promise.all([ana.browser.close(), bob.browser.close()]);
	}
});
