import type { Messages } from './en.ts';

export const ru: Messages = {
	meta: { title: 'Diceri — привычный видеозвонок на любом языке' },
	header: {
		signOut: 'Выйти',
		language: 'Язык',
		toDark: 'Включить тёмную тему',
		toLight: 'Включить светлую тему'
	},
	hero: {
		headline: 'Тот самый видеозвонок, который вы знаете.',
		promise: 'Теперь на любом языке.',
		illustration:
			'Иллюстрация: один человек говорит, Diceri переводит посередине, а другой слышит это на своём языке.'
	},
	onboarding: {
		label: 'Войдите или займите имя своей комнаты',
		usernamePlaceholder: 'yourname',
		checking: (handle: string) => `Проверяем @${handle}…`,
		available: (handle: string) => `@${handle} свободно — займите его`,
		welcomeBack: (handle: string) => `С возвращением, @${handle}`,
		invalid: 'От 3 до 20 латинских букв (a–z), цифр или _, начиная с буквы.',
		checkFailed: 'Не удалось проверить имя. Попробуйте ещё раз.',
		password: 'Пароль',
		repeatPassword: 'Повторите пароль',
		newPasswordPlaceholder: (min: number) => `Не меньше ${String(min)} символов`,
		passwordPlaceholder: 'Ваш пароль',
		repeatPlaceholder: 'Введите ещё раз',
		passwordsMatch: 'Пароли совпадают',
		tooShort: (min: number) => `Используйте не меньше ${String(min)} символов.`,
		tooLong: (max: number) => `Используйте не больше ${String(max)} символов.`,
		mismatch: 'Пароли не совпадают.',
		create: 'Создать мою комнату',
		creating: 'Создаём…',
		signIn: 'Войти',
		signingIn: 'Входим…',
		taken: 'Это имя только что заняли.',
		wrongPassword: 'Неверный пароль.',
		failed: 'Что-то пошло не так. Попробуйте ещё раз.'
	},
	mascots: {
		legend: 'Талисман комнаты',
		names: {
			wizard: 'Волшебник',
			robot: 'Робот',
			astronaut: 'Космонавт',
			alien: 'Пришелец',
			ninja: 'Ниндзя',
			dragon: 'Дракон',
			knight: 'Рыцарь',
			ghost: 'Призрак',
			cybercat: 'Кибер-кот',
			slime: 'Слизень',
			pirate: 'Космический пират',
			gamer: 'Геймер'
		}
	},
	openRoom: {
		livesAt: (username: string) => `Ваша комната: diceri.com/@${username}`,
		listed: 'Показывать на живой доске',
		listedHint: 'Все на главной странице видят вашу комнату и могут её открыть.',
		whoGetsIn: 'Кто может войти',
		approval: 'Я одобряю каждого',
		open: 'Входить может любой',
		openHint: 'Пока комната открыта, любой, кто знает ваш @, входит сразу.',
		submit: 'Открыть мою комнату',
		submitting: 'Открываем…',
		failed: 'Не удалось открыть комнату. Попробуйте ещё раз.'
	},
	footer: { freeSoftware: 'Свободное ПО', sourceCode: 'Исходный код' }
};
