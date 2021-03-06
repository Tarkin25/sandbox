package com.example.authentication.config;

import com.example.authentication.security.JwtOncePerRequestAuthenticationFilter;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.config.web.server.SecurityWebFiltersOrder;
import org.springframework.security.config.web.server.ServerHttpSecurity;
import org.springframework.security.web.server.SecurityWebFilterChain;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.reactive.CorsConfigurationSource;
import org.springframework.web.cors.reactive.CorsWebFilter;
import org.springframework.web.cors.reactive.UrlBasedCorsConfigurationSource;

import java.util.List;

@Configuration
public class SecurityConfig {

    @Bean
    public SecurityWebFilterChain securityWebFilterChain(
            ServerHttpSecurity security,
            CorsWebFilter corsWebFilter,
            JwtOncePerRequestAuthenticationFilter jwtOncePerRequestAuthenticationFilter
    ) {
        return security
                .csrf().disable()
                .authorizeExchange()
                .pathMatchers(HttpMethod.GET, "/v3/api-docs/**", "/swagger-resources/**", "/swagger-ui.html", "/swagger-ui/**", "/webjars/**", "/swagger.yaml").permitAll()
                .pathMatchers(HttpMethod.POST, "/authentication/**").permitAll()
                .anyExchange().authenticated()
                .and()
                .httpBasic().disable()
                .formLogin().disable()
                .addFilterAt(corsWebFilter, SecurityWebFiltersOrder.CORS)
                .addFilterAt(jwtOncePerRequestAuthenticationFilter, SecurityWebFiltersOrder.SECURITY_CONTEXT_SERVER_WEB_EXCHANGE)
                .build();
    }

    @Bean
    public CorsConfiguration corsConfiguration() {
        var configuration = new CorsConfiguration();
        configuration.applyPermitDefaultValues();
        configuration.setAllowedOriginPatterns(List.of("*"));
        configuration.setAllowedMethods(List.of("GET", "POST", "PUT", "DELETE"));
        configuration.setAllowCredentials(true);
        return configuration;
    }

    @Bean
    public CorsConfigurationSource configurationSource(CorsConfiguration corsConfiguration) {
        var source = new UrlBasedCorsConfigurationSource();
        source.registerCorsConfiguration("/**", corsConfiguration);
        return source;
    }

    @Bean
    public CorsWebFilter corsWebFilter(CorsConfigurationSource configurationSource) {
        return new CorsWebFilter(configurationSource);
    }

}
