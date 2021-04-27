package com.example.authentication.domain.user;

import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/auth")
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class AuthenticationController {

    UserService userService;

    @PostMapping("/sign-in/google")
    public Mono<User> googleSignIn(@RequestParam String googleIdToken) {
        return userService.googleSignIn(googleIdToken);
    }

    @PostMapping("/sign-up/google")
    public Mono<User> googleSignUp(@RequestParam String googleIdToken) {
        return userService.googleSignUp(googleIdToken);
    }

    @PostMapping("/sign-in/facebook")
    public Mono<User> facebookSignIn(@RequestParam String accessToken) {
        return userService.facebookSignIn(accessToken);
    }

    @PostMapping("/sign-up/facebook")
    public Mono<User> facebookSignUp(@RequestParam String accessToken) {
        return userService.facebookSignUp(accessToken);
    }

}
