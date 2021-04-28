package com.example.authentication.security;

import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;

public interface UserIdConverter {

    Mono<Long> convert(ServerWebExchange exchange);

}
