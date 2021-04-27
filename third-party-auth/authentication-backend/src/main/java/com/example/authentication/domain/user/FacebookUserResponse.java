package com.example.authentication.domain.user;

import lombok.AccessLevel;
import lombok.Data;
import lombok.experimental.FieldDefaults;

@Data
@FieldDefaults(level = AccessLevel.PRIVATE)
public class FacebookUserResponse implements UserData {

    String id;

    String name;

    String email;

}
