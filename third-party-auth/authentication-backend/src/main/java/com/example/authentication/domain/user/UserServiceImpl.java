package com.example.authentication.domain.user;

import com.google.api.client.googleapis.auth.oauth2.GoogleIdToken;
import com.google.api.client.googleapis.auth.oauth2.GoogleIdTokenVerifier;
import io.r2dbc.spi.Row;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import lombok.extern.slf4j.Slf4j;
import org.springframework.r2dbc.core.DatabaseClient;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.stream.Stream;

@Service
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
@Slf4j
public class UserServiceImpl implements UserService {

    UserRepository repository;
    GoogleIdTokenVerifier tokenVerifier;
    DatabaseClient databaseClient;

    @Override
    public Flux<GrantedAuthority> getUserAuthorities(Long userId) {
        return databaseClient.sql(
                "select concat('ROLE_', r.name) as name from role r\n" +
                "join user_role ur on r.id = ur.role_id\n" +
                "where ur.user_id = :userId\n" +
                "union\n" +
                "select a.name as name from authority a\n" +
                "join role_authority ra on a.id = ra.authority_id\n" +
                "join user_role ur on ra.role_id = ur.role_id\n" +
                "where ur.user_id = :userId"
        )
                .bind("userId", userId)
                .map(this::rowToGrantedAuthority)
                .all();
    }

    private GrantedAuthority rowToGrantedAuthority(Row row) {
        return new SimpleGrantedAuthority(row.get("name", String.class));
    }

    @Override
    public Mono<User> googleSignIn(String tokenString) {
        return verifyToken(tokenString)
                .flatMap(this::updateOrCreate)
                .doOnNext(user -> log.debug("Sign-in successful"));
    }

    private Mono<GoogleIdToken> verifyToken(String tokenString) {
        return Mono.create(sink -> {
            log.debug("Verifying google id token '{}'", tokenString);

            try {
                GoogleIdToken token = tokenVerifier.verify(tokenString);

                log.debug("Token is valid");
                sink.success(token);
            } catch (GeneralSecurityException | IOException e) {
                sink.error(e);
            }
        });
    }

    private Mono<User> updateOrCreate(GoogleIdToken token) {
        GoogleIdToken.Payload payload = token.getPayload();
        String googleId = payload.getSubject();

        return repository.findByGoogleIdAndDeletedFalse(googleId)
                .flatMap(user -> updateIfNeeded(user, payload))
                .switchIfEmpty(create(payload));
    }

    private Mono<User> updateIfNeeded(User user, GoogleIdToken.Payload payload) {

        if (isUpdateNeeded(user, payload)) {
            log.debug("Updating user with payload values");
            return update(user, payload);
        } else {
            return Mono.just(user);
        }
    }

    private boolean isUpdateNeeded(User user, GoogleIdToken.Payload payload) {
        return Stream.of(
                user.getEmail().equals(payload.getEmail()),
                user.getName().equals(payload.get("name"))
        ).noneMatch(Boolean::booleanValue);
    }

    private Mono<User> update(User user, GoogleIdToken.Payload payload) {
        return repository.save(
                user.withEmail(payload.getEmail())
                        .withName((String) payload.get("name"))
        );
    }

    private Mono<User> create(GoogleIdToken.Payload payload) {
        User user = new User()
                .withGoogleId(payload.getSubject())
                .withEmail(payload.getEmail())
                .withName((String) payload.get("name"))
                .withDeleted(false);

        return repository.save(user)
                .doOnNext(createdUser -> log.debug("Created new user"));
    }
}
