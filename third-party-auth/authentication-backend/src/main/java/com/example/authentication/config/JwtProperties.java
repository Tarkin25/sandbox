package com.example.authentication.config;

import lombok.AccessLevel;
import lombok.Getter;
import lombok.Setter;
import lombok.experimental.FieldDefaults;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Component
@ConfigurationProperties("jwt")
@FieldDefaults(level = AccessLevel.PRIVATE)
@Getter
@Setter
public class JwtProperties {

    String secret;

    String issuer;

}
