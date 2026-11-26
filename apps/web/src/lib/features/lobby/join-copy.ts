/** Words on the pre-join screen, by who is joining and how the room lets people in. */
export interface JoinCopy {
	heading: string;
	hint: string;
	action: string;
}

export function joinCopy(isHost: boolean, open: boolean): JoinCopy {
	if (isHost) {
		return {
			heading: 'Ready to join?',
			hint: 'You are the host of this meeting.',
			action: 'Join now'
		};
	}
	if (open) {
		return { heading: 'Ready to join?', hint: 'Anyone can join this meeting.', action: 'Join now' };
	}
	return {
		heading: 'Ask to join',
		hint: 'The host will see your name and let you in.',
		action: 'Ask to join'
	};
}
