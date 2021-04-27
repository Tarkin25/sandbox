package com.example.authentication.domain.user;

import com.example.authentication.exception.InvalidGoogleTokenException;
import com.example.authentication.exception.SignInException;
import com.example.authentication.exception.UserExistsException;
import com.google.api.client.googleapis.auth.oauth2.GoogleIdToken;
import com.google.api.client.googleapis.auth.oauth2.GoogleIdTokenVerifier;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.core.publisher.SynchronousSink;

import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.function.BiConsumer;
import java.util.stream.Stream;

@Service
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
@Slf4j
public class UserServiceImpl implements UserService {

    UserRepository repository;
    GoogleIdTokenVerifier tokenVerifier;
    WebClient graphClient = WebClient.create("https://graph.facebook.com");

    /**
     * Sign in using a Google ID token.
     * <br/>
     * If the token is valid, update user data in the database to match Google's user data if needed (e.g. user's name changed, etc.).
     * <br/>
     * If the token is invalid or if the user does not exist in our database, return an error.
     *
     * @param token Google ID token provided by Google's JavaScript API login
     * @return a Mono containing the signed in user, or an error in case the sign-in is unsuccessful
     */
    @Override
    public Mono<User> googleSignIn(String token) {
        return verifyGoogleToken(token)
                .flatMap(
                        googleIdToken -> findGoogleUser(googleIdToken)
                                .flatMap(
                                        user -> updateIfNeeded(user, new GoogleUserData(googleIdToken))
                                )
                )
                .doOnNext(this::signInSuccess)
                .switchIfEmpty(Mono.error(new SignInException()));
    }

    /**
     * Sign up a new user using a Google ID token.
     * <br/>
     * If the token is valid, create a new user with Google's user data in the database.
     * <br/>
     * If the token is invalid or the user already exists in the database, return an error.
     *
     * @param token Google ID token provided by Google's JavaScript API login
     * @return a Mono containing the created user, or an error in case the sign-up is unsuccessful
     */
    @Override
    public Mono<User> googleSignUp(String token) {
        return verifyGoogleToken(token)
                .map(GoogleUserData::new)
                .flatMap(this::assertNewGoogleUser)
                .flatMap(this::createFromGoogle)
                .doOnNext(this::signUpSuccess);
    }

    /**
     * Sign in using a Facebook access token.
     * <br/>
     * If the token is valid, update user data in the database to match Facebook's user data if needed (e.g. user's name changed, etc.).
     * <br/>
     * If the token is invalid or if the user does not exist in our database, return an error.
     *
     * @param token Facebook access token provided by Facebook's JavaScript SDK login
     * @return a Mono containing the signed in user, or an error in case the sign-in is unsuccessful
     */
    @Override
    public Mono<User> facebookSignIn(String token) {
        return getUserDataFromFacebook(token)
                .flatMap(
                        facebookUserResponse -> findFacebookUser(facebookUserResponse)
                                .flatMap(
                                        user -> updateIfNeeded(user, facebookUserResponse)
                                )
                )
                .doOnNext(this::signInSuccess)
                .switchIfEmpty(Mono.error(new SignInException()));
    }

    /**
     * Sign up a new user using a Facebook access token.
     * <br/>
     * If the token is valid, create a new user with Facebook's user data in the database.
     * <br/>
     * If the token is invalid or the user already exists in the database, return an error.
     *
     * @param token Facebook access token provided by Facebook's JavaScript SDK login
     * @return a Mono containing the created user, or an error in case the sign-up is unsuccessful
     */
    @Override
    public Mono<User> facebookSignUp(String token) {
        return getUserDataFromFacebook(token)
                .flatMap(this::assertNewFacebookUser)
                .flatMap(this::createFromFacebook)
                .doOnNext(this::signUpSuccess);
    }

    /**
     * Fetches user data from the Facebook graph API using a user access token.
     * @param tokenString Facebook user access token
     * @return user data provided by the Facebook graph API
     */
    private Mono<FacebookUserResponse> getUserDataFromFacebook(String tokenString) {
        return graphClient
                .get()
                .uri(
                        uriBuilder -> uriBuilder
                        .path("/me")
                        .queryParam("fields", "id,name,email")
                        .queryParam("access_token", tokenString)
                        .build()
                )
                .retrieve()
                .bodyToMono(FacebookUserResponse.class);
    }

