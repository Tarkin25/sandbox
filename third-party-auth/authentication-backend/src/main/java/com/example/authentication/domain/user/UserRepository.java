package com.example.authentication.domain.user;

import com.example.authentication.core.DomainEntityRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Mono;

@Repository
public interface UserRepository extends DomainEntityRepository<User> {

    Mono<User> findByGoogleIdAndDeletedFalse(String googleId);

}
