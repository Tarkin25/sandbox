package com.example.todo.cors;

import org.springframework.http.HttpMethod;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import org.springframework.web.server.WebFilter;
import org.springframework.web.server.WebFilterChain;
import reactor.core.publisher.Mono;

import java.util.List;

@Component
public class CorsFilter implements WebFilter {

    @Override
    public Mono<Void> filter(ServerWebExchange exchange, WebFilterChain chain) {
        var requestHeaders = exchange.getRequest().getHeaders();

        var responseHeaders = exchange.getResponse().getHeaders();

        responseHeaders.setAccessControlAllowOrigin(requestHeaders.getOrigin());
        responseHeaders.setAccessControlAllowMethods(List.of(HttpMethod.GET, HttpMethod.POST, HttpMethod.PUT, HttpMethod.DELETE, HttpMethod.OPTIONS));
        responseHeaders.setAccessControlAllowHeaders(List.of("*"));

        return chain.filter(exchange);
    }
}
