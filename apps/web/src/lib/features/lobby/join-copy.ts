/**
 * The pre-join button: hosts and guests of open rooms join directly; guests of rooms
 * with approval ask the host first.
 */
export function joinAction(isHost: boolean, open: boolean): string {
	return isHost || open ? 'Join now' : 'Ask to join';
}
