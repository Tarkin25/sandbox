package com.example.authentication.domain.user;

import org.springframework.security.core.GrantedAuthority;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

public interface UserService {

    Mono<User> googleSignIn(String token);

    Flux<GrantedAuthority> getUserAuthorities(Long userId);

}
