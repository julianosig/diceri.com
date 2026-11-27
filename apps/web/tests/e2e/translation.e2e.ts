import { openMyRoom, signUp } from './support/accounts.ts';
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
	await signUp(guest.page);
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
		const probe = await ana.page.request.get(`${url}/api/v1/presence`);
		test.skip(!probe.ok(), 'API not reachable');

		await signUp(ana.page);
		await openMyRoom(ana.page);
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

test('detects the spoken language regardless of the output language', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	// Bob hears English but speaks Portuguese (the pt-BR fixture); Ana hears Spanish.
	const bob = await launch('Bob', 'en', url, 'pt-BR');
	const ana = await launch('Ana', 'es', url);
	try {
		await signUp(bob.page);
		await openMyRoom(bob.page);
		await fillProfile(bob);
		await bob.page.getByRole('button', { name: 'Join now' }).click();
		await expect(bob.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await admit(bob.page, ana);

		await expectCaption(ana, 'Bob', /mañana|tarde/i);
	} finally {
		await Promise.all([bob.browser.close(), ana.browser.close()]);
	}
});

test('recording a voice takes seconds and the call keeps translating', async ({ baseURL }) => {
	test.setTimeout(120_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const bob = await launch('Bob', 'en', url);
	const ana = await launch('Ana', 'pt-BR', url);
	try {
		await signUp(bob.page);
		await openMyRoom(bob.page);
		await fillProfile(bob);
		await bob.page.getByRole('button', { name: /Record my voice/ }).click();
		await expect(bob.page.getByText('Recording · read aloud')).toBeVisible();
		await expect(bob.page.getByText(/I agree to Diceri using my voice/)).toBeVisible();
		await expect(bob.page.getByText('Your voice is ready')).toBeVisible({ timeout: 30_000 });

		await bob.page.getByRole('button', { name: 'Join now' }).click();
		await expect(bob.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await admit(bob.page, ana);
		// The fixture is a synthetic voice, which the provider refuses to clone-speak,
		// so this also proves the stock-voice fallback keeps translations flowing.
		await expectCaption(ana, 'Bob', /parque/i);
	} finally {
		await Promise.all([bob.browser.close(), ana.browser.close()]);
	}
});

test('when the host ends the call everyone gets the transcript', async ({ baseURL }) => {
	test.setTimeout(180_000);
	const url = baseURL ?? 'http://127.0.0.1:4180';
	const ana = await launch('Ana', 'pt-BR', url);
	const bob = await launch('Bob', 'en', url);
	try {
		await signUp(ana.page);
		await openMyRoom(ana.page);
		await fillProfile(ana);
		await ana.page.getByRole('button', { name: 'Join now' }).click();
		await expect(ana.page.getByRole('button', { name: 'Leave call' })).toBeVisible({
			timeout: 20_000
		});
		await admit(ana.page, bob);
		// Both phrases must be in before ending, or the transcripts can miss one.
		await expectCaption(bob, 'Ana', /contract/i);
		await expectCaption(ana, 'Bob', /tempo/i);

		await ana.page.getByRole('button', { name: 'Leave call' }).click();
		await ana.page.getByRole('menuitem', { name: 'End call for everyone' }).click();

		// LiveKit Cloud sometimes takes 20 s+ to tell remaining participants the room is gone.
		await expect(bob.page.getByRole('heading', { name: 'The host ended the call' })).toBeVisible({
			timeout: 40_000
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
