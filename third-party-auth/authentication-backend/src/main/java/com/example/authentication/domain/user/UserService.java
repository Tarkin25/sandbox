package com.example.authentication.domain.user;

import reactor.core.publisher.Mono;

public interface UserService {

    /**
     * Sign in using a Google ID token.
     * <br/>
     * If the token is valid, update user data in the database to match Google's user data if needed (e.g. user's name changed, etc.).
     * <br/>
     * If the token is invalid or if the user does not exist in our database, return an error.
     * @param token Google ID token provided by Google's JavaScript API login
     * @return a Mono containing the signed in user, or an error in case the sign-in is unsuccessful
     */
    Mono<User> googleSignIn(String token);

    /**
     * Sign up a new user using a Google ID token.
     * <br/>
     * If the token is valid, create a new user with Google's user data in the database.
     * <br/>
     * If the token is invalid or the user already exists in the database, return an error.
     * @param token Google ID token provided by Google's JavaScript API login
     * @return a Mono containing the created user, or an error in case the sign-up is unsuccessful
     */
    Mono<User> googleSignUp(String token);

    /**
     * Sign in using a Facebook access token.
     * <br/>
     * If the token is valid, update user data in the database to match Facebook's user data if needed (e.g. user's name changed, etc.).
     * <br/>
     * If the token is invalid or if the user does not exist in our database, return an error.
     * @param token Facebook access token provided by Facebook's JavaScript SDK login
     * @return a Mono containing the signed in user, or an error in case the sign-in is unsuccessful
     */
    Mono<User> facebookSignIn(String token);

    /**
     * Sign up a new user using a Facebook access token.
     * <br/>
     * If the token is valid, create a new user with Facebook's user data in the database.
     * <br/>
     * If the token is invalid or the user already exists in the database, return an error.
     * @param token Facebook access token provided by Facebook's JavaScript SDK login
     * @return a Mono containing the created user, or an error in case the sign-up is unsuccessful
     */
    Mono<User> facebookSignUp(String token);

}
