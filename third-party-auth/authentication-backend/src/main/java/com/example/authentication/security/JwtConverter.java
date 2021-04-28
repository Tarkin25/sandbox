package com.example.authentication.security;

import com.example.authentication.config.JwtProperties;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.SignatureAlgorithm;
import io.jsonwebtoken.security.Keys;
import lombok.AccessLevel;
import lombok.experimental.FieldDefaults;
import org.springframework.stereotype.Component;

import java.nio.charset.StandardCharsets;
import java.security.Key;

@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@Component
public class JwtConverter {

    JwtProperties jwtProperties;
    Key signatureKey;

    public JwtConverter(JwtProperties jwtProperties) {
        this.jwtProperties = jwtProperties;
        this.signatureKey = Keys.hmacShaKeyFor(jwtProperties.getSecret().getBytes(StandardCharsets.UTF_8));
    }

    public String parseSubject(String token) {
        return Jwts.parserBuilder()
                .setSigningKey(signatureKey)
                .requireIssuer(jwtProperties.getIssuer())
                .build()
                .parseClaimsJws(token)
                .getBody()
                .getSubject();
    }

    public String createToken(String subject) {
        return Jwts.builder()
                .signWith(signatureKey, SignatureAlgorithm.HS512)
                .setIssuer(jwtProperties.getIssuer())
                .setSubject(subject)
                .compact();
    }

}
