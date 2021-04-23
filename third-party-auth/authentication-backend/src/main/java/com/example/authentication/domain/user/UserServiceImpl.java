package com.example.authentication.domain.user;

import com.google.api.client.googleapis.auth.oauth2.GoogleIdToken;
import com.google.api.client.googleapis.auth.oauth2.GoogleIdTokenVerifier;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;

import java.io.IOException;
import java.security.GeneralSecurityException;

@Service
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class UserServiceImpl implements UserService {

    UserRepository repository;
    GoogleIdTokenVerifier tokenVerifier;

    @Override
    public Mono<User> googleSignIn(String tokenString) {
        return verifyToken(tokenString)
                .flatMap(this::findOrCreate);

    }

    private Mono<GoogleIdToken> verifyToken(String tokenString) {
        return Mono.create(sink -> {
            try {
                GoogleIdToken token = tokenVerifier.verify(tokenString);

                sink.success(token);
            } catch (GeneralSecurityException | IOException e) {
                sink.error(e);
            }
        });
    }

    private Mono<User> findOrCreate(GoogleIdToken token) {
        GoogleIdToken.Payload payload = token.getPayload();
        String googleId = payload.getSubject();

        return repository.findByGoogleIdAndDeletedFalse(googleId)
                .switchIfEmpty(create(payload));
    }

    private Mono<User> create(GoogleIdToken.Payload payload) {
        User user = new User()
                .withGoogleId(payload.getSubject())
                .withEmail(payload.getEmail())
                .withName((String) payload.get("name"))
                .withDeleted(false);

        return repository.save(user);
    }
}
