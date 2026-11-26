/** What the pre-join button does: join directly, or ask the host first. */
export type JoinAction = 'join' | 'ask';

/** Hosts and guests of open rooms join directly; guests of approval rooms ask first. */
export function joinAction(isHost: boolean, open: boolean): JoinAction {
	return isHost || open ? 'join' : 'ask';
}
