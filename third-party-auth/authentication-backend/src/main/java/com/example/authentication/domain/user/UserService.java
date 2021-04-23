package com.example.authentication.domain.user;

import reactor.core.publisher.Mono;

public interface UserService {

    Mono<User> googleSignIn(String token);

}
