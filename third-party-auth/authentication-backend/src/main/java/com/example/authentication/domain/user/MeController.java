package com.example.authentication.domain.user;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/me")
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class MeController {

    UserMapper userMapper;

    @GetMapping
    public Mono<UserDTO> getMe(@AuthenticationPrincipal(expression = "user") User user) {
        return Mono.just(user)
                .map(userMapper::toDTO);
    }

}
