/** `GET /api/v1/accounts/me`. */
export interface Account {
	username: string;
}

/** `POST /api/v1/accounts`: the key comes back once; it signs in on other devices. */
export interface RegisteredAccount extends Account {
	key: string;
}
