package com.example.authentication.security;

import com.example.authentication.domain.user.UserDTO;
import lombok.Value;

@Value
public class AuthenticationResponse {

    String authToken;

    UserDTO user;

}
