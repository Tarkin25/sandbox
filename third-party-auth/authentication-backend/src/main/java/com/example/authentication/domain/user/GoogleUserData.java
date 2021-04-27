package com.example.authentication.domain.user;

import com.google.api.client.googleapis.auth.oauth2.GoogleIdToken;
import lombok.AccessLevel;
import lombok.experimental.FieldDefaults;

@FieldDefaults(level = AccessLevel.PRIVATE, makeFinal = true)
public class GoogleUserData implements UserData {

    GoogleIdToken.Payload payload;

    public GoogleUserData(GoogleIdToken token) {
        this.payload = token.getPayload();
    }

    @Override
    public String getId() {
        return payload.getSubject();
    }

    @Override
    public String getEmail() {
        return payload.getEmail();
    }

    @Override
    public String getName() {
        return (String) payload.get("name");
    }
}