    /**
     * Verifies a Google ID token and returns it if it is valid.
     * @param tokenString Google ID token to verify
     * @return valid Google ID token
     */
    private Mono<GoogleIdToken> verifyGoogleToken(String tokenString) {
        return Mono.create(sink -> {
            log.debug("Verifying google id token '{}'", tokenString);

            try {
                GoogleIdToken token = tokenVerifier.verify(tokenString);

                if(token != null) {
                    log.debug("Token is valid");
                    sink.success(token);
                } else {
                    log.debug("Token is invalid");
                    sink.error(new InvalidGoogleTokenException());
                }
            } catch (GeneralSecurityException | IOException e) {
                sink.error(e);
            }
        });
    }

    /**
     * Finds a User which has signed up using Google
     * @param token Google ID token used to find the User
     * @return Mono containing the User
     */
    private Mono<User> findGoogleUser(GoogleIdToken token) {
        return repository.findByGoogleId(token.getPayload().getSubject());
    }

    /**
     * Finds a User which has signed up using Facebook
     * @param response {@link FacebookUserResponse} used to find the User
     * @return Mono containing the User
     */
    private Mono<User> findFacebookUser(FacebookUserResponse response) {
        return repository.findByFacebookId(response.getId());
    }

    /**
     * Checks if a given User needs to be updated based on given {@link UserData} and possibly updates them.
     * @param user User to be checked and possibly updated.
     * @param userData UserData used to determine if an update is required
     * @return A possibly updated User
     */
    private Mono<User> updateIfNeeded(User user, UserData userData) {

        if (isUpdateNeeded(user, userData)) {
            log.debug("Updating user with payload values");
            return update(user, userData);
        } else {
            return Mono.just(user);
        }
    }

    /**
     * Determines if a given User needs to be updated based on given {@link UserData}.
     * <br/>
     * An update is required if the User's
     * <ul>
     *     <li>email</li>
     *     <li>name</li>
     * </ul>
     * don't match with the given UserData
     * @param user User to be checked
     * @param userData UserData to determine if an update is required
     * @return {@code true} if an updated is required, {@code false} otherwise
     */
    private boolean isUpdateNeeded(User user, UserData userData) {
        return Stream.of(
                user.getEmail().equals(userData.getEmail()),
                user.getName().equals(userData.getName())
        ).noneMatch(Boolean::booleanValue);
    }

    /**
     * Updates a given User according to the given {@link UserData}.
     * <br/>
     * The updated attributes are:
     * <ul>
     *     <li>email</li>
     *     <li>name</li>
     * </ul>
     * @param user User to be updated
     * @param userData UserData with the new values
     * @return The updated User
     */
    private Mono<User> update(User user, UserData userData) {
        return repository.save(
                user.withEmail(userData.getEmail())
                        .withName(userData.getName())
        );
    }
    private Mono<UserData> assertNewGoogleUser(UserData userData) {
        return repository.existsByGoogleId(userData.getId())
                .handle(handleUserExists(userData));
    }

    private Mono<UserData> assertNewFacebookUser(UserData userData) {
        return repository.existsByFacebookId(userData.getId())
                .handle(handleUserExists(userData));
    }

    private BiConsumer<Boolean, SynchronousSink<UserData>> handleUserExists(UserData userData) {
        return (exists, sink) -> {
            if(Boolean.FALSE.equals(exists)) {
                sink.next(userData);
            } else {
                sink.error(new UserExistsException());
            }
        };
    }

    /**
     * Creates a new User from given {@link UserData} and marks them as having signed up with Google.
     * @param userData UserData with the values for the new User
     * @return The created User
     */
    private Mono<User> createFromGoogle(UserData userData) {
        User user = new User()
                .withGoogleId(userData.getId())
                .withEmail(userData.getEmail())
                .withName(userData.getName());

        return repository.save(user);
    }

    /**
     * Creates a new User from given {@link UserData} and marks them as having signed up with Facebook.
     * @param userData UserData with the values for the new User
     * @return The created User
     */
    private Mono<User> createFromFacebook(UserData userData) {
        User user = new User()
                .withFacebookId(userData.getId())
                .withEmail(userData.getEmail())
                .withName(userData.getName());

        return repository.save(user);
    }

    private void signInSuccess(User user) {
        log.debug("Sign-in successful");
    }

    private void signUpSuccess(User user) {
        log.debug("Sign-up successful");
    }
}
