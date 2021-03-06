package com.example.authentication.security;

import com.example.authentication.domain.user.UserService;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.http.HttpMethod;
import org.springframework.security.core.context.ReactiveSecurityContextHolder;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.web.server.util.matcher.ServerWebExchangeMatcher;
import org.springframework.security.web.server.util.matcher.ServerWebExchangeMatchers;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import org.springframework.web.server.WebFilter;
import org.springframework.web.server.WebFilterChain;
import reactor.core.publisher.Mono;

import java.util.function.Function;

@Component
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class JwtOncePerRequestAuthenticationFilter implements WebFilter {

    private static final String FILTERED_ATTRIBUTE = JwtOncePerRequestAuthenticationFilter.class.getName() + ".FILTERED";

    UserIdConverter userIdConverter;
    UserService userService;
    ServerWebExchangeMatcher ignoreAuthenticationMatcher = ServerWebExchangeMatchers.pathMatchers(HttpMethod.POST, "/authentication/**");

    /**
     * Process the Web request and (optionally) delegate to the next
     * {@code WebFilter} through the given {@link WebFilterChain}.
     *
     * @param exchange the current server exchange
     * @param chain    provides a way to delegate to the next filter
     * @return {@code Mono<Void>} to indicate when request processing is complete
     */
    @Override
    public Mono<Void> filter(ServerWebExchange exchange, WebFilterChain chain) {
        Boolean filtered = exchange.getAttribute(FILTERED_ATTRIBUTE);

        if(!Boolean.TRUE.equals(filtered)) {
            return ignoreAuthenticationMatcher.matches(exchange).filter(matchResult -> !matchResult.isMatch())
                    .flatMap(matchResult -> userIdConverter.convert(exchange))
                    .flatMap(userService::findById)
                    .switchIfEmpty(chain.filter(exchange).then(Mono.empty()))
                    .map(UserDetailsImpl::new)
                    .flatMap(completeAuthentication(exchange, chain));
        } else {
            return chain.filter(exchange);
        }
    }

    private Function<UserDetails, Mono<Void>> completeAuthentication(ServerWebExchange exchange, WebFilterChain chain) {
        return userDetails -> {
            exchange.getAttributes().put(FILTERED_ATTRIBUTE, Boolean.TRUE);

            return chain.filter(exchange)
                    .contextWrite(ReactiveSecurityContextHolder.withAuthentication(IdAuthenticationToken.authenticated(userDetails)));
        };
    }
}
