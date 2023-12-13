package auth

import "context"

// Authenticator authenticates users returning an identifier for the user.
// (Currently it handles only username+password single-step authentication.
// This interface will need to change significantly in order to support
// challenge-response protocols.)
type Authenticator interface {
	// AuthenticateUser authenticates a user matching username and
	// password and returns their ID.
	AuthenticateUser(ctx context.Context, username, password string) (string, error)
}
