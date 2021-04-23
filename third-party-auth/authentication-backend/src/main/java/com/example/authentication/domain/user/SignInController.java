package com.example.authentication.domain.user;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/sign-in")
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class SignInController {

    UserService userService;

    @PostMapping("/google")
    public Mono<User> googleSignIn(@RequestParam String googleIdToken) {
        return userService.googleSignIn(googleIdToken);
    }

}
