package com.example.authentication.domain.user;

import com.example.authentication.core.DomainEntityRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Mono;

@Repository
public interface UserRepository extends DomainEntityRepository<User> {

    Mono<User> findByGoogleId(String googleId);

    Mono<User> findByFacebookId(String facebookId);

    Mono<Boolean> existsByGoogleId(String googleId);

    Mono<Boolean> existsByFacebookId(String facebookId);

}
