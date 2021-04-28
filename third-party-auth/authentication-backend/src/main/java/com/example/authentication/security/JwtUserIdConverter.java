package com.example.authentication.security;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.http.HttpHeaders;
import org.springframework.security.core.Authentication;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;

@Component
@RequiredArgsConstructor
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
public class JwtUserIdConverter implements UserIdConverter {

    private static final String TOKEN_PREFIX = "Bearer ";

    JwtConverter jwtConverter;

    /**
     * Converts a {@link ServerWebExchange} to an {@link Authentication}
     *
     * @param exchange The {@link ServerWebExchange}
     * @return A {@link Mono} representing an {@link Authentication}
     */
    @Override
    public Mono<Long> convert(ServerWebExchange exchange) {
        return Mono.just(exchange.getRequest().getHeaders())
                .filter(headers -> headers.containsKey(HttpHeaders.AUTHORIZATION))
                .map(headers -> headers.get(HttpHeaders.AUTHORIZATION))
                .filter(values -> !values.isEmpty())
                .map(values -> values.get(0))
                .filter(value -> value.startsWith(TOKEN_PREFIX))
                .map(value -> value.substring(TOKEN_PREFIX.length()))
                .map(this::parseId);
    }

    private Long parseId(String token) {
        String subject = jwtConverter.parseSubject(token);

        return Long.parseLong(subject);
    }
}
