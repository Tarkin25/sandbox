package com.example.authentication.security;

import com.example.authentication.domain.user.User;
import com.example.authentication.domain.user.UserDTO;
import com.example.authentication.domain.user.UserMapper;
import com.example.authentication.domain.user.UserService;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import lombok.experimental.FieldDefaults;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Mono;

@RestController
@RequestMapping("/authentication")
@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
@RequiredArgsConstructor
public class AuthenticationController {

    JwtConverter jwtConverter;
    UserService userService;
    UserMapper userMapper;

    @PostMapping("/google/sign-up")
    @ResponseStatus(HttpStatus.CREATED)
    public Mono<AuthenticationResponse> googleSignUp(@RequestParam String googleIdToken) {
        return userService.googleSignUp(googleIdToken)
                .map(this::authenticationResponse);
    }

    @PostMapping("/google/sign-in")
    public Mono<AuthenticationResponse> googleSignIn(@RequestParam String googleIdToken) {
        return userService.googleSignIn(googleIdToken)
                .map(this::authenticationResponse);
    }

    @PostMapping("/facebook/sign-up")
    public Mono<AuthenticationResponse> facebookSignUp(@RequestParam String accessToken) {
        return userService.facebookSignUp(accessToken)
                .map(this::authenticationResponse);
    }

    @PostMapping("/facebook/sign-in")
    public Mono<AuthenticationResponse> facebookSignIn(@RequestParam String accessToken) {
        return userService.facebookSignIn(accessToken)
                .map(this::authenticationResponse);
    }

    private AuthenticationResponse authenticationResponse(User user) {
        String authToken = jwtConverter.createToken(user.getId().toString());
        UserDTO userDTO = userMapper.toDTO(user);

        return new AuthenticationResponse(authToken, userDTO);
    }

}
